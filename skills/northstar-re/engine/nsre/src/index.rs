//! Local corpus index. One SQLite writer; bounded work batches; never execute inputs.
use super::{hash_file, unsafe_entry};
use rayon::prelude::*;
use rusqlite::{params, Connection, OpenFlags, OptionalExtension};
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    error::Error,
    ffi::OsString,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    time::{Duration, Instant, UNIX_EPOCH},
};
use walkdir::WalkDir;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const APP: i64 = 0x4e535245;
const SCHEMA: i64 = 1;
const DDL: &str = include_str!("schema.sql");

fn bad(s: &str) -> Box<dyn Error> {
    io::Error::other(s).into()
}
fn text(p: &Path) -> Result<String> {
    Ok(p.to_str().ok_or_else(|| bad("non UTF-8 path"))?.to_owned())
}
fn root_path(p: &Path) -> Result<PathBuf> {
    let m = fs::symlink_metadata(p)?;
    if unsafe_entry(&m) || !m.is_dir() {
        return Err(bad("root must be a normal directory"));
    }
    Ok(p.canonicalize()?)
}
fn normal_file(p: &Path) -> Result<()> {
    let m = fs::symlink_metadata(p)?;
    if unsafe_entry(&m) || !m.is_file() {
        return Err(bad("unsafe or non-regular database file"));
    }
    Ok(())
}
fn database_path(root: &Path, p: &Path) -> Result<PathBuf> {
    let parent = p
        .parent()
        .filter(|v| !v.as_os_str().is_empty())
        .unwrap_or(Path::new("."))
        .canonicalize()?;
    if parent.starts_with(root) {
        return Err(bad("database must be outside corpus"));
    }
    let dest = parent.join(
        p.file_name()
            .ok_or_else(|| bad("database filename required"))?,
    );
    for suffix in ["", "-wal", "-shm", "-journal"] {
        let mut name = dest.as_os_str().to_os_string();
        name.push(suffix);
        let item = PathBuf::from(name);
        match fs::symlink_metadata(&item) {
            Ok(_) => {
                normal_file(&item)?;
                if item.canonicalize()?.starts_with(root) {
                    return Err(bad("database path inside corpus"));
                }
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e.into()),
        }
    }
    Ok(dest)
}
fn check_db(c: &Connection) -> Result<()> {
    let a: i64 = c.query_row("PRAGMA application_id", [], |r| r.get(0))?;
    let v: i64 = c.query_row("PRAGMA user_version", [], |r| r.get(0))?;
    if (a, v) != (APP, SCHEMA) {
        return Err(bad("not an NSRE index or unsupported schema"));
    }
    Ok(())
}
fn open_db(p: &Path) -> Result<Connection> {
    let fresh = match fs::OpenOptions::new().write(true).create_new(true).open(p) {
        Ok(_) => true,
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            normal_file(p)?;
            false
        }
        Err(e) => return Err(e.into()),
    };
    let c = Connection::open_with_flags(p, OpenFlags::SQLITE_OPEN_READ_WRITE)?;
    c.busy_timeout(Duration::from_secs(15))?;
    if fresh {
        c.execute_batch(DDL)?;
    }
    check_db(&c)?;
    c.execute_batch("PRAGMA foreign_keys=ON; PRAGMA journal_mode=WAL; PRAGMA synchronous=FULL;")?;
    Ok(c)
}
fn read_db(p: &Path) -> Result<Connection> {
    normal_file(p)?;
    let c = Connection::open_with_flags(p, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    c.busy_timeout(Duration::from_secs(15))?;
    check_db(&c)?;
    Ok(c)
}
fn stamp(p: &Path) -> Result<(i64, String)> {
    let m = fs::symlink_metadata(p)?;
    if unsafe_entry(&m) || !m.is_file() {
        return Err(bad("not a regular file"));
    }
    let time = m.modified()?.duration_since(UNIX_EPOCH)?;
    Ok((
        i64::try_from(m.len())?,
        format!("{}:{:09}", time.as_secs(), time.subsec_nanos()),
    ))
}
#[derive(Debug)]
struct Work {
    path: PathBuf,
    name: String,
    cached: Option<(i64, String, String)>,
}
#[derive(Debug)]
struct Row {
    name: String,
    size: i64,
    stamp: String,
    hash: String,
    source: &'static str,
    error: Option<String>,
}
fn failure(name: String, e: impl ToString) -> Row {
    Row {
        name,
        size: 0,
        stamp: String::new(),
        hash: String::new(),
        source: "error",
        error: Some(e.to_string()),
    }
}
fn inspect(w: &Work) -> Row {
    let result = (|| -> Result<Row> {
        let (size, mtime) = stamp(&w.path)?;
        if let Some((n, t, h)) = &w.cached {
            if *n == size
                && *t == mtime
                && h.len() == 64
                && h.bytes().all(|b| b.is_ascii_hexdigit())
            {
                return Ok(Row {
                    name: w.name.clone(),
                    size,
                    stamp: mtime,
                    hash: h.clone(),
                    source: "metadata-cache",
                    error: None,
                });
            }
        }
        let (actual, hash) = hash_file(&w.path)?;
        if actual != size as u64 || stamp(&w.path)? != (size, mtime.clone()) {
            return Err(bad("file changed during hashing"));
        }
        Ok(Row {
            name: w.name.clone(),
            size,
            stamp: mtime,
            hash,
            source: "fresh",
            error: None,
        })
    })();
    result.unwrap_or_else(|e| failure(w.name.clone(), e))
}
#[derive(Default)]
struct Counts {
    fresh: u64,
    reused: u64,
    bytes: u64,
    errors: u64,
    skipped: u64,
}
fn batch(
    c: &mut Connection,
    id: i64,
    root: &str,
    prior: Option<i64>,
    pool: &rayon::ThreadPool,
    items: &mut Vec<Work>,
    counts: &mut Counts,
) -> Result<()> {
    if let Some(prior) = prior {
        let mut q=c.prepare_cached("SELECT f.size,f.mtime,f.sha256 FROM files f JOIN runs r ON r.id=f.run_id WHERE r.root=?1 AND r.id<=?2 AND f.path=?3 AND f.source='fresh' ORDER BY r.id DESC LIMIT 1")?;
        for w in items.iter_mut() {
            w.cached = q
                .query_row(params![root, prior, w.name], |r| {
                    Ok((r.get(0)?, r.get(1)?, r.get(2)?))
                })
                .optional()?;
        }
    }
    let rows: Vec<_> = pool.install(|| items.par_iter().map(inspect).collect());
    let tx = c.transaction()?;
    {
        let mut insert=tx.prepare_cached("INSERT INTO files(run_id,path,size,mtime,sha256,source,error) VALUES (?1,?2,?3,?4,?5,?6,?7)")?;
        for row in rows {
            match row.source {
                "fresh" => counts.fresh += 1,
                "metadata-cache" => counts.reused += 1,
                _ => counts.errors += 1,
            };
            counts.bytes += row.size as u64;
            insert.execute(params![
                id, row.name, row.size, row.stamp, row.hash, row.source, row.error
            ])?;
        }
    }
    tx.commit()?;
    items.clear();
    Ok(())
}
fn issue(c: &Connection, id: i64, p: &str, kind: &str, detail: &str) -> Result<()> {
    c.execute(
        "INSERT INTO issues(run_id,path,kind,detail) VALUES (?1,?2,?3,?4)",
        params![id, p, kind, detail],
    )?;
    Ok(())
}
fn index(root: &Path, db: &Path, jobs: usize, reuse: bool) -> Result<bool> {
    if !(1..=64).contains(&jobs) {
        return Err(bad("jobs must be 1..=64"));
    }
    let root = root_path(root)?;
    let root_name = text(&root)?;
    let db = database_path(&root, db)?;
    let mut c = open_db(&db)?;
    let prior: Option<i64> = if reuse {
        c.query_row(
            "SELECT max(id) FROM runs WHERE root=?1",
            [&root_name],
            |r| r.get(0),
        )?
    } else {
        None
    };
    c.execute(
        "INSERT INTO runs(root,mode,state,jobs) VALUES (?1,?2,'running',?3)",
        params![
            root_name,
            if reuse { "reuse-metadata" } else { "strict" },
            jobs as i64
        ],
    )?;
    let id = c.last_insert_rowid();
    eprintln!("run_id={id}");
    let pool = rayon::ThreadPoolBuilder::new().num_threads(jobs).build()?;
    let start = Instant::now();
    let mut counts = Counts::default();
    let mut items = Vec::with_capacity(jobs * 16);
    let mut iter = WalkDir::new(&root)
        .follow_links(false)
        .max_open(32)
        .into_iter();
    while let Some(item) = iter.next() {
        let e = match item {
            Ok(e) => e,
            Err(e) => {
                counts.errors += 1;
                issue(
                    &c,
                    id,
                    &e.path().map(|p| p.to_string_lossy()).unwrap_or_default(),
                    "walk",
                    &e.to_string(),
                )?;
                continue;
            }
        };
        let name = match text(e.path().strip_prefix(&root)?) {
            Ok(n) => n,
            Err(err) => {
                counts.errors += 1;
                issue(&c, id, "<non-UTF8>", "path", &err.to_string())?;
                if e.file_type().is_dir() {
                    iter.skip_current_dir();
                }
                continue;
            }
        };
        match fs::symlink_metadata(e.path()) {
            Ok(m) if unsafe_entry(&m) => {
                if e.file_type().is_dir() {
                    iter.skip_current_dir();
                }
                counts.skipped += 1;
                issue(&c, id, &name, "skipped", "link or reparse point")?;
            }
            Ok(m) if m.is_file() => {
                items.push(Work {
                    path: e.into_path(),
                    name,
                    cached: None,
                });
                if items.len() >= jobs * 16 {
                    batch(
                        &mut c,
                        id,
                        &root_name,
                        prior,
                        &pool,
                        &mut items,
                        &mut counts,
                    )?;
                }
            }
            Ok(m) if m.is_dir() => {}
            Ok(_) => {
                counts.errors += 1;
                issue(
                    &c,
                    id,
                    &name,
                    "unsupported",
                    "not regular file or directory",
                )?;
            }
            Err(err) => {
                if e.file_type().is_dir() {
                    iter.skip_current_dir();
                }
                counts.errors += 1;
                issue(&c, id, &name, "metadata", &err.to_string())?;
            }
        }
    }
    batch(
        &mut c,
        id,
        &root_name,
        prior,
        &pool,
        &mut items,
        &mut counts,
    )?;
    let complete = counts.errors == 0 && counts.skipped == 0;
    let state = if complete { "complete" } else { "partial" };
    c.execute("UPDATE runs SET state=?1,fresh=?2,reused=?3,bytes=?4,errors=?5,skipped=?6,elapsed_ms=?7,finished=CURRENT_TIMESTAMP WHERE id=?8",params![state,counts.fresh as i64,counts.reused as i64,i64::try_from(counts.bytes)?,counts.errors as i64,counts.skipped as i64,i64::try_from(start.elapsed().as_millis())?,id])?;
    println!(
        "{}",
        json!({"schema":1,"run_id":id,"state":state,"fresh":counts.fresh,"reused":counts.reused,"bytes":counts.bytes,"errors":counts.errors,"skipped":counts.skipped,"elapsed_ms":start.elapsed().as_millis(),"evidence_grade":complete&&!reuse})
    );
    Ok(complete)
}
fn run_meta(c: &Connection, id: i64) -> Result<(String, String, String)> {
    Ok(
        c.query_row("SELECT root,state,mode FROM runs WHERE id=?1", [id], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?))
        })?,
    )
}
fn emit(w: &mut impl Write, v: Value) -> Result<()> {
    serde_json::to_writer(&mut *w, &v)?;
    w.write_all(b"\n")?;
    Ok(())
}
fn export(db: &Path, id: i64) -> Result<bool> {
    let c = read_db(db)?;
    let (root, state, mode) = run_meta(&c, id)?;
    let stdout = io::stdout();
    let mut w = io::BufWriter::new(stdout.lock());
    emit(
        &mut w,
        json!({"kind":"header","schema":2,"run_id":id,"root":root,"mode":mode,"state":state}),
    )?;
    let mut s =
        c.prepare("SELECT path,size,sha256,source,error FROM files WHERE run_id=?1 ORDER BY path")?;
    let mut rows = s.query([id])?;
    while let Some(r) = rows.next()? {
        let error: Option<String> = r.get(4)?;
        emit(
            &mut w,
            json!({"kind":if error.is_some(){"error"}else{"file"},"path":r.get::<_,String>(0)?,"size":r.get::<_,i64>(1)?,"sha256":r.get::<_,String>(2)?,"source":r.get::<_,String>(3)?,"error":error}),
        )?;
    }
    let mut s = c.prepare("SELECT path,kind,detail FROM issues WHERE run_id=?1 ORDER BY id")?;
    let mut rows = s.query([id])?;
    while let Some(r) = rows.next()? {
        emit(
            &mut w,
            json!({"kind":"issue","path":r.get::<_,String>(0)?,"category":r.get::<_,String>(1)?,"detail":r.get::<_,String>(2)?}),
        )?;
    }
    emit(
        &mut w,
        json!({"kind":"summary","complete":state=="complete","evidence_grade":state=="complete"&&mode=="strict"}),
    )?;
    w.flush()?;
    Ok(state == "complete")
}
fn diff(db: &Path, old: i64, new: i64) -> Result<bool> {
    let c = read_db(db)?;
    let (r1, s1, m1) = run_meta(&c, old)?;
    let (r2, s2, m2) = run_meta(&c, new)?;
    if r1 != r2 || s1 != "complete" || s2 != "complete" || m1 != "strict" || m2 != "strict" {
        return Err(bad("diff requires two complete strict runs of same root"));
    }
    let mut q=c.prepare("SELECT a.path,CASE WHEN b.path IS NULL THEN 'removed' ELSE 'changed' END,a.sha256,b.sha256 FROM files a LEFT JOIN files b ON b.run_id=?2 AND b.path=a.path WHERE a.run_id=?1 AND (b.path IS NULL OR a.sha256<>b.sha256) UNION ALL SELECT b.path,'added',NULL,b.sha256 FROM files b LEFT JOIN files a ON a.run_id=?1 AND a.path=b.path WHERE b.run_id=?2 AND a.path IS NULL ORDER BY 1")?;
    let stdout = io::stdout();
    let mut w = io::BufWriter::new(stdout.lock());
    let mut rows = q.query(params![old, new])?;
    let mut count = 0u64;
    emit(
        &mut w,
        json!({"kind":"header","schema":1,"old":old,"new":new}),
    )?;
    while let Some(r) = rows.next()? {
        count += 1;
        emit(
            &mut w,
            json!({"kind":"difference","path":r.get::<_,String>(0)?,"change":r.get::<_,String>(1)?,"old_sha256":r.get::<_,Option<String>>(2)?,"new_sha256":r.get::<_,Option<String>>(3)?}),
        )?;
    }
    emit(
        &mut w,
        json!({"kind":"summary","differences":count,"complete":true}),
    )?;
    w.flush()?;
    Ok(true)
}
fn take(m: &mut BTreeMap<String, OsString>, key: &str) -> Result<OsString> {
    m.remove(key).ok_or_else(|| bad(&format!("missing {key}")))
}
fn num(v: OsString) -> Result<i64> {
    let n = v
        .to_str()
        .ok_or_else(|| bad("invalid number"))?
        .parse::<i64>()?;
    if n < 1 {
        return Err(bad("positive number required"));
    }
    Ok(n)
}
pub fn command(cmd: &str, args: impl Iterator<Item = OsString>) -> Result<bool> {
    let mut m = BTreeMap::new();
    let mut args = args;
    while let Some(f) = args.next() {
        let f = f.into_string().map_err(|_| bad("invalid flag"))?;
        let v = args.next().ok_or_else(|| bad("missing flag value"))?;
        if m.insert(f, v).is_some() {
            return Err(bad("duplicate flag"));
        }
    }
    let db = PathBuf::from(take(&mut m, "--db")?);
    match cmd {
        "index" => {
            let root = PathBuf::from(take(&mut m, "--root")?);
            let jobs = match m.remove("--jobs") {
                Some(v) => usize::try_from(num(v)?)?,
                None => 4,
            };
            let reuse = match m.remove("--mode") {
                None => false,
                Some(v) if v == "strict" => false,
                Some(v) if v == "reuse-metadata" => true,
                _ => return Err(bad("invalid mode")),
            };
            if !m.is_empty() {
                return Err(bad("unknown flags"));
            }
            index(&root, &db, jobs, reuse)
        }
        "export" => {
            let id = num(take(&mut m, "--run")?)?;
            if !m.is_empty() {
                return Err(bad("unknown flags"));
            }
            export(&db, id)
        }
        "diff" => {
            let old = num(take(&mut m, "--old")?)?;
            let new = num(take(&mut m, "--new")?)?;
            if !m.is_empty() {
                return Err(bad("unknown flags"));
            }
            diff(&db, old, new)
        }
        _ => Err(bad("unknown command")),
    }
}
