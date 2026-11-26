//! Durable, explicitly driven offline lab queue. No daemon or target execution.
use rusqlite::{params, Connection, OpenFlags, OptionalExtension, TransactionBehavior};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    ffi::OsString,
    fs::{self, File, OpenOptions},
    io::{self, Read},
    path::Path,
    time::{Duration, Instant},
};

const APP_ID: i64 = 0x4e53514a;
const LIMIT: u64 = 1024 * 1024;

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

fn digest(s: &str) -> String {
    format!("{:x}", Sha256::digest(s.as_bytes()))
}

fn validate_source_scope(c: &Connection, value: &Value) -> io::Result<()> {
    if value["schema"] == 2 {
        let root = value["root"]
            .as_str()
            .ok_or_else(|| invalid("collector root required"))?;
        let root = crate::collect::normal_dir(Path::new(root))?;
        if let Some(db) = c.path().filter(|s| !s.is_empty()) {
            if Path::new(db).canonicalize()?.starts_with(root) {
                return Err(invalid(
                    "queue database must be outside collection source root",
                ));
            }
        }
    }
    Ok(())
}

fn init(path: &Path) -> io::Result<()> {
    let parent = path.parent().filter(|p| !p.as_os_str().is_empty());
    if let Some(parent) = parent {
        if crate::unsafe_entry(&fs::symlink_metadata(parent)?) {
            return Err(invalid("unsafe database parent"));
        }
    }
    OpenOptions::new().write(true).create_new(true).open(path)?;
    let c = Connection::open(path).map_err(io::Error::other)?;
    c.execute_batch(
        "PRAGMA journal_mode=WAL;
         PRAGMA synchronous=FULL;
         BEGIN IMMEDIATE;
         PRAGMA application_id=1314083146;
         PRAGMA user_version=2;
         CREATE TABLE jobs(
            id TEXT PRIMARY KEY,
            input TEXT NOT NULL,
            input_sha256 TEXT NOT NULL,
            priority INTEGER NOT NULL,
            workers INTEGER NOT NULL,
            state TEXT NOT NULL CHECK(state IN
              ('queued','running','completed','failed','cancelled','interrupted')),
            result TEXT,
            result_sha256 TEXT,
            skipped_claims INTEGER NOT NULL DEFAULT 0
         );
         CREATE TABLE dependencies(job TEXT NOT NULL REFERENCES jobs(id), dependency TEXT NOT NULL REFERENCES jobs(id), PRIMARY KEY(job,dependency));
         CREATE TABLE exclusive_keys(job TEXT NOT NULL REFERENCES jobs(id), key TEXT NOT NULL, PRIMARY KEY(job,key));
         COMMIT;",
    )
    .map_err(io::Error::other)?;
    Ok(())
}

fn open(path: &Path) -> io::Result<Connection> {
    open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_WRITE)
}

fn open_with_flags(path: &Path, flags: OpenFlags) -> io::Result<Connection> {
    let m = fs::symlink_metadata(path)?;
    if !m.is_file() || crate::unsafe_entry(&m) {
        return Err(invalid("expected regular queue database"));
    }
    let c = Connection::open_with_flags(path, flags).map_err(io::Error::other)?;
    c.busy_timeout(Duration::from_secs(5))
        .map_err(io::Error::other)?;
    let app: i64 = c
        .query_row("PRAGMA application_id", [], |r| r.get(0))
        .map_err(io::Error::other)?;
    let version: i64 = c
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .map_err(io::Error::other)?;
    if app != APP_ID || !matches!(version, 1 | 2) {
        return Err(invalid("not a compatible NorthStar queue"));
    }
    c.execute_batch("PRAGMA synchronous=FULL; PRAGMA foreign_keys=ON;")
        .map_err(io::Error::other)?;
    Ok(c)
}

pub(crate) fn read_status(path: &Path, id: &str) -> io::Result<Value> {
    let c = open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    c.execute_batch("PRAGMA query_only=ON;")
        .map_err(io::Error::other)?;
    status(&c, id)
}

