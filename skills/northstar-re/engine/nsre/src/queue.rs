//! Durable, explicitly driven offline lab queue. No daemon or target execution.
use rusqlite::{params, Connection, OpenFlags, OptionalExtension, TransactionBehavior};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    ffi::OsString,
    fs::{self, File, OpenOptions},
    io::{self, Read},
    path::Path,
    time::Duration,
};

const APP_ID: i64 = 0x4e53514a;
const LIMIT: u64 = 1024 * 1024;

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

fn digest(s: &str) -> String {
    format!("{:x}", Sha256::digest(s.as_bytes()))
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
         PRAGMA user_version=1;
         CREATE TABLE jobs(
            id TEXT PRIMARY KEY,
            input TEXT NOT NULL,
            input_sha256 TEXT NOT NULL,
            priority INTEGER NOT NULL,
            workers INTEGER NOT NULL,
            state TEXT NOT NULL CHECK(state IN
              ('queued','running','completed','failed','cancelled','interrupted')),
            result TEXT,
            result_sha256 TEXT
         );
         COMMIT;",
    )
    .map_err(io::Error::other)?;
    Ok(())
}

fn open(path: &Path) -> io::Result<Connection> {
    let m = fs::symlink_metadata(path)?;
    if !m.is_file() || crate::unsafe_entry(&m) {
        return Err(invalid("expected regular queue database"));
    }
    let c = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_WRITE)
        .map_err(io::Error::other)?;
    c.busy_timeout(Duration::from_secs(5))
        .map_err(io::Error::other)?;
    let app: i64 = c
        .query_row("PRAGMA application_id", [], |r| r.get(0))
        .map_err(io::Error::other)?;
    let version: i64 = c
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .map_err(io::Error::other)?;
    if app != APP_ID || version != 1 {
        return Err(invalid("not a compatible NorthStar queue"));
    }
    c.execute_batch("PRAGMA synchronous=FULL;")
        .map_err(io::Error::other)?;
    Ok(c)
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
    crate::batch::validate(&value, workers)?;
    c.execute(
        "INSERT INTO jobs(id,input,input_sha256,priority,workers,state)
         VALUES(?1,?2,?3,?4,?5,'queued')",
        params![id, input, digest(input), priority, workers as i64],
    )
    .map_err(io::Error::other)?;
    Ok(())
}

fn run_one(c: &mut Connection) -> io::Result<Value> {
    let tx = c
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(io::Error::other)?;
    let job = tx
        .query_row(
            "SELECT id,input,input_sha256,workers FROM jobs
             WHERE state='queued' ORDER BY priority DESC,rowid LIMIT 1",
            [],
            |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                    r.get::<_, i64>(3)?,
                ))
            },
        )
        .optional()
        .map_err(io::Error::other)?;
    let Some((id, input, expected, workers)) = job else {
        return Ok(json!({"state":"idle"}));
    };
    tx.execute(
        "UPDATE jobs SET state='running' WHERE id=?1 AND state='queued'",
        [&id],
    )
    .map_err(io::Error::other)?;
    tx.commit().map_err(io::Error::other)?;
    let result = if digest(&input) != expected {
        Err(invalid("input hash mismatch"))
    } else {
        serde_json::from_str(&input)
            .map_err(io::Error::other)
            .and_then(|v| crate::batch::evaluate(&v, workers as usize))
    };
    let (state, report) = match result {
        Ok(report) if report["passed"] == true => ("completed", report),
        Ok(report) => ("failed", report),
        Err(e) => ("failed", json!({"error":e.to_string()})),
    };
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

fn status(c: &Connection, id: &str) -> io::Result<Value> {
    let (state, result, hash) = c
        .query_row(
            "SELECT state,result,result_sha256 FROM jobs WHERE id=?1",
            [id],
            |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, Option<String>>(1)?,
                    r.get::<_, Option<String>>(2)?,
                ))
            },
        )
        .map_err(io::Error::other)?;
    let report: Option<Value> = match (result, hash) {
        (Some(body), Some(hash)) if digest(&body) == hash => {
            Some(serde_json::from_str(&body).map_err(io::Error::other)?)
        }
        (None, None) if state != "completed" && state != "failed" => None,
        _ => return Err(invalid("result integrity failure")),
    };
    Ok(json!({"id":id,"state":state,"result":report}))
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
    let usage = "queue init DB | submit DB ID INPUT PRIORITY WORKERS --reviewed-redacted | run-one DB | status DB ID | cancel DB ID | interrupt DB ID --worker-stopped";
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
        "status" if args.len() == 3 => println!("{}", status(&c, id()?)?),
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
        assert_eq!(run_one(&mut c).unwrap()["id"], "low");
        assert_eq!(run_one(&mut c).unwrap()["state"], "idle");
        assert_eq!(status(&c, "cancel").unwrap()["state"], "cancelled");
        drop(c);
        fs::remove_dir_all(dir).unwrap();
    }
}
