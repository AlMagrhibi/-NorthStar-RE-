//! Read-only inventory prototype. Never loads or executes target files.
mod batch;
mod framework;
mod index;
mod lab;
mod probe;
mod rpc;
use rayon::prelude::*;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    env,
    fs::{self, File, Metadata, OpenOptions},
    io::{self, BufWriter, Read, Write},
    path::{Path, PathBuf},
    process::ExitCode,
    time::Instant,
};
use walkdir::{DirEntry, WalkDir};

fn unsafe_entry(m: &Metadata) -> bool {
    if m.file_type().is_symlink() {
        return true;
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        m.file_attributes() & 0x400 != 0
    }
    #[cfg(not(windows))]
    {
        false
    }
}
fn entry_allowed(e: &DirEntry) -> bool {
    // Keep unreadable entries visible to error reporting.
    fs::symlink_metadata(e.path())
        .map(|m| !unsafe_entry(&m))
        .unwrap_or(true)
}
fn hash_file(path: &Path) -> io::Result<(u64, String)> {
    let pre = fs::symlink_metadata(path)?;
    if !pre.is_file() || unsafe_entry(&pre) {
        return Err(io::Error::other("not a regular non-reparse file"));
    }
    let mut f = File::open(path)?;
    let mut h = Sha256::new();
    let mut buf = vec![0u8; 256 * 1024];
    let mut bytes = 0u64;
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
        bytes += n as u64;
    }
    let post = fs::symlink_metadata(path)?;
    if unsafe_entry(&post)
        || pre.len() != bytes
        || post.len() != bytes
        || pre.modified()? != post.modified()?
    {
        return Err(io::Error::other("file changed during scan"));
    }
    Ok((bytes, format!("{:x}", h.finalize())))
}
fn line(w: &mut impl Write, value: &Value) -> io::Result<()> {
    serde_json::to_writer(&mut *w, value).map_err(io::Error::other)?;
    w.write_all(b"\n")
}
fn flush_batch(
    batch: &mut Vec<PathBuf>,
    root: &Path,
    pool: &rayon::ThreadPool,
    w: &mut impl Write,
    counts: &mut (u64, u64, u64),
) -> io::Result<()> {
    let results: Vec<_> = pool.install(|| {
        batch
            .par_iter()
            .map(|p| {
                let rel = p.strip_prefix(root).unwrap_or(p);
                match rel.to_str() {
                    Some(name) => match hash_file(p) {
                        Ok((size, sha256)) => (
                            json!({"kind":"file","path":name,"size":size,"sha256":sha256}),
                            size,
                            true,
                        ),
                        Err(e) => (
                            json!({"kind":"error","path":name,"error":e.to_string()}),
                            0,
                            false,
                        ),
                    },
                    None => (
                        json!({"kind":"error","error":"path cannot be represented as UTF-8"}),
                        0,
                        false,
                    ),
                }
            })
            .collect()
    });
    for (r, b, ok) in results {
        line(w, &r)?;
        if ok {
            counts.0 += 1;
            counts.1 += b;
        } else {
            counts.2 += 1;
        }
    }
    batch.clear();
    Ok(())
}
fn scan(root: &Path, out: &Path, jobs: usize) -> io::Result<bool> {
    if jobs == 0 || jobs > 64 {
        return Err(io::Error::other("jobs must be in 1..=64"));
    }
    let root_meta = fs::symlink_metadata(root)?;
    if unsafe_entry(&root_meta) || !root_meta.is_dir() {
        return Err(io::Error::other("root must be a normal directory"));
    }
    let root = root.canonicalize()?;
    let parent = out
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."))
        .canonicalize()?;
    let name = out
        .file_name()
        .ok_or_else(|| io::Error::other("output needs a file name"))?;
    if parent.starts_with(&root) {
        return Err(io::Error::other("output must be outside source tree"));
    }
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(jobs)
        .build()
        .map_err(io::Error::other)?;
    let mut w = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(parent.join(name))?,
    );
    let start = Instant::now();
    let mut counts = (0, 0, 0);
    let mut skipped = 0u64;
    line(
        &mut w,
        &json!({"kind":"header","schema":1,"root":root.to_string_lossy(),"jobs":jobs}),
    )?;
    let mut batch = Vec::with_capacity(jobs * 8);
    // Filter reparse trees before descending; interior symlinks are not followed.
    let iter = WalkDir::new(&root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            let allowed = entry_allowed(e);
            if !allowed {
                skipped += 1;
            }
            allowed
        });
    for item in iter {
        match item {
            Ok(e) if e.file_type().is_file() => {
                batch.push(e.into_path());
                if batch.len() >= jobs * 8 {
                    flush_batch(&mut batch, &root, &pool, &mut w, &mut counts)?;
                }
            }
            Ok(e) if !e.file_type().is_dir() => {
                counts.2 += 1;
                line(
                    &mut w,
                    &json!({"kind":"error","error":"unsupported entry","path":e.path().to_string_lossy()}),
                )?;
            }
            Ok(_) => {}
            Err(e) => {
                counts.2 += 1;
                line(&mut w, &json!({"kind":"error","error":e.to_string()}))?;
            }
        }
    }
    flush_batch(&mut batch, &root, &pool, &mut w, &mut counts)?;
    let complete = counts.2 == 0 && skipped == 0;
    line(
        &mut w,
        &json!({"kind":"summary","files":counts.0,"bytes":counts.1,"errors":counts.2,"skipped_links_or_reparse":skipped,"elapsed_ms":start.elapsed().as_millis(),"complete":complete}),
    )?;
    w.flush()?;
    w.get_ref().sync_all()?;
    Ok(complete)
}
fn run() -> io::Result<bool> {
    let mut args = env::args_os().skip(1);
    if args.next().as_deref() != Some(std::ffi::OsStr::new("scan")) {
        return Err(io::Error::other(
            "usage: nsre scan --root DIR --out NEW_FILE --jobs N",
        ));
    }
    let (mut root, mut out, mut jobs) = (None, None, None);
    while let Some(flag) = args.next() {
        let value = args
            .next()
            .ok_or_else(|| io::Error::other("missing flag value"))?;
        match flag.to_str() {
            Some("--root") if root.is_none() => root = Some(PathBuf::from(value)),
            Some("--out") if out.is_none() => out = Some(PathBuf::from(value)),
            Some("--jobs") if jobs.is_none() => {
                jobs = Some(
                    value
                        .to_str()
                        .and_then(|s| s.parse::<usize>().ok())
                        .ok_or_else(|| io::Error::other("invalid jobs"))?,
                )
            }
            _ => return Err(io::Error::other("unknown or duplicate option")),
        }
    }
    scan(
        &root.ok_or_else(|| io::Error::other("missing root"))?,
        &out.ok_or_else(|| io::Error::other("missing out"))?,
        jobs.unwrap_or(4),
    )
}
fn main() -> ExitCode {
    let mut args = env::args_os().skip(1);
    if let Some(cmd) = args.next().and_then(|x| x.into_string().ok()) {
        if cmd == "--help" || cmd == "help" {
            println!(
                "nsre 0.3.0: scan index export diff probe analyze batch {}",
                framework::COMMANDS.join(" ")
            );
            return ExitCode::SUCCESS;
        }
        if cmd == "batch" {
            return match batch::command(args) {
                Ok(true) => ExitCode::SUCCESS,
                Ok(false) => ExitCode::from(2),
                Err(e) => {
                    eprintln!("nsre: {e}");
                    ExitCode::FAILURE
                }
            };
        }
        if framework::COMMANDS.contains(&cmd.as_str()) {
            return match framework::command(&cmd, args) {
                Ok(true) => ExitCode::SUCCESS,
                Ok(false) => ExitCode::from(2),
                Err(e) => {
                    eprintln!("nsre: {e}");
                    ExitCode::FAILURE
                }
            };
        }
        if cmd == "analyze" {
            return match args.next() {
                Some(p) if args.next().is_none() => match probe::probe(Path::new(&p)) {
                    Ok(true) => ExitCode::SUCCESS,
                    Ok(false) => ExitCode::from(2),
                    Err(e) => {
                        eprintln!("nsre: {e}");
                        ExitCode::FAILURE
                    }
                },
                _ => ExitCode::FAILURE,
            };
        }
        if matches!(cmd.as_str(), "index" | "export" | "diff") {
            return match index::command(&cmd, args) {
                Ok(true) => ExitCode::SUCCESS,
                Ok(false) => ExitCode::from(2),
                Err(e) => {
                    eprintln!("nsre: {e}");
                    ExitCode::FAILURE
                }
            };
        }
        if cmd == "probe" {
            let path = args.next();
            if path.is_none() || args.next().is_some() {
                eprintln!("usage: nsre probe FILE");
                return ExitCode::FAILURE;
            }
            return match probe::probe(Path::new(&path.unwrap())) {
                Ok(true) => ExitCode::SUCCESS,
                Ok(false) => ExitCode::from(2),
                Err(e) => {
                    eprintln!("nsre: {e}");
                    ExitCode::FAILURE
                }
            };
        }
        if cmd == "--version" {
            println!("nsre {}", env!("CARGO_PKG_VERSION"));
            return ExitCode::SUCCESS;
        }
    }
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(2),
        Err(e) => {
            eprintln!("nsre: {e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn known_hash_and_scan() {
        let t = env::temp_dir().join(format!(
            "nsre-test-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let r = t.join("input");
        fs::create_dir_all(&r).unwrap();
        fs::write(r.join("a.txt"), b"abc").unwrap();
        assert_eq!(
            hash_file(&r.join("a.txt")).unwrap(),
            (
                3,
                "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad".into()
            )
        );
        assert!(scan(&r, &r.join("bad.jsonl"), 1).is_err());
        assert!(scan(&r, &t.join("out.jsonl"), 2).unwrap());
        assert!(scan(&r, &t.join("out.jsonl"), 2).is_err());
        let text = fs::read_to_string(t.join("out.jsonl")).unwrap();
        assert!(text.contains("\"complete\":true"));
        fs::remove_dir_all(t).unwrap();
    }
}