fn submit(c: &Connection, id: &str, input: &str, priority: i64, workers: usize) -> io::Result<()> {
    if id.is_empty()
        || id.len() > 64
        || !id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
        || !(0..=100).contains(&priority)
        || !(1..=8).contains(&workers)
        || input.len() as u64 > LIMIT
    {
        return Err(invalid("invalid job id, priority, workers or input size"));
    }
    let value: Value = serde_json::from_str(input).map_err(io::Error::other)?;
    if value["schema"] == 2 {
        crate::collect::validate_job(&value, workers)?;
        validate_source_scope(c, &value)?;
    } else {
        crate::batch::validate(&value, workers)?;
    }
    let (dependencies, keys) = scheduling(&value)?;
    let tx = rusqlite::Transaction::new_unchecked(c, TransactionBehavior::Immediate)
        .map_err(io::Error::other)?;
    let version: i64 = tx
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .map_err(io::Error::other)?;
    if version == 1 && (value["schema"] == 2 || !dependencies.is_empty() || !keys.is_empty()) {
        return Err(invalid(
            "queue migrate required for scheduling or collection",
        ));
    }
    // Dependencies must already exist: append-only submission cannot introduce a cycle.
    for dependency in &dependencies {
        if dependency == id
            || !tx
                .query_row(
                    "SELECT EXISTS(SELECT 1 FROM jobs WHERE id=?1)",
                    [dependency],
                    |r| r.get::<_, bool>(0),
                )
                .map_err(io::Error::other)?
        {
            return Err(invalid("self or unknown queue dependency"));
        }
    }
    tx.execute(
        "INSERT INTO jobs(id,input,input_sha256,priority,workers,state)
         VALUES(?1,?2,?3,?4,?5,'queued')",
        params![id, input, digest(input), priority, workers as i64],
    )
    .map_err(io::Error::other)?;
    if version == 2 {
        for dependency in dependencies {
            tx.execute(
                "INSERT INTO dependencies VALUES(?1,?2)",
                params![id, dependency],
            )
            .map_err(io::Error::other)?;
        }
        for key in keys {
            tx.execute("INSERT INTO exclusive_keys VALUES(?1,?2)", params![id, key])
                .map_err(io::Error::other)?;
        }
    }
    tx.commit().map_err(io::Error::other)?;
    Ok(())
}

fn scheduling(value: &Value) -> io::Result<(Vec<String>, Vec<String>)> {
    let Some(schedule) = value.get("queue") else {
        return Ok((vec![], vec![]));
    };
    let object = schedule
        .as_object()
        .ok_or_else(|| invalid("queue must be an object"))?;
    if schedule["schema"] != 1
        || object
            .keys()
            .any(|key| !matches!(key.as_str(), "schema" | "depends_on" | "exclusive_keys"))
    {
        return Err(invalid("unsupported queue scheduling contract"));
    }
    let list = |name: &str, limit: usize| -> io::Result<Vec<String>> {
        let Some(value) = schedule.get(name) else {
            return Ok(vec![]);
        };
        let values = value
            .as_array()
            .filter(|a| a.len() <= limit)
            .ok_or_else(|| invalid("invalid scheduling list"))?;
        let mut result = std::collections::BTreeSet::new();
        for value in values {
            let text = value
                .as_str()
                .filter(|s| {
                    !s.is_empty()
                        && s.len() <= 64
                        && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
                })
                .ok_or_else(|| invalid("invalid scheduling identifier"))?;
            if !result.insert(text.to_owned()) {
                return Err(invalid("duplicate scheduling identifier"));
            }
        }
        Ok(result.into_iter().collect())
    };
    Ok((list("depends_on", 64)?, list("exclusive_keys", 16)?))
}

fn migrate(c: &mut Connection) -> io::Result<()> {
    let tx = c
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(io::Error::other)?;
    let version: i64 = tx
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .map_err(io::Error::other)?;
    if version == 1 {
        let running: i64 = tx
            .query_row("SELECT count(*) FROM jobs WHERE state='running'", [], |r| {
                r.get(0)
            })
            .map_err(io::Error::other)?;
        if running != 0 {
            return Err(invalid("stop and reconcile workers before migration"));
        }
        tx.execute_batch("ALTER TABLE jobs ADD COLUMN skipped_claims INTEGER NOT NULL DEFAULT 0;
            CREATE TABLE dependencies(job TEXT NOT NULL REFERENCES jobs(id), dependency TEXT NOT NULL REFERENCES jobs(id), PRIMARY KEY(job,dependency));
            CREATE TABLE exclusive_keys(job TEXT NOT NULL REFERENCES jobs(id), key TEXT NOT NULL, PRIMARY KEY(job,key));
            PRAGMA user_version=2;").map_err(io::Error::other)?;
    }
    tx.commit().map_err(io::Error::other)
}

fn claim(c: &mut Connection) -> io::Result<Option<(String, String, String, i64)>> {
    let tx = c
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(io::Error::other)?;
    let version: i64 = tx
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .map_err(io::Error::other)?;
    let eligible = "state='queued'
        AND NOT EXISTS(SELECT 1 FROM dependencies d JOIN jobs parent ON parent.id=d.dependency
            WHERE d.job=jobs.id AND parent.state IN ('queued','running'))
        AND NOT EXISTS(SELECT 1 FROM exclusive_keys wanted JOIN exclusive_keys held ON wanted.key=held.key
            JOIN jobs owner ON owner.id=held.job
            WHERE wanted.job=jobs.id AND owner.state='running')";
    let sql = if version == 2 {
        format!("SELECT id,input,input_sha256,workers FROM jobs WHERE {eligible} ORDER BY priority+skipped_claims DESC,rowid LIMIT 1")
    } else {
        "SELECT id,input,input_sha256,workers FROM jobs WHERE state='queued' ORDER BY priority DESC,rowid LIMIT 1".to_owned()
    };
    let job = tx
        .query_row(&sql, [], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, i64>(3)?,
            ))
        })
        .optional()
        .map_err(io::Error::other)?;
    if let Some((id, _, _, _)) = &job {
        if version == 2 {
            // Ready jobs age by successful claims, not wall clock; capped to avoid overflow.
            tx.execute(
                &format!(
                    "UPDATE jobs SET skipped_claims=min(skipped_claims+1,9000000000000000) WHERE {eligible}"
                ),
                [],
            )
            .map_err(io::Error::other)?;
        }
        tx.execute(
            "UPDATE jobs SET state='running' WHERE id=?1 AND state='queued'",
            [id],
        )
        .map_err(io::Error::other)?;
    }
    tx.commit().map_err(io::Error::other)?;
    Ok(job)
}

fn run_one(c: &mut Connection) -> io::Result<Value> {
    run_one_with(c, |c, v, workers| {
        if v["schema"] == 2 {
            validate_source_scope(c, &v)?;
            crate::collect::evaluate_job(&v, workers)
        } else {
            crate::batch::evaluate(&v, workers)
        }
    })
}

fn run_one_with<F>(c: &mut Connection, evaluate: F) -> io::Result<Value>
where
    F: FnOnce(&Connection, Value, usize) -> io::Result<Value>,
{
    let Some((id, input, expected, workers)) = claim(c)? else {
        return Ok(json!({"state":"idle"}));
    };
    let dependencies_passed = (|| -> io::Result<Vec<Value>> {
        if digest(&input) != expected {
            return Err(invalid("input hash mismatch"));
        }
        let value: Value = serde_json::from_str(&input).map_err(io::Error::other)?;
        let (dependencies, _) = scheduling(&value)?;
        let mut provenance = Vec::new();
        for dependency in dependencies {
            let report = status(c, &dependency)?;
            if report["state"] != "completed" {
                return Err(invalid("queue dependency did not pass"));
            }
            provenance.push(json!({"id":report["id"],"input_sha256":report["input_sha256"],"result_sha256":report["result_sha256"]}));
        }
        Ok(provenance)
    })();
    let (result, dependencies) = match dependencies_passed {
        Err(error) => (Err(error), vec![]),
        Ok(dependencies) => (
            serde_json::from_str(&input)
                .map_err(io::Error::other)
                .and_then(|v: Value| guarded_evaluation(|| evaluate(c, v, workers as usize))),
            dependencies,
        ),
    };
    let (state, mut report) = match result {
        Ok(report) if report["passed"] == true => ("completed", report),
        Ok(report) => ("failed", report),
        Err(e) => ("failed", json!({"error":e.to_string()})),
    };
    report["queue_provenance"] = json!({"schema":1,"producer":"nsre","producer_version":env!("CARGO_PKG_VERSION"),"input_sha256":expected,"dependencies":dependencies});
    let body = serde_json::to_string(&report).map_err(io::Error::other)?;
    let changed = c
        .execute(
            "UPDATE jobs SET state=?1,result=?2,result_sha256=?3
             WHERE id=?4 AND state='running'",
            params![state, body, digest(&body), id],
        )
        .map_err(io::Error::other)?;
    if changed != 1 {
        return Err(invalid("job no longer owns running state"));
    }
    Ok(json!({"id":id,"state":state}))
}

// An unwinding evaluator must not leave its claimed job silently running.
// Abort, power loss and process death still require owner/lease reconciliation.
fn guarded_evaluation<F>(evaluate: F) -> io::Result<Value>
where
    F: FnOnce() -> io::Result<Value>,
{
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(evaluate)).unwrap_or_else(|_| {
        Err(io::Error::other(
            "WORKER_LOST: evaluator unwound; result unverified",
        ))
    })
}

fn drain(c: &mut Connection, limit: usize) -> io::Result<Value> {
    if !(1..=64).contains(&limit) {
        return Err(invalid("drain limit must be within 1..=64"));
    }
    let mut jobs = Vec::new();
    let mut idle = false;
    for _ in 0..limit {
        let job = run_one(c)?;
        if job["state"] == "idle" {
            idle = true;
            break;
        }
        jobs.push(job);
    }
    let passed = jobs.iter().all(|job| job["state"] == "completed");
    Ok(json!({"schema":1,"passed":passed,"idle_observed":idle,"jobs":jobs}))
}

fn status(c: &Connection, id: &str) -> io::Result<Value> {
    let (state, input, input_hash, result, hash) = c
        .query_row(
            "SELECT state,input,input_sha256,result,result_sha256 FROM jobs WHERE id=?1",
            [id],
            |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                    r.get::<_, Option<String>>(3)?,
                    r.get::<_, Option<String>>(4)?,
                ))
            },
        )
        .map_err(io::Error::other)?;
    if digest(&input) != input_hash {
        return Err(invalid("input integrity failure"));
    }
    let report: Option<Value> = match (result, hash.as_ref()) {
        (Some(body), Some(hash)) if digest(&body) == *hash => {
            Some(serde_json::from_str(&body).map_err(io::Error::other)?)
        }
        (None, None) if state != "completed" && state != "failed" => None,
        _ => return Err(invalid("result integrity failure")),
    };
    if (state == "completed" && report.as_ref().is_none_or(|v| v["passed"] != true))
        || (state == "failed" && report.as_ref().is_some_and(|v| v["passed"] == true))
        || (!matches!(state.as_str(), "completed" | "failed") && report.is_some())
    {
        return Err(invalid("inconsistent state and result"));
    }
    Ok(json!({
        "id":id,"state":state,"result":report,
        "input_sha256":input_hash,"result_sha256":hash,
        "scheduling":serde_json::from_str::<Value>(&input).map_err(io::Error::other)?.get("queue")
    }))
}

fn wait(c: &Connection, id: &str, seconds: u64) -> io::Result<Value> {
    if seconds > 600 {
        return Err(invalid("wait timeout must be within 0..=600 seconds"));
    }
    let start = Instant::now();
    let timeout = Duration::from_secs(seconds);
    loop {
        let mut report = status(c, id)?;
        let terminal = matches!(
            report["state"].as_str(),
            Some("completed" | "failed" | "cancelled" | "interrupted")
        );
        if terminal || start.elapsed() >= timeout {
            report["terminal"] = json!(terminal);
            report["timed_out"] = json!(!terminal);
            return Ok(report);
        }
        std::thread::sleep(Duration::from_millis(100).min(timeout.saturating_sub(start.elapsed())));
    }
}

fn retry(c: &Connection, old_id: &str, new_id: &str) -> io::Result<()> {
    status(c, old_id)?;
    let (input, expected, priority, workers, state) = c
        .query_row(
            "SELECT input,input_sha256,priority,workers,state FROM jobs WHERE id=?1",
            [old_id],
            |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, i64>(2)?,
                    r.get::<_, i64>(3)?,
                    r.get::<_, String>(4)?,
                ))
            },
        )
        .map_err(io::Error::other)?;
    if state != "interrupted" || digest(&input) != expected || old_id == new_id {
        return Err(invalid("retry requires intact interrupted job and new id"));
    }
    submit(c, new_id, &input, priority, workers as usize)
}

fn transition(c: &Connection, id: &str, from: &str, to: &str) -> io::Result<()> {
    let count = c
        .execute(
            "UPDATE jobs SET state=?1 WHERE id=?2 AND state=?3",
            params![to, id, from],
        )
        .map_err(io::Error::other)?;
    if count != 1 {
        return Err(invalid("job missing or invalid state transition"));
    }
    Ok(())
}

pub fn command(args: impl Iterator<Item = OsString>) -> io::Result<bool> {
    let args: Vec<_> = args.collect();
    let usage = "queue init DB | migrate DB | submit DB ID INPUT PRIORITY WORKERS --reviewed-redacted | run-one DB | drain DB LIMIT | status DB ID | wait DB ID SECONDS | retry DB OLD_ID NEW_ID --reviewed-redacted | cancel DB ID | interrupt DB ID --worker-stopped";
    if args.len() < 2 {
        return Err(invalid(usage));
    }
    let db = Path::new(&args[1]);
    let verb = args[0].to_str().ok_or_else(|| invalid(usage))?;
    if verb == "init" && args.len() == 2 {
        init(db)?;
        return Ok(true);
    }
    let mut c = open(db)?;
    let id = || {
        args.get(2)
            .and_then(|a| a.to_str())
            .ok_or_else(|| invalid("job id required"))
    };
    match verb {
        "migrate" if args.len() == 2 => migrate(&mut c)?,
        "submit" if args.len() == 7 && args[6] == "--reviewed-redacted" => {
            let input = Path::new(&args[3]);
            let m = fs::symlink_metadata(input)?;
            if !m.is_file() || crate::unsafe_entry(&m) || m.len() > LIMIT {
                return Err(invalid("invalid input file"));
            }
            let mut body = String::new();
            File::open(input)?
                .take(LIMIT + 1)
                .read_to_string(&mut body)?;
            let priority = args[4]
                .to_str()
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| invalid("invalid priority"))?;
            let workers = args[5]
                .to_str()
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| invalid("invalid workers"))?;
            submit(&c, id()?, &body, priority, workers)?;
            println!("{}", json!({"id":id()?,"state":"queued"}));
        }
        "run-one" if args.len() == 2 => {
            let report = run_one(&mut c)?;
            println!("{report}");
            return Ok(report["state"] != "failed");
        }
        "drain" if args.len() == 3 => {
            let limit = args[2]
                .to_str()
                .and_then(|value| value.parse().ok())
                .ok_or_else(|| invalid("invalid drain limit"))?;
            let report = drain(&mut c, limit)?;
            println!("{report}");
            return Ok(report["passed"] == true);
        }
        "status" if args.len() == 3 => println!("{}", status(&c, id()?)?),
        "wait" if args.len() == 4 => {
            let seconds = args[3]
                .to_str()
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| invalid("invalid timeout"))?;
            let report = wait(&c, id()?, seconds)?;
            println!("{report}");
            return Ok(report["state"] == "completed" && report["terminal"] == true);
        }
        "retry" if args.len() == 5 && args[4] == "--reviewed-redacted" => {
            let new_id = args[3].to_str().ok_or_else(|| invalid("new id required"))?;
            retry(&c, id()?, new_id)?;
            println!("{}", json!({"id":new_id,"retry_of":id()?,"state":"queued"}));
        }
        "cancel" if args.len() == 3 => transition(&c, id()?, "queued", "cancelled")?,
        "interrupt" if args.len() == 4 && args[3] == "--worker-stopped" => {
            transition(&c, id()?, "running", "interrupted")?;
        }
        _ => return Err(invalid(usage)),
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn evaluator_panic_is_explicit_failure_without_panic_payload() {
        let error = guarded_evaluation(|| panic!("synthetic private payload")).unwrap_err();
        assert!(error.to_string().contains("WORKER_LOST"));
        assert!(!error.to_string().contains("private payload"));
        assert_eq!(
            guarded_evaluation(|| Ok(json!({"passed":true}))).unwrap()["passed"],
            true
        );
        assert!(guarded_evaluation(|| Err(invalid("fixture error"))).is_err());
    }

    #[test]
    fn panicked_job_persists_failure_and_blocks_dependent_after_reopen() {
        let fixture = Fixture::new();
        init(&fixture.db()).unwrap();
        let mut c = open(&fixture.db()).unwrap();
        submit(&c, "parent", &manifest(&[], &[], true), 1, 1).unwrap();
        submit(&c, "child", &manifest(&["parent"], &[], true), 1, 1).unwrap();
        let result = run_one_with(&mut c, |_, _, _| panic!("fixture evaluator fault")).unwrap();
        assert_eq!(result["state"], "failed");
        drop(c);
        let mut c = open(&fixture.db()).unwrap();
        let parent = status(&c, "parent").unwrap();
        assert_eq!(parent["state"], "failed");
        assert!(parent["result"]["error"]
            .as_str()
            .unwrap()
            .contains("WORKER_LOST"));
        let child = run_one_with(&mut c, |_, _, _| panic!("dependent must not execute")).unwrap();
        assert_eq!(child["state"], "failed");
        assert!(status(&c, "child").unwrap()["result"]["error"]
            .as_str()
            .unwrap()
            .contains("dependency"));
    }

    struct Fixture(std::path::PathBuf);
    impl Fixture {
        fn new() -> Self {
            let dir = std::env::temp_dir().join(format!(
                "nsre-scheduling-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            fs::create_dir(&dir).unwrap();
            Self(dir)
        }
        fn db(&self) -> std::path::PathBuf {
            self.0.join("queue.sqlite")
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    fn manifest(deps: &[&str], keys: &[&str], passed: bool) -> String {
        json!({"schema":1,"queue":{"schema":1,"depends_on":deps,"exclusive_keys":keys},"tasks":[{"id":"check","input":{"operation":"regression-check","baseline":true,"actual":passed}}]}).to_string()
    }

    #[test]
    fn cross_job_dependencies_fail_closed_and_survive_restart() {
        let f = Fixture::new();
        init(&f.db()).unwrap();
        let c = open(&f.db()).unwrap();
        assert!(submit(&c, "orphan", &manifest(&["absent"], &[], true), 0, 1).is_err());
        assert!(submit(&c, "self", &manifest(&["self"], &[], true), 0, 1).is_err());
        assert!(submit(&c, "dupe", &manifest(&["x", "x"], &[], true), 0, 1).is_err());
        submit(&c, "parent", &manifest(&[], &[], true), 0, 1).unwrap();
        submit(&c, "child", &manifest(&["parent"], &[], true), 100, 1).unwrap();
        drop(c);
        let mut c = open(&f.db()).unwrap();
        assert_eq!(run_one(&mut c).unwrap()["id"], "parent");
        assert_eq!(run_one(&mut c).unwrap()["id"], "child");
        submit(&c, "cancel", &manifest(&[], &[], true), 0, 1).unwrap();
        submit(&c, "blocked", &manifest(&["cancel"], &[], true), 100, 1).unwrap();
        submit(&c, "transitive", &manifest(&["blocked"], &[], true), 100, 1).unwrap();
        transition(&c, "cancel", "queued", "cancelled").unwrap();
        assert_eq!(run_one(&mut c).unwrap()["state"], "failed");
        assert_eq!(run_one(&mut c).unwrap()["state"], "failed");
        assert!(status(&c, "blocked").unwrap()["result"]["results"].is_null());
        // A successful state with corrupt provenance cannot authorize a dependent.
        submit(&c, "corrupt-child", &manifest(&["parent"], &[], true), 0, 1).unwrap();
        c.execute("UPDATE jobs SET result_sha256='bad' WHERE id='parent'", [])
            .unwrap();
        assert_eq!(run_one(&mut c).unwrap()["state"], "failed");
    }

    #[test]
    fn exclusive_keys_are_claimed_atomically_and_released_on_interruption() {
        let f = Fixture::new();
        init(&f.db()).unwrap();
        let mut a = open(&f.db()).unwrap();
        submit(&a, "writer-a", &manifest(&[], &["index"], true), 100, 1).unwrap();
        submit(&a, "writer-b", &manifest(&[], &["index"], true), 100, 1).unwrap();
        submit(&a, "other", &manifest(&[], &["other"], true), 0, 1).unwrap();
        assert_eq!(claim(&mut a).unwrap().unwrap().0, "writer-a");
        let mut b = open(&f.db()).unwrap();
        assert_eq!(claim(&mut b).unwrap().unwrap().0, "other");
        assert!(claim(&mut b).unwrap().is_none());
        transition(&a, "writer-a", "running", "interrupted").unwrap();
        assert_eq!(claim(&mut b).unwrap().unwrap().0, "writer-b");
        assert!(claim(&mut a).unwrap().is_none());
    }

    #[test]
    fn concurrent_claims_have_one_owner_for_an_exclusive_key() {
        let f = Fixture::new();
        init(&f.db()).unwrap();
        let c = open(&f.db()).unwrap();
        for i in 0..8 {
            submit(
                &c,
                &format!("job-{i}"),
                &manifest(&[], &["shared"], true),
                50,
                1,
            )
            .unwrap();
        }
        drop(c);
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(8));
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let db = f.db();
                let barrier = barrier.clone();
                std::thread::spawn(move || {
                    let mut c = open(&db).unwrap();
                    barrier.wait();
                    claim(&mut c).unwrap()
                })
            })
            .collect();
        let claims: Vec<_> = handles
            .into_iter()
            .filter_map(|h| h.join().unwrap())
            .collect();
        assert_eq!(claims.len(), 1);
        let mut c = open(&f.db()).unwrap();
        transition(&c, &claims[0].0, "running", "interrupted").unwrap();
        assert!(claim(&mut c).unwrap().is_some());
    }

    #[test]
    fn ready_job_ages_past_continuous_high_priority_arrivals() {
        let f = Fixture::new();
        init(&f.db()).unwrap();
        let mut c = open(&f.db()).unwrap();
        let input = manifest(&[], &[], true);
        submit(&c, "low", &input, 0, 1).unwrap();
        let mut low_claimed = false;
        for i in 0..=101 {
            submit(&c, &format!("high-{i}"), &input, 100, 1).unwrap();
            if run_one(&mut c).unwrap()["id"] == "low" {
                low_claimed = true;
                break;
            }
        }
        assert!(
            low_claimed,
            "new high-priority arrivals must not starve an eligible job"
        );
    }

    #[test]
    fn schema_one_migration_preserves_jobs_and_refuses_running_workers() {
        let f = Fixture::new();
        let c = Connection::open(f.db()).unwrap();
        c.execute_batch("PRAGMA application_id=1314083146; PRAGMA user_version=1;
            CREATE TABLE jobs(id TEXT PRIMARY KEY,input TEXT NOT NULL,input_sha256 TEXT NOT NULL,priority INTEGER NOT NULL,workers INTEGER NOT NULL,state TEXT NOT NULL,result TEXT,result_sha256 TEXT);").unwrap();
        drop(c);
        let mut c = open(&f.db()).unwrap();
        let input =
            r#"{"schema":1,"tasks":[{"id":"a","input":{"operation":"json-roundtrip","data":1}}]}"#;
        submit(&c, "old", input, 50, 1).unwrap();
        assert!(submit(&c, "new", &manifest(&["old"], &[], true), 0, 1).is_err());
        assert_eq!(claim(&mut c).unwrap().unwrap().0, "old");
        assert!(migrate(&mut c).is_err());
        transition(&c, "old", "running", "interrupted").unwrap();
        migrate(&mut c).unwrap();
        migrate(&mut c).unwrap();
        assert_eq!(status(&c, "old").unwrap()["input_sha256"], digest(input));
        retry(&c, "old", "retry").unwrap();
        submit(&c, "new", &manifest(&["retry"], &[], true), 100, 1).unwrap();
        assert_eq!(run_one(&mut c).unwrap()["id"], "retry");
        assert_eq!(run_one(&mut c).unwrap()["id"], "new");
    }

    #[test]
    fn scheduled_collection_rehashes_changed_sources_without_cache() {
        let f = Fixture::new();
        init(&f.db()).unwrap();
        let source = f.0.join("source");
        fs::create_dir(&source).unwrap();
        fs::write(source.join("a.txt"), b"abc").unwrap();
        let expected = digest("abc");
        let mut input = json!({"schema":2,"kind":"collect","root":source,"manifest":{"schema":1,"files":[{"path":"a.txt","expected_sha256":expected}]}});
        let mut c = open(&f.db()).unwrap();
        submit(&c, "collect", &input.to_string(), 50, 1).unwrap();
        assert_eq!(run_one(&mut c).unwrap()["state"], "completed");
        let result = status(&c, "collect").unwrap();
        assert_eq!(result["result"]["results"][0]["sha256"], expected);
        assert_eq!(result["result"]["results"][0]["hash_recomputed"], true);
        fs::write(source.join("a.txt"), b"changed").unwrap();
        input["queue"] = json!({"schema":1,"depends_on":["collect"]});
        submit(&c, "stale", &input.to_string(), 50, 1).unwrap();
        assert_eq!(run_one(&mut c).unwrap()["state"], "failed");
        assert_eq!(
            status(&c, "stale").unwrap()["result"]["results"][0]["sha256"],
            digest("changed")
        );
        input["manifest"]["files"][0]["path"] = json!("../queue.sqlite");
        assert!(submit(&c, "escape", &input.to_string(), 50, 1).is_err());
        assert!(status(&c, "escape").is_err());
        input["root"] = json!("relative");
        assert!(submit(&c, "relative", &input.to_string(), 50, 1).is_err());
        input["kind"] = json!("shell");
        assert!(submit(&c, "shell", &input.to_string(), 50, 1).is_err());
    }

    #[test]
    fn durable_priority_cancel_and_results() {
        let dir = std::env::temp_dir().join(format!(
            "nsre-queue-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&dir).unwrap();
        let db = dir.join("jobs.sqlite");
        init(&db).unwrap();
        let c = open(&db).unwrap();
        let input = r#"{"schema":1,"tasks":[{"id":"one","input":{"operation":"json-roundtrip","data":1}}]}"#;
        submit(&c, "low", input, 0, 1).unwrap();
        submit(&c, "high", input, 100, 1).unwrap();
        submit(&c, "cancel", input, 50, 1).unwrap();
        assert!(submit(&c, "high", input, 100, 1).is_err());
        transition(&c, "cancel", "queued", "cancelled").unwrap();
        drop(c);
        let mut c = open(&db).unwrap();
        assert_eq!(run_one(&mut c).unwrap()["id"], "high");
        assert_eq!(status(&c, "high").unwrap()["state"], "completed");
        let report = drain(&mut c, 2).unwrap();
        assert_eq!(report["jobs"][0]["id"], "low");
        assert_eq!(report["idle_observed"], true);
        assert_eq!(report["passed"], true);
        assert!(drain(&mut c, 0).is_err());
        assert_eq!(status(&c, "cancel").unwrap()["state"], "cancelled");
        let complete = wait(&c, "high", 0).unwrap();
        assert_eq!(complete["terminal"], true);
        assert_eq!(complete["input_sha256"], digest(input));
        assert!(complete["result_sha256"].is_string());
        submit(&c, "crashed", input, 10, 1).unwrap();
        assert_eq!(wait(&c, "crashed", 0).unwrap()["timed_out"], true);
        assert!(retry(&c, "crashed", "retry").is_err());
        transition(&c, "crashed", "queued", "running").unwrap();
        transition(&c, "crashed", "running", "interrupted").unwrap();
        retry(&c, "crashed", "retry").unwrap();
        assert!(retry(&c, "crashed", "retry").is_err());
        assert_eq!(run_one(&mut c).unwrap()["id"], "retry");
        assert_eq!(status(&c, "crashed").unwrap()["state"], "interrupted");
        assert_eq!(wait(&c, "retry", 0).unwrap()["state"], "completed");
        c.execute("UPDATE jobs SET input='tampered' WHERE id='retry'", [])
            .unwrap();
        assert!(status(&c, "retry").is_err());
        drop(c);
        fs::remove_dir_all(dir).unwrap();
    }
}
