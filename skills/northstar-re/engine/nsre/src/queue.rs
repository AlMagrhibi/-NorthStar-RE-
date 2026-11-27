//! Durable, explicitly driven offline lab queue. No daemon or target execution.
use rusqlite::{params, Connection, OpenFlags, OptionalExtension, TransactionBehavior};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    ffi::OsString,
    fs::{self, File, OpenOptions},
    io::{self, Read},
    path::Path,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

const APP_ID: i64 = 0x4e53514a;
const LIMIT: u64 = 1024 * 1024;
const SCHEMA_VERSION: i64 = 5;
const DEFAULT_LEASE_MS: u64 = 30_000;
const MAX_LEASE_MS: u64 = 3_600_000;

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
         PRAGMA user_version=5;
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
            skipped_claims INTEGER NOT NULL DEFAULT 0,
            lease_owner TEXT,
            lease_token INTEGER NOT NULL DEFAULT 0,
            lease_ms INTEGER NOT NULL DEFAULT 0,
            lease_expires_ms INTEGER,
            heartbeat_ms INTEGER,
            attempt INTEGER NOT NULL DEFAULT 0,
            cancel_requested INTEGER NOT NULL DEFAULT 0,
            progress TEXT,
            holds_keys INTEGER NOT NULL DEFAULT 0,
            worker_stop_verified INTEGER NOT NULL DEFAULT 0,
            effectful INTEGER NOT NULL DEFAULT 0,
            effect_verified INTEGER NOT NULL DEFAULT 0,
            worker_pid INTEGER,
            worker_created_ms INTEGER
         );
         CREATE TABLE lease_events(
            at_utc_ms INTEGER NOT NULL,
            job TEXT NOT NULL,
            kind TEXT NOT NULL,
            worker TEXT,
            token INTEGER,
            detail TEXT
         );
         CREATE INDEX lease_events_job ON lease_events(job, at_utc_ms);
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
    if app != APP_ID || !matches!(version, 1..=5) {
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
    submit_attempt(c, id, input, priority, workers, None)
}

fn submit_attempt(
    c: &Connection,
    id: &str,
    input: &str,
    priority: i64,
    workers: usize,
    retry_of: Option<&str>,
) -> io::Result<()> {
    if id.is_empty()
        || id.len() > 64
        || !id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
        || !(0..=100).contains(&priority)
        || !(1..=8).contains(&workers)
        || input.len() as u64 > LIMIT
    {
        return Err(invalid("invalid job id, priority, workers or input size"));
    }
    let version: i64 = c
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .map_err(io::Error::other)?;
    if version < SCHEMA_VERSION {
        return Err(invalid(
            "queue migrate required before submitting to this database",
        ));
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
    // Exclusive keys are a contract for a real-world effect, so the job cannot be replayed
    // after an uncertain stop until that effect is verified.
    let effectful = !keys.is_empty() || value.get("desktop").is_some();
    tx.execute(
        "INSERT INTO jobs(id,input,input_sha256,priority,workers,state,effectful)
         VALUES(?1,?2,?3,?4,?5,'queued',?6)",
        params![
            id,
            input,
            digest(input),
            priority,
            workers as i64,
            effectful as i64
        ],
    )
    .map_err(io::Error::other)?;
    if let Some(parent) = retry_of {
        tx.execute_batch("CREATE TABLE IF NOT EXISTS retry_links(job TEXT PRIMARY KEY REFERENCES jobs(id), parent TEXT NOT NULL REFERENCES jobs(id))").map_err(io::Error::other)?;
        let inserted = tx.execute("INSERT INTO retry_links SELECT ?1,id FROM jobs WHERE id=?2 AND state='interrupted' AND input=?3 AND input_sha256=?4", params![id,parent,input,digest(input)]).map_err(io::Error::other)?;
        if inserted != 1 {
            return Err(invalid("retry parent changed before commit"));
        }
    }
    if version >= 2 {
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
    let running: i64 = tx
        .query_row("SELECT count(*) FROM jobs WHERE state='running'", [], |r| {
            r.get(0)
        })
        .map_err(io::Error::other)?;
    if running != 0 {
        return Err(invalid("stop and reconcile workers before migration"));
    }
    if version == 1 {
        tx.execute_batch("ALTER TABLE jobs ADD COLUMN skipped_claims INTEGER NOT NULL DEFAULT 0;
            CREATE TABLE dependencies(job TEXT NOT NULL REFERENCES jobs(id), dependency TEXT NOT NULL REFERENCES jobs(id), PRIMARY KEY(job,dependency));
            CREATE TABLE exclusive_keys(job TEXT NOT NULL REFERENCES jobs(id), key TEXT NOT NULL, PRIMARY KEY(job,key));").map_err(io::Error::other)?;
    }
    if version < 3 {
        // Lease columns are additive with defaults, so existing rows keep their meaning.
        tx.execute_batch(
            "ALTER TABLE jobs ADD COLUMN lease_owner TEXT;
            ALTER TABLE jobs ADD COLUMN lease_token INTEGER NOT NULL DEFAULT 0;
            ALTER TABLE jobs ADD COLUMN lease_ms INTEGER NOT NULL DEFAULT 0;
            ALTER TABLE jobs ADD COLUMN lease_expires_ms INTEGER;
            ALTER TABLE jobs ADD COLUMN heartbeat_ms INTEGER;
            ALTER TABLE jobs ADD COLUMN attempt INTEGER NOT NULL DEFAULT 0;
            ALTER TABLE jobs ADD COLUMN cancel_requested INTEGER NOT NULL DEFAULT 0;
            ALTER TABLE jobs ADD COLUMN progress TEXT;
            CREATE TABLE lease_events(
               at_utc_ms INTEGER NOT NULL,
               job TEXT NOT NULL,
               kind TEXT NOT NULL,
               worker TEXT,
               token INTEGER,
               detail TEXT
            );
            CREATE INDEX lease_events_job ON lease_events(job, at_utc_ms);",
        )
        .map_err(io::Error::other)?;
    }
    if version < 4 {
        // Reserved exclusive keys survive lease expiry until the worker stop is verified.
        let has_column: bool = tx
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM pragma_table_info('jobs') WHERE name='holds_keys')",
                [],
                |r| r.get(0),
            )
            .map_err(io::Error::other)?;
        if !has_column {
            tx.execute_batch("ALTER TABLE jobs ADD COLUMN holds_keys INTEGER NOT NULL DEFAULT 0;")
                .map_err(io::Error::other)?;
        }
        let has_stop: bool = tx
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM pragma_table_info('jobs') WHERE name='worker_stop_verified')",
                [],
                |r| r.get(0),
            )
            .map_err(io::Error::other)?;
        if !has_stop {
            tx.execute_batch(
                "ALTER TABLE jobs ADD COLUMN worker_stop_verified INTEGER NOT NULL DEFAULT 0;",
            )
            .map_err(io::Error::other)?;
        }
        let has_effect: bool = tx
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM pragma_table_info('jobs') WHERE name='effectful')",
                [],
                |r| r.get(0),
            )
            .map_err(io::Error::other)?;
        if !has_effect {
            tx.execute_batch(
                "ALTER TABLE jobs ADD COLUMN effectful INTEGER NOT NULL DEFAULT 0;
                 ALTER TABLE jobs ADD COLUMN effect_verified INTEGER NOT NULL DEFAULT 0;",
            )
            .map_err(io::Error::other)?;
        }
        let has_identity: bool = tx
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM pragma_table_info('jobs') WHERE name='worker_pid')",
                [],
                |r| r.get(0),
            )
            .map_err(io::Error::other)?;
        if !has_identity {
            tx.execute_batch(
                "ALTER TABLE jobs ADD COLUMN worker_pid INTEGER;
                 ALTER TABLE jobs ADD COLUMN worker_created_ms INTEGER;",
            )
            .map_err(io::Error::other)?;
        }
        // A running job from a previous schema holds whatever keys it reserved.
        tx.execute_batch(
            "UPDATE jobs SET holds_keys=1 WHERE state IN ('running','interrupted')
               AND EXISTS(SELECT 1 FROM exclusive_keys k WHERE k.job=jobs.id);",
        )
        .map_err(io::Error::other)?;
    }
    tx.execute_batch(&format!("PRAGMA user_version={SCHEMA_VERSION};"))
        .map_err(io::Error::other)?;
    tx.commit().map_err(io::Error::other)
}

fn has_table(c: &Connection, name: &str) -> io::Result<bool> {
    c.query_row(
        "SELECT EXISTS(SELECT 1 FROM sqlite_schema WHERE type='table' AND name=?1)",
        [name],
        |r| r.get(0),
    )
    .map_err(io::Error::other)
}

/// Append-only lease trail. Old schemas without the table stay readable and skip logging.
fn log_event(
    c: &Connection,
    job: &str,
    kind: &str,
    worker: Option<&str>,
    token: Option<i64>,
    detail: Option<&str>,
) -> io::Result<()> {
    if !has_table(c, "lease_events")? {
        let version: i64 = c
            .query_row("PRAGMA user_version", [], |r| r.get(0))
            .map_err(io::Error::other)?;
        if version >= 4 {
            return Err(io::Error::other(
                "durable evidence is missing; refusing to change lease state",
            ));
        }
        return Ok(());
    }
    c.execute(
        "INSERT INTO lease_events(at_utc_ms,job,kind,worker,token,detail) VALUES(?1,?2,?3,?4,?5,?6)",
        params![utc_millis()? as i64, job, kind, worker, token, detail],
    )
    .map_err(io::Error::other)?;
    Ok(())
}

struct Claim {
    id: String,
    input: String,
    expected: String,
    workers: i64,
    token: i64,
}

fn claim(
    c: &mut Connection,
    worker: &str,
    lease_ms: u64,
    only_job: Option<&str>,
) -> io::Result<Option<Claim>> {
    let tx = c
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(io::Error::other)?;
    let version: i64 = tx
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .map_err(io::Error::other)?;
    if version < SCHEMA_VERSION {
        return Err(invalid(
            "queue migrate required before running workers on this database",
        ));
    }
    let eligible = if version >= 4 {
        // An interrupted job keeps its exclusive keys until the worker stop is verified,
        // because an expired lease is not proof that the old worker stopped acting.
        "state='queued'
        AND NOT EXISTS(SELECT 1 FROM dependencies d JOIN jobs parent ON parent.id=d.dependency
            WHERE d.job=jobs.id AND parent.state IN ('queued','running'))
        AND NOT EXISTS(SELECT 1 FROM exclusive_keys wanted JOIN exclusive_keys held ON wanted.key=held.key
            JOIN jobs owner ON owner.id=held.job
            WHERE wanted.job=jobs.id
              AND (owner.state='running' OR (owner.state='interrupted' AND owner.holds_keys=1)))"
    } else {
        "state='queued'
        AND NOT EXISTS(SELECT 1 FROM dependencies d JOIN jobs parent ON parent.id=d.dependency
            WHERE d.job=jobs.id AND parent.state IN ('queued','running'))
        AND NOT EXISTS(SELECT 1 FROM exclusive_keys wanted JOIN exclusive_keys held ON wanted.key=held.key
            JOIN jobs owner ON owner.id=held.job
            WHERE wanted.job=jobs.id AND owner.state='running')"
    };
    let pinned = only_job
        .map(|id| format!(" AND id='{}'", id.replace('\'', "''")))
        .unwrap_or_default();
    let sql = if version >= 2 {
        format!("SELECT id,input,input_sha256,workers FROM jobs WHERE {eligible}{pinned} ORDER BY priority+skipped_claims DESC,rowid LIMIT 1")
    } else {
        format!("SELECT id,input,input_sha256,workers FROM jobs WHERE state='queued'{pinned} ORDER BY priority DESC,rowid LIMIT 1")
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
    let Some((id, input, expected, workers)) = job else {
        tx.commit().map_err(io::Error::other)?;
        return Ok(None);
    };
    if version >= 2 {
        // Ready jobs age by successful claims, not wall clock; capped to avoid overflow.
        tx.execute(
            &format!(
                "UPDATE jobs SET skipped_claims=min(skipped_claims+1,9000000000000000) WHERE {eligible}"
            ),
            [],
        )
        .map_err(io::Error::other)?;
    }
    let token = {
        let now = utc_millis()? as i64;
        let holds: bool = tx
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM exclusive_keys WHERE job=?1)",
                [&id],
                |r| r.get(0),
            )
            .map_err(io::Error::other)?;
        // heartbeat_ms records the claim instant; lease_expires_ms is the deadline.
        // The claiming process records its own creation identity so a reused PID can never
        // pass for the original worker.
        let (worker_pid, worker_created_ms) = (
            std::process::id() as i64,
            crate::supervisor::self_creation_ms()?.unwrap_or(0),
        );
        let changed = tx
            .execute(
                "UPDATE jobs SET state='running', lease_owner=?2, lease_token=lease_token+1,
                   lease_ms=?3, lease_expires_ms=?4, heartbeat_ms=?5, attempt=attempt+1,
                   progress=NULL, holds_keys=?6, worker_pid=?7, worker_created_ms=?8
                 WHERE id=?1 AND state='queued'",
                params![
                    id,
                    worker,
                    lease_ms as i64,
                    now + lease_ms as i64,
                    now,
                    holds as i64,
                    worker_pid,
                    worker_created_ms as i64
                ],
            )
            .map_err(io::Error::other)?;
        if changed != 1 {
            return Err(invalid("job was claimed concurrently"));
        }
        tx.query_row("SELECT lease_token FROM jobs WHERE id=?1", [&id], |r| {
            r.get::<_, i64>(0)
        })
        .map_err(io::Error::other)?
    };
    if version >= 3 {
        log_event(
            &tx,
            &id,
            "claimed",
            Some(worker),
            Some(token),
            Some(&format!("lease_ms={lease_ms}")),
        )?;
    }
    tx.commit().map_err(io::Error::other)?;
    Ok(Some(Claim {
        id,
        input,
        expected,
        workers,
        token,
    }))
}

fn heartbeat(
    c: &Connection,
    id: &str,
    worker: &str,
    token: i64,
    lease_ms: u64,
) -> io::Result<Value> {
    if !(1_000..=MAX_LEASE_MS).contains(&lease_ms) {
        return Err(invalid("lease must be within 1000..=3600000 ms"));
    }
    let tx = c.unchecked_transaction().map_err(io::Error::other)?;
    // The deadline is measured while the write lock is held, so a slow lock acquisition
    // cannot silently shorten the new lease.
    let now = utc_millis()? as i64;
    let expires = now + lease_ms as i64;
    // Ownership, token, state, cancellation and lease validity are enforced by the same
    // statement: a worker whose lease already expired cannot renew it just because
    // reconciliation has not run yet.
    let changed = tx
        .execute(
            "UPDATE jobs SET heartbeat_ms=?4, lease_expires_ms=?5, lease_ms=?6
             WHERE id=?1 AND state='running' AND lease_owner=?2 AND lease_token=?3
               AND cancel_requested=0 AND lease_expires_ms IS NOT NULL
               AND lease_expires_ms > ?4",
            params![id, worker, token, now, expires, lease_ms as i64],
        )
        .map_err(io::Error::other)?;
    if changed != 1 {
        return Err(invalid(
            "LEASE_LOST: worker does not own a live lease for this job",
        ));
    }
    log_event(&tx, id, "heartbeat", Some(worker), Some(token), None)?;
    tx.commit().map_err(io::Error::other)?;
    Ok(json!({
        "id":id,"state":"running","lease_owner":worker,"lease_token":token,
        "heartbeat_utc_ms":now,"lease_expires_utc_ms":expires
    }))
}

/// Expired leases become `interrupted`, never silently requeued: expiry does not prove the
/// worker stopped, and replaying an effectful job is unsafe until a human or supervisor
/// verifies the effect. Requeueing stays an explicit `retry` decision with its own evidence.
///
/// The scan is only a hint. Every row is re-checked against the live expiry and fencing
/// token inside its own conditional statement, so a heartbeat that lands between the scan
/// and the update wins instead of being overwritten by a stale decision.
fn reconcile(c: &Connection) -> io::Result<Value> {
    let now = utc_millis()? as i64;
    let mut expired = Vec::new();
    {
        let mut stmt = c
            .prepare(
                "SELECT id,lease_owner,lease_token,lease_expires_ms,cancel_requested,holds_keys FROM jobs
                 WHERE state='running' AND lease_expires_ms IS NOT NULL AND lease_expires_ms <= ?1
                 ORDER BY id",
            )
            .map_err(io::Error::other)?;
        let rows = stmt
            .query_map([now], |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, Option<String>>(1)?,
                    r.get::<_, i64>(2)?,
                    r.get::<_, i64>(3)?,
                    r.get::<_, i64>(4)?,
                    r.get::<_, i64>(5)?,
                ))
            })
            .map_err(io::Error::other)?;
        for row in rows {
            expired.push(row.map_err(io::Error::other)?);
        }
    }
    let mut reconciled = Vec::new();
    for (id, owner, token, expires, cancel_requested, holds_keys) in expired {
        let tx = c.unchecked_transaction().map_err(io::Error::other)?;
        let changed = tx
            .execute(
                "UPDATE jobs SET state='interrupted', lease_expires_ms=NULL, heartbeat_ms=NULL
                 WHERE id=?1 AND state='running' AND lease_token=?2
                   AND lease_expires_ms IS NOT NULL AND lease_expires_ms <= ?3",
                params![id, token, now],
            )
            .map_err(io::Error::other)?;
        if changed != 1 {
            // A heartbeat renewed the lease after the scan, or another reconciler won.
            continue;
        }
        let (kind, detail) = if cancel_requested == 1 {
            (
                "cancel_unconfirmed",
                "cancellation was requested but the owner never confirmed it; worker stop is unverified",
            )
        } else {
            (
                "lease_expired",
                "reconciled to interrupted; effect not verified",
            )
        };
        // Status change and its evidence commit together, or not at all.
        log_event(&tx, &id, kind, owner.as_deref(), Some(token), Some(detail))?;
        tx.commit().map_err(io::Error::other)?;
        reconciled.push(json!({
            "id":id,"state":"interrupted","previous_owner":owner,
            "lease_token":token,"lease_expired_utc_ms":expires,
            "cancel_requested":cancel_requested == 1,
            "keys_held":holds_keys == 1,"worker_stop_verified":false
        }));
    }
    let running: i64 = c
        .query_row("SELECT count(*) FROM jobs WHERE state='running'", [], |r| {
            r.get(0)
        })
        .map_err(io::Error::other)?;
    Ok(json!({
        "schema":1,"at_utc_ms":now,"reconciled":reconciled,
        "still_running":running,"unverified_effect":true
    }))
}

/// Running cancellation is a request, not a kill: the owner observes it at its next
/// checkpoint or commit boundary and releases the lease itself.
fn cancel_running(c: &Connection, id: &str) -> io::Result<Value> {
    let tx = c.unchecked_transaction().map_err(io::Error::other)?;
    let changed = tx
        .execute(
            "UPDATE jobs SET cancel_requested=1 WHERE id=?1 AND state='running'",
            [id],
        )
        .map_err(io::Error::other)?;
    if changed != 1 {
        return Err(invalid("cancellation request requires a running job"));
    }
    log_event(&tx, id, "cancel_requested", None, None, None)?;
    tx.commit().map_err(io::Error::other)?;
    Ok(json!({"id":id,"state":"running","cancel_requested":true}))
}

/// Record a verified stop from an observed process exit (supervisor kill or crash).
pub fn record_verified_stop(db: &Path, id: &str) -> io::Result<Value> {
    let c = open(db)?;
    worker_stopped(&c, id)
}

/// Supervisor-driven stop verification: the operating system, not a statement, proves the
/// worker exited. PID reuse is defeated by comparing the recorded creation time.
pub fn verify_stopped(db: &Path, id: &str) -> io::Result<Value> {
    let c = open(db)?;
    let (state, pid, created): (String, Option<i64>, Option<i64>) = c
        .query_row(
            "SELECT state,worker_pid,worker_created_ms FROM jobs WHERE id=?1",
            [id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .map_err(io::Error::other)?;
    if state != "running" && state != "interrupted" {
        return Err(invalid(
            "worker verification applies to a live or interrupted job",
        ));
    }
    let report = crate::supervisor::verify_worker_gone(pid, created);
    if report["worker_stop_verified"] == true {
        worker_stopped(&c, id)?;
    }
    Ok(json!({"id":id,"state":state,"verification":report}))
}

/// Recording that the operator verified the worker exited. This is the only path that
/// releases exclusive reservations held by an interrupted job and the only one that makes
/// a requested cancellation final.
fn worker_stopped(c: &Connection, id: &str) -> io::Result<Value> {
    let tx = c.unchecked_transaction().map_err(io::Error::other)?;
    let changed = tx
        .execute(
            "UPDATE jobs SET state='interrupted', lease_owner=NULL, lease_expires_ms=NULL,
               heartbeat_ms=NULL, progress=NULL, holds_keys=0, worker_stop_verified=1
             WHERE id=?1 AND state='running'",
            [id],
        )
        .map_err(io::Error::other)?;
    let released = if changed == 1 {
        0
    } else {
        tx.execute(
            "UPDATE jobs SET holds_keys=0, worker_stop_verified=1
             WHERE id=?1 AND state='interrupted'",
            [id],
        )
        .map_err(io::Error::other)?
    };
    if changed != 1 && released != 1 {
        return Err(invalid(
            "no running job, or interrupted job holding keys, to mark as stopped",
        ));
    }
    log_event(
        &tx,
        id,
        "worker_stopped",
        None,
        None,
        Some("operator verified the owned worker exited"),
    )?;
    tx.commit().map_err(io::Error::other)?;
    Ok(json!({
        "id":id,"state":"interrupted","worker_stop_verified":true,
        "keys_released":released == 1
    }))
}

/// Recording that the effect of an interrupted job was checked by the operator. Together
/// with a verified worker stop this is what re-opens the explicit `retry` path.
fn effect_verified(c: &Connection, id: &str) -> io::Result<Value> {
    let tx = c.unchecked_transaction().map_err(io::Error::other)?;
    let changed = tx
        .execute(
            "UPDATE jobs SET effect_verified=1
             WHERE id=?1 AND state='interrupted' AND worker_stop_verified=1 AND effectful=1",
            [id],
        )
        .map_err(io::Error::other)?;
    if changed != 1 {
        return Err(invalid(
            "effect verification requires an interrupted effectful job whose worker stop is verified",
        ));
    }
    log_event(
        &tx,
        id,
        "effect_verified",
        None,
        None,
        Some("operator recorded the post-run effect check"),
    )?;
    tx.commit().map_err(io::Error::other)?;
    Ok(json!({"id":id,"effect_verified":true}))
}

/// Terminal cancellation. A queued job is cancelled directly; a job whose worker died may
/// only be cancelled after the worker stop was verified, so the framework never claims a
/// cancellation completed while the old worker might still be acting.
fn cancel_terminal(c: &Connection, id: &str) -> io::Result<Value> {
    let tx = c.unchecked_transaction().map_err(io::Error::other)?;
    let queued = tx
        .execute(
            "UPDATE jobs SET state='cancelled' WHERE id=?1 AND state='queued'",
            [id],
        )
        .map_err(io::Error::other)?;
    if queued == 1 {
        log_event(
            &tx,
            id,
            "cancelled",
            None,
            None,
            Some("cancelled while queued"),
        )?;
        tx.commit().map_err(io::Error::other)?;
        return Ok(json!({"id":id,"state":"cancelled","worker_stop_verified":false}));
    }
    let stopped = tx
        .execute(
            "UPDATE jobs SET state='cancelled'
             WHERE id=?1 AND state='interrupted' AND worker_stop_verified=1",
            [id],
        )
        .map_err(io::Error::other)?;
    if stopped != 1 {
        return Err(invalid(
            "cancel requires a queued job or an interrupted job whose worker stop is verified",
        ));
    }
    log_event(
        &tx,
        id,
        "cancelled",
        None,
        None,
        Some("cancelled after verified worker stop"),
    )?;
    tx.commit().map_err(io::Error::other)?;
    Ok(json!({"id":id,"state":"cancelled","worker_stop_verified":true}))
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
    run_one_leased(c, "local", DEFAULT_LEASE_MS, 0, None, None, evaluate)
}

fn run_one_leased<F>(
    c: &mut Connection,
    worker: &str,
    lease_ms: u64,
    heartbeat_ms: u64,
    deadline_ms: Option<u64>,
    only_job: Option<&str>,
    evaluate: F,
) -> io::Result<Value>
where
    F: FnOnce(&Connection, Value, usize) -> io::Result<Value>,
{
    if worker.is_empty()
        || worker.len() > 64
        || !worker.bytes().all(|b| {
            b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b':' | b'/' | b'@')
        })
    {
        return Err(invalid("invalid worker identity"));
    }
    if !(1_000..=MAX_LEASE_MS).contains(&lease_ms) {
        return Err(invalid("lease must be within 1000..=3600000 ms"));
    }
    if heartbeat_ms != 0 && !(250..=MAX_LEASE_MS).contains(&heartbeat_ms) {
        return Err(invalid(
            "heartbeat must be within 250..=3600000 ms or 0 to disable",
        ));
    }
    if let Some(deadline) = deadline_ms {
        if !(1_000..=MAX_LEASE_MS).contains(&deadline) {
            return Err(invalid("deadline must be within 1000..=3600000 ms"));
        }
    }
    let started = Instant::now();
    let started_utc_ms = utc_millis()?;
    let Some(claim) = claim(c, worker, lease_ms, only_job)? else {
        return Ok(json!({"state":"idle"}));
    };
    let id = claim.id;
    let token = claim.token;
    let workers = claim.workers;
    let expected = claim.expected;
    let input = claim.input;
    // A cancellation requested after claim but before evaluation is honoured without work.
    // The whole handback is one transaction: state, lease release, reservation release and
    // the evidence row. It only applies to the current owner while the lease is still live,
    // so a stale worker cannot use a leftover cancellation flag to abandon someone else's job.
    let now = utc_millis()? as i64;
    let changed = c
        .execute(
            "UPDATE jobs SET state='cancelled',lease_owner=NULL,lease_expires_ms=NULL,
               heartbeat_ms=NULL,progress=NULL,holds_keys=0
             WHERE id=?1 AND state='running' AND lease_owner=?2 AND lease_token=?3
               AND cancel_requested=1 AND lease_expires_ms IS NOT NULL
               AND lease_expires_ms > ?4",
            params![id, worker, token, now],
        )
        .map_err(io::Error::other)?;
    if changed == 1 {
        log_event(
            c,
            &id,
            "cancelled",
            Some(worker),
            Some(token),
            Some("before evaluation"),
        )?;
        return Ok(json!({"id":id,"state":"cancelled","lease_token":token}));
    }
    let pending: bool = c
        .query_row(
            "SELECT cancel_requested=1 FROM jobs WHERE id=?1 AND state='running'
               AND lease_owner=?2 AND lease_token=?3",
            params![id, worker, token],
            |r| r.get(0),
        )
        .optional()
        .map_err(io::Error::other)?
        .unwrap_or(false);
    if pending {
        // The lease already expired while the cancellation was pending: the worker stops
        // without publishing anything, and reconciliation owns the recovery.
        return Err(invalid(
            "CANCELLED: cancellation is pending on an expired lease; stopping without a result",
        ));
    }
    // Independent heartbeat: a dedicated thread renews the lease and reports progress
    // while the evaluation runs. It stops when the job reaches a terminal state, and a
    // pending cancellation is recorded in the progress trail instead of being ignored.
    let heartbeat_stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let heartbeat_handle = if heartbeat_ms != 0 {
        let db_path = c.path().unwrap_or("").to_owned();
        let stop = heartbeat_stop.clone();
        let job_id = id.clone();
        let owner = worker.to_owned();
        let interval = heartbeat_ms;
        Some(std::thread::spawn(move || {
            let mut note = "running";
            loop {
                if stop.load(std::sync::atomic::Ordering::SeqCst) {
                    break;
                }
                std::thread::sleep(Duration::from_millis(interval.min(1_000)));
                if stop.load(std::sync::atomic::Ordering::SeqCst) {
                    break;
                }
                let Ok(conn) = open(Path::new(&db_path)) else {
                    break;
                };
                let cancelled: bool = conn
                    .query_row(
                        "SELECT cancel_requested=1 FROM jobs WHERE id=?1",
                        [&job_id],
                        |r| r.get(0),
                    )
                    .unwrap_or(false);
                if cancelled {
                    note = "cancellation pending; owner must stop";
                }
                let _ = heartbeat(&conn, &job_id, &owner, token, 30_000);
                let _ = conn.execute(
                    "UPDATE jobs SET progress=?2 WHERE id=?1 AND state='running'",
                    params![
                        job_id,
                        json!({"at_utc_ms":utc_millis().ok(),"note":note}).to_string()
                    ],
                );
            }
        }))
    } else {
        None
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
    let (mut state, mut report) = match result {
        Ok(report) if report["passed"] == true => ("completed", report),
        Ok(report) => ("failed", report),
        Err(e) => ("failed", json!({"error":e.to_string()})),
    };
    // A monotonic deadline never trusts wall-clock adjustments.
    if state == "completed" {
        if let Some(deadline) = deadline_ms {
            if started.elapsed().as_millis() as u64 > deadline {
                state = "failed";
                report = json!({
                    "error":"DEADLINE_EXCEEDED",
                    "deadline_ms":deadline,
                    "duration_ms":started.elapsed().as_millis() as u64
                });
            }
        }
    }
    report["queue_provenance"] = json!({"schema":1,"producer":"nsre","producer_version":env!("CARGO_PKG_VERSION"),"input_sha256":expected,"dependencies":dependencies});
    report["execution"] = json!({
        "schema":1,"execution_id":id,"state":state,
        "verification":"UNVERIFIED","producer_version":env!("CARGO_PKG_VERSION"),
        "worker":worker,"lease_token":token,"attempt_lease_ms":lease_ms,
        "started_utc_ms":started_utc_ms,"finished_utc_ms":utc_millis().ok(),
        "duration_ms":started.elapsed().as_millis() as u64,
        "duration_clock":"monotonic","resource_usage":{"measured_by_engine":false},
        "automatic_retry":false
    });
    let body = serde_json::to_string(&report).map_err(io::Error::other)?;
    heartbeat_stop.store(true, std::sync::atomic::Ordering::SeqCst);
    if let Some(handle) = heartbeat_handle {
        let _ = handle.join();
    }
    // Fencing: only the current lease holder of a lease that is still valid may publish a
    // terminal result, even if reconciliation has not run yet. The status change and its
    // evidence row commit together.
    let now = utc_millis()? as i64;
    let tx = c
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(io::Error::other)?;
    let changed = tx
        .execute(
            "UPDATE jobs SET state=?1,result=?2,result_sha256=?3,
               lease_owner=NULL,lease_expires_ms=NULL,heartbeat_ms=NULL,progress=NULL,holds_keys=0
             WHERE id=?4 AND state='running' AND lease_owner=?7 AND lease_token=?5
               AND cancel_requested=0 AND lease_expires_ms IS NOT NULL AND lease_expires_ms > ?6",
            params![state, body, digest(&body), id, token, now, worker],
        )
        .map_err(io::Error::other)?;
    if changed != 1 {
        let (cancelled, expired): (bool, bool) = tx
            .query_row(
                "SELECT cancel_requested=1,
                        (lease_expires_ms IS NULL OR lease_expires_ms <= ?2)
                 FROM jobs WHERE id=?1 AND state='running' AND lease_token=?3",
                params![id, now, token],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()
            .map_err(io::Error::other)?
            .unwrap_or((false, true));
        if cancelled {
            tx.execute(
                "UPDATE jobs SET state='cancelled',lease_owner=NULL,lease_expires_ms=NULL,
                   heartbeat_ms=NULL,progress=NULL,holds_keys=0
                 WHERE id=?1 AND state='running' AND lease_owner=?3 AND lease_token=?2
                   AND cancel_requested=1",
                params![id, token, worker],
            )
            .map_err(io::Error::other)?;
            log_event(&tx, &id, "cancelled", Some(worker), Some(token), None)?;
            tx.commit().map_err(io::Error::other)?;
            return Err(invalid(
                "CANCELLED: owner honoured a running cancellation request; result discarded",
            ));
        }
        let detail = if expired {
            "lease expired before commit"
        } else {
            "lease lost before commit"
        };
        log_event(
            &tx,
            &id,
            "result_rejected",
            Some(worker),
            Some(token),
            Some(detail),
        )?;
        tx.commit().map_err(io::Error::other)?;
        return Err(invalid(
            "LEASE_LOST: stale worker result rejected; the lease expired or moved to another owner",
        ));
    }
    log_event(&tx, &id, "finished", Some(worker), Some(token), Some(state))?;
    tx.commit().map_err(io::Error::other)?;
    Ok(json!({"id":id,"state":state,"lease_token":token}))
}

fn utc_millis() -> io::Result<u64> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(io::Error::other)?;
    u64::try_from(duration.as_millis()).map_err(io::Error::other)
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

struct LeaseRow {
    owner: Option<String>,
    token: i64,
    expires: Option<i64>,
    heartbeat: Option<i64>,
    attempt: i64,
    cancel: i64,
    progress: Option<String>,
    stop_verified: i64,
    holds_keys: i64,
    pid: Option<i64>,
    created_ms: Option<i64>,
    effectful: i64,
    effect_verified: i64,
}

fn status(c: &Connection, id: &str) -> io::Result<Value> {
    let version: i64 = c
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .map_err(io::Error::other)?;
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
    let has_links: bool = c
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_schema WHERE type='table' AND name='retry_links')",
            [],
            |r| r.get(0),
        )
        .map_err(io::Error::other)?;
    let retry_of: Option<String> = if has_links {
        c.query_row("SELECT parent FROM retry_links WHERE job=?1", [id], |r| {
            r.get(0)
        })
        .optional()
        .map_err(io::Error::other)?
    } else {
        None
    };
    let lease = if version >= 3 {
        let row: LeaseRow = c
            .query_row(
                "SELECT lease_owner,lease_token,lease_expires_ms,heartbeat_ms,attempt,cancel_requested,progress,worker_stop_verified,holds_keys,worker_pid,worker_created_ms,effectful,effect_verified FROM jobs WHERE id=?1",
                [id],
                |r| {
                    Ok(LeaseRow {
                        owner: r.get(0)?,
                        token: r.get(1)?,
                        expires: r.get(2)?,
                        heartbeat: r.get(3)?,
                        attempt: r.get(4)?,
                        cancel: r.get(5)?,
                        progress: r.get(6)?,
                        stop_verified: r.get(7)?,
                        holds_keys: r.get(8)?,
                        pid: r.get(9)?,
                        created_ms: r.get(10)?,
                        effectful: r.get(11)?,
                        effect_verified: r.get(12)?,
                    })
                },
            )
            .map_err(io::Error::other)?;
        json!({
            "owner":row.owner,"token":row.token,"expires_utc_ms":row.expires,
            "heartbeat_utc_ms":row.heartbeat,"attempt":row.attempt,
            "cancel_requested":row.cancel == 1,"progress":row.progress,
            "worker_stop_verified":row.stop_verified == 1,"exclusive_keys_held":row.holds_keys == 1,
            "worker_pid":row.pid,"worker_created_ms":row.created_ms,
            "effectful":row.effectful == 1,"effect_verified":row.effect_verified == 1
        })
    } else {
        json!({"schema":"legacy","owner":null,"token":0,"cancel_requested":false})
    };
    Ok(json!({
        "id":id,"state":state,"result":report,"retry_of":retry_of,
        "input_sha256":input_hash,"result_sha256":hash,"lease":lease,
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
    let (effectful, effect_verified, stop_verified): (i64, i64, i64) = c
        .query_row(
            "SELECT effectful,effect_verified,worker_stop_verified FROM jobs WHERE id=?1",
            [old_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .map_err(io::Error::other)?;
    if effectful == 1 && (stop_verified == 0 || effect_verified == 0) {
        return Err(invalid(
            "retrying an effectful job requires a verified worker stop and a verified effect check",
        ));
    }
    submit_attempt(c, new_id, &input, priority, workers as usize, Some(old_id))
}

#[cfg(test)]
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
    let usage = "queue init DB | migrate DB | submit DB ID INPUT PRIORITY WORKERS --reviewed-redacted | run-one DB [--worker NAME] [--lease-ms N] | drain DB LIMIT | status DB ID | wait DB ID SECONDS | retry DB OLD_ID NEW_ID --reviewed-redacted | cancel DB ID [--running] | heartbeat DB ID WORKER TOKEN LEASE_MS | reconcile DB | effect-verified DB ID | interrupt DB ID --worker-stopped";
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
        "run-one" if args.len() >= 2 => {
            let mut worker = "local".to_owned();
            let mut lease_ms = DEFAULT_LEASE_MS;
            let mut heartbeat_ms = 0u64;
            let mut deadline_ms = None;
            let mut only_job = None;
            let mut rest = args[2..].iter();
            while let Some(flag) = rest.next() {
                match flag.to_str() {
                    Some("--worker") => {
                        worker = rest
                            .next()
                            .and_then(|v| v.to_str())
                            .ok_or_else(|| invalid("worker name required"))?
                            .to_owned();
                    }
                    Some("--lease-ms") => {
                        lease_ms = rest
                            .next()
                            .and_then(|v| v.to_str())
                            .and_then(|v| v.parse().ok())
                            .ok_or_else(|| invalid("lease milliseconds required"))?;
                    }
                    Some("--heartbeat-ms") => {
                        heartbeat_ms = rest
                            .next()
                            .and_then(|v| v.to_str())
                            .and_then(|v| v.parse().ok())
                            .ok_or_else(|| invalid("heartbeat milliseconds required"))?;
                    }
                    Some("--deadline-ms") => {
                        deadline_ms = Some(
                            rest.next()
                                .and_then(|v| v.to_str())
                                .and_then(|v| v.parse().ok())
                                .ok_or_else(|| invalid("deadline milliseconds required"))?,
                        );
                    }
                    Some("--job") => {
                        only_job = Some(
                            rest.next()
                                .and_then(|v| v.to_str())
                                .ok_or_else(|| invalid("job id required"))?,
                        );
                    }
                    _ => return Err(invalid(usage)),
                }
            }
            let report = run_one_leased(
                &mut c,
                &worker,
                lease_ms,
                heartbeat_ms,
                deadline_ms,
                only_job,
                |c, v, workers| {
                    if v["schema"] == 2 {
                        validate_source_scope(c, &v)?;
                        crate::collect::evaluate_job(&v, workers)
                    } else {
                        crate::batch::evaluate(&v, workers)
                    }
                },
            )?;
            println!("{report}");
            return Ok(report["state"] != "failed" && report["state"] != "cancelled");
        }
        "heartbeat" if args.len() == 6 => {
            let worker = args[3]
                .to_str()
                .ok_or_else(|| invalid("worker name required"))?;
            let token = args[4]
                .to_str()
                .and_then(|v| v.parse::<i64>().ok())
                .ok_or_else(|| invalid("lease token required"))?;
            let lease_ms = args[5]
                .to_str()
                .and_then(|v| v.parse::<u64>().ok())
                .ok_or_else(|| invalid("lease milliseconds required"))?;
            println!("{}", heartbeat(&c, id()?, worker, token, lease_ms)?);
        }
        "reconcile" if args.len() == 2 => println!("{}", reconcile(&c)?),
        "effect-verified" if args.len() == 3 => println!("{}", effect_verified(&c, id()?)?),
        "cancel" if args.len() == 4 && args[3] == "--running" => {
            println!("{}", cancel_running(&c, id()?)?)
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
        "cancel" if args.len() == 3 => println!("{}", cancel_terminal(&c, id()?)?),
        "interrupt" if args.len() == 4 && args[3] == "--worker-stopped" => {
            println!("{}", worker_stopped(&c, id()?)?)
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
        assert_eq!(parent["result"]["execution"]["execution_id"], "parent");
        assert_eq!(parent["result"]["execution"]["verification"], "UNVERIFIED");
        assert!(
            parent["result"]["execution"]["started_utc_ms"]
                .as_u64()
                .unwrap()
                > 0
        );
        assert!(parent["result"]["execution"]["duration_ms"].is_u64());
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
        assert_eq!(
            claim(&mut a, "worker-a", 1000, None).unwrap().unwrap().id,
            "writer-a"
        );
        let mut b = open(&f.db()).unwrap();
        assert_eq!(
            claim(&mut b, "worker-b", 1000, None).unwrap().unwrap().id,
            "other"
        );
        assert!(claim(&mut b, "worker-b", 1000, None).unwrap().is_none());
        transition(&a, "writer-a", "running", "interrupted").unwrap();
        assert!(claim(&mut b, "worker-b", 1000, None).unwrap().is_none());
        worker_stopped(&a, "writer-a").unwrap();
        assert_eq!(
            claim(&mut b, "worker-b", 1000, None).unwrap().unwrap().id,
            "writer-b"
        );
        assert!(claim(&mut a, "worker-a", 1000, None).unwrap().is_none());
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
                    claim(&mut c, "racer", 1000, None).unwrap()
                })
            })
            .collect();
        let claims: Vec<_> = handles
            .into_iter()
            .filter_map(|h| h.join().unwrap())
            .collect();
        assert_eq!(claims.len(), 1);
        let mut c = open(&f.db()).unwrap();
        transition(&c, &claims[0].id, "running", "interrupted").unwrap();
        assert!(claim(&mut c, "racer", 1000, None).unwrap().is_none());
        worker_stopped(&c, &claims[0].id).unwrap();
        assert!(claim(&mut c, "racer", 1000, None).unwrap().is_some());
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
        // Writing and running require the current schema; migration is the explicit gate.
        assert!(submit(&c, "old", input, 50, 1).is_err());
        assert!(claim(&mut c, "worker-a", 1000, None).is_err());
        migrate(&mut c).unwrap();
        migrate(&mut c).unwrap();
        assert_eq!(
            c.query_row("PRAGMA user_version", [], |r| r.get::<_, i64>(0))
                .unwrap(),
            5
        );
        submit(&c, "old", input, 50, 1).unwrap();
        submit(&c, "new", &manifest(&["old"], &[], true), 100, 1).unwrap();
        assert_eq!(run_one(&mut c).unwrap()["id"], "old");
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
    fn heartbeat_extends_only_the_current_lease_owner() {
        let f = Fixture::new();
        init(&f.db()).unwrap();
        let mut c = open(&f.db()).unwrap();
        submit(&c, "job", &manifest(&[], &[], true), 10, 1).unwrap();
        let claim = claim(&mut c, "worker-a", 1000, None).unwrap().unwrap();
        assert_eq!(claim.token, 1);
        let before = status(&c, "job").unwrap();
        assert_eq!(before["state"], "running");
        assert_eq!(before["lease"]["owner"], "worker-a");
        assert_eq!(before["lease"]["attempt"], 1);
        assert!(heartbeat(&c, "job", "worker-b", claim.token, 5000).is_err());
        assert!(heartbeat(&c, "job", "worker-a", claim.token + 1, 5000).is_err());
        let extended = heartbeat(&c, "job", "worker-a", claim.token, 5000).unwrap();
        assert_eq!(extended["lease_token"], 1);
        assert!(
            extended["lease_expires_utc_ms"].as_i64().unwrap()
                > before["lease"]["expires_utc_ms"].as_i64().unwrap()
        );
    }

    #[test]
    fn expired_lease_is_reconciled_to_interrupted_not_requeued() {
        let f = Fixture::new();
        init(&f.db()).unwrap();
        let mut c = open(&f.db()).unwrap();
        submit(&c, "job", &manifest(&[], &[], true), 10, 1).unwrap();
        let first = claim(&mut c, "worker-a", 1000, None).unwrap().unwrap();
        c.execute("UPDATE jobs SET lease_expires_ms=1 WHERE id='job'", [])
            .unwrap();
        let report = reconcile(&c).unwrap();
        assert_eq!(report["reconciled"][0]["id"], "job");
        assert_eq!(report["reconciled"][0]["previous_owner"], "worker-a");
        assert_eq!(report["unverified_effect"], true);
        assert_eq!(status(&c, "job").unwrap()["state"], "interrupted");
        // Reconciliation never silently replays the job.
        assert!(claim(&mut c, "worker-b", 1000, None).unwrap().is_none());
        // The interrupted job can only continue through an explicit, evidenced retry.
        retry(&c, "job", "job-retry").unwrap();
        let next = claim(&mut c, "worker-b", 1000, None).unwrap().unwrap();
        assert_eq!(next.id, "job-retry");
        assert_eq!(next.token, 1, "each job fences its own lease tokens");
        let events: i64 = c
            .query_row(
                "SELECT count(*) FROM lease_events WHERE job='job'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert!(events >= 2, "claim and expiry leave a durable trail");
        assert_eq!(first.token, 1);
    }

    #[test]
    fn stale_worker_result_is_rejected_after_lease_loss() {
        let f = Fixture::new();
        init(&f.db()).unwrap();
        let mut c = open(&f.db()).unwrap();
        submit(&c, "job", &manifest(&[], &[], true), 10, 1).unwrap();
        // The evaluator loses its lease mid-flight (supervisor reconciliation).
        let error = run_one_leased(&mut c, "worker-a", 1000, 0, None, None, |c, _, _| {
            c.execute("UPDATE jobs SET lease_expires_ms=1 WHERE id='job'", [])
                .unwrap();
            reconcile(c).unwrap();
            Ok(json!({"passed":true}))
        })
        .unwrap_err();
        assert!(error.to_string().contains("LEASE_LOST"), "{error}");
        let after = status(&c, "job").unwrap();
        assert_eq!(after["state"], "interrupted");
        assert!(
            after["result_sha256"].is_null(),
            "no result may be published"
        );
    }

    #[test]
    fn running_cancellation_discards_the_result_and_releases_the_lease() {
        let f = Fixture::new();
        init(&f.db()).unwrap();
        let mut c = open(&f.db()).unwrap();
        submit(&c, "job", &manifest(&[], &[], true), 10, 1).unwrap();
        let error = run_one_leased(&mut c, "worker-a", 1000, 0, None, None, |c, _, _| {
            cancel_running(c, "job").unwrap();
            Ok(json!({"passed":true}))
        })
        .unwrap_err();
        assert!(error.to_string().contains("CANCELLED"), "{error}");
        let after = status(&c, "job").unwrap();
        assert_eq!(after["state"], "cancelled");
        assert_eq!(after["lease"]["cancel_requested"], true);
        assert!(after["lease"]["owner"].is_null());
        assert!(after["result_sha256"].is_null());
    }

    #[test]
    fn schema_two_migration_adds_leases_without_losing_jobs() {
        let f = Fixture::new();
        let c = Connection::open(f.db()).unwrap();
        c.execute_batch(
            "PRAGMA application_id=1314083146; PRAGMA user_version=2;
             CREATE TABLE jobs(id TEXT PRIMARY KEY,input TEXT NOT NULL,input_sha256 TEXT NOT NULL,
               priority INTEGER NOT NULL,workers INTEGER NOT NULL,state TEXT NOT NULL,
               result TEXT,result_sha256 TEXT,skipped_claims INTEGER NOT NULL DEFAULT 0);
             CREATE TABLE dependencies(job TEXT NOT NULL REFERENCES jobs(id), dependency TEXT NOT NULL REFERENCES jobs(id), PRIMARY KEY(job,dependency));
             CREATE TABLE exclusive_keys(job TEXT NOT NULL REFERENCES jobs(id), key TEXT NOT NULL, PRIMARY KEY(job,key));",
        )
        .unwrap();
        drop(c);
        let mut c = open(&f.db()).unwrap();
        let input = manifest(&[], &[], true);
        // v2 databases must migrate before any write, and the migration keeps their rows.
        assert!(submit(&c, "legacy", &input, 5, 1).is_err());
        migrate(&mut c).unwrap();
        let version: i64 = c
            .query_row("PRAGMA user_version", [], |r| r.get(0))
            .unwrap();
        assert_eq!(version, 5);
        submit(&c, "legacy", &input, 5, 1).unwrap();
        let legacy = status(&c, "legacy").unwrap();
        assert_eq!(legacy["input_sha256"], digest(&input));
        assert_eq!(legacy["lease"]["token"], 0);
        let claimed = claim(&mut c, "worker-a", 1000, None).unwrap().unwrap();
        assert_eq!(claimed.id, "legacy");
        assert_eq!(claimed.token, 1);
        // Migration is idempotent once the worker released the lease, and the trail stays usable.
        c.execute("UPDATE jobs SET lease_expires_ms=1 WHERE id='legacy'", [])
            .unwrap();
        assert_eq!(reconcile(&c).unwrap()["reconciled"][0]["id"], "legacy");
        drop(c);
        let mut c = open(&f.db()).unwrap();
        migrate(&mut c).unwrap();
        assert_eq!(status(&c, "legacy").unwrap()["state"], "interrupted");
    }

    #[test]
    fn heartbeat_rejects_an_already_expired_lease() {
        let f = Fixture::new();
        init(&f.db()).unwrap();
        let mut c = open(&f.db()).unwrap();
        submit(&c, "job", &manifest(&[], &[], true), 10, 1).unwrap();
        let first = claim(&mut c, "worker-a", 1000, None).unwrap().unwrap();
        c.execute("UPDATE jobs SET lease_expires_ms=1 WHERE id='job'", [])
            .unwrap();
        // Reconciliation has not run, yet the lease is already void.
        let error = heartbeat(&c, "job", "worker-a", first.token, 5000).unwrap_err();
        assert!(error.to_string().contains("LEASE_LOST"), "{error}");
        let lease = status(&c, "job").unwrap();
        assert_eq!(lease["state"], "running");
        assert_eq!(lease["lease"]["expires_utc_ms"], 1);
    }

    #[test]
    fn terminal_result_is_rejected_when_the_lease_expired_without_reconciliation() {
        let f = Fixture::new();
        init(&f.db()).unwrap();
        let mut c = open(&f.db()).unwrap();
        submit(&c, "job", &manifest(&[], &[], true), 10, 1).unwrap();
        let error = run_one_leased(&mut c, "worker-a", 1000, 0, None, None, |c, _, _| {
            c.execute("UPDATE jobs SET lease_expires_ms=1 WHERE id='job'", [])
                .unwrap();
            Ok(json!({"passed":true}))
        })
        .unwrap_err();
        assert!(error.to_string().contains("LEASE_LOST"), "{error}");
        let after = status(&c, "job").unwrap();
        assert_eq!(
            after["state"], "running",
            "an expired lease publishes nothing"
        );
        assert!(after["result_sha256"].is_null());
    }

    #[test]
    fn reconcile_leaves_a_renewed_lease_running() {
        let f = Fixture::new();
        init(&f.db()).unwrap();
        let mut c = open(&f.db()).unwrap();
        submit(&c, "job", &manifest(&[], &[], true), 10, 1).unwrap();
        let first = claim(&mut c, "worker-a", 1000, None).unwrap().unwrap();
        // Renew before reconciliation scans; reconciliation must not touch a live lease.
        let extended = heartbeat(&c, "job", "worker-a", first.token, 60_000).unwrap();
        let report = reconcile(&c).unwrap();
        assert_eq!(report["reconciled"].as_array().unwrap().len(), 0);
        let after = status(&c, "job").unwrap();
        assert_eq!(after["state"], "running");
        assert_eq!(after["lease"]["owner"], "worker-a");
        assert_eq!(
            after["lease"]["expires_utc_ms"],
            extended["lease_expires_utc_ms"]
        );
    }

    #[test]
    fn reconcile_and_heartbeat_racing_never_leave_a_half_interrupted_lease() {
        let f = Fixture::new();
        init(&f.db()).unwrap();
        let mut c = open(&f.db()).unwrap();
        for i in 0..4 {
            submit(&c, &format!("job-{i}"), &manifest(&[], &[], true), 10, 1).unwrap();
            claim(&mut c, &format!("worker-{i}"), 2000, None).unwrap();
        }
        drop(c);
        let db = f.db();
        // Deterministic start: the barrier releases both sides together, and the assertion
        // below is what must hold in every interleaving, not any particular timing.
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
        let reconciler = {
            let db = db.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                let c = open(&db).unwrap();
                barrier.wait();
                for _ in 0..40 {
                    reconcile(&c).unwrap();
                    std::thread::sleep(std::time::Duration::from_millis(2));
                }
            })
        };
        let heartbeat_worker = {
            let db = db.clone();
            std::thread::spawn(move || {
                let c = open(&db).unwrap();
                barrier.wait();
                for i in 0..40 {
                    let job = format!("job-{}", i % 4);
                    let owner = format!("worker-{}", i % 4);
                    let _ = heartbeat(&c, &job, &owner, 1, 2000);
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
            })
        };
        reconciler.join().unwrap();
        heartbeat_worker.join().unwrap();
        let c = open(&db).unwrap();
        for i in 0..4 {
            let row: (String, Option<i64>) = c
                .query_row(
                    "SELECT state,lease_expires_ms FROM jobs WHERE id=?1",
                    [format!("job-{i}")],
                    |r| Ok((r.get(0)?, r.get(1)?)),
                )
                .unwrap();
            match row.0.as_str() {
                "running" => assert!(row.1.unwrap() > 0, "a running job keeps a deadline"),
                "interrupted" => {
                    assert!(row.1.is_none(), "an interrupted job has no live deadline")
                }
                other => panic!("unexpected state {other}"),
            }
        }
    }

    #[test]
    fn cancellation_of_a_dead_worker_needs_a_verified_stop() {
        let f = Fixture::new();
        init(&f.db()).unwrap();
        let mut c = open(&f.db()).unwrap();
        submit(&c, "job", &manifest(&[], &[], true), 10, 1).unwrap();
        claim(&mut c, "worker-a", 1000, None).unwrap();
        cancel_running(&c, "job").unwrap();
        c.execute("UPDATE jobs SET lease_expires_ms=1 WHERE id='job'", [])
            .unwrap();
        let report = reconcile(&c).unwrap();
        assert_eq!(report["reconciled"][0]["cancel_requested"], true);
        assert_eq!(report["reconciled"][0]["worker_stop_verified"], false);
        let kind: String = c
            .query_row(
                "SELECT kind FROM lease_events WHERE job='job' ORDER BY rowid DESC LIMIT 1",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(kind, "cancel_unconfirmed");
        // Cancellation is not final while the old worker might still act.
        let error = cancel_terminal(&c, "job").unwrap_err();
        assert!(
            error.to_string().contains("worker stop is verified"),
            "{error}"
        );
        assert_eq!(status(&c, "job").unwrap()["state"], "interrupted");
        worker_stopped(&c, "job").unwrap();
        assert_eq!(cancel_terminal(&c, "job").unwrap()["state"], "cancelled");
        assert_eq!(status(&c, "job").unwrap()["state"], "cancelled");
    }

    #[test]
    fn exclusive_keys_stay_held_until_the_old_worker_is_verified_stopped() {
        let f = Fixture::new();
        init(&f.db()).unwrap();
        let mut c = open(&f.db()).unwrap();
        submit(&c, "effect", &manifest(&[], &["desktop"], true), 100, 1).unwrap();
        submit(&c, "rival", &manifest(&[], &["desktop"], true), 0, 1).unwrap();
        let first = claim(&mut c, "worker-a", 1000, None).unwrap().unwrap();
        assert_eq!(first.id, "effect");
        assert_eq!(
            status(&c, "effect").unwrap()["lease"]["exclusive_keys_held"],
            true
        );
        c.execute("UPDATE jobs SET lease_expires_ms=1 WHERE id='effect'", [])
            .unwrap();
        reconcile(&c).unwrap();
        // The effect may still be running: the rival must not acquire the same resource.
        assert!(claim(&mut c, "worker-b", 1000, None).unwrap().is_none());
        worker_stopped(&c, "effect").unwrap();
        let rival = claim(&mut c, "worker-b", 1000, None).unwrap().unwrap();
        assert_eq!(rival.id, "rival");
    }

    #[test]
    fn a_missing_evidence_trail_rolls_back_the_state_change() {
        let f = Fixture::new();
        init(&f.db()).unwrap();
        let mut c = open(&f.db()).unwrap();
        submit(&c, "job", &manifest(&[], &[], true), 10, 1).unwrap();
        c.execute_batch("ALTER TABLE lease_events RENAME TO lease_events_off")
            .unwrap();
        assert!(
            !has_table(&c, "lease_events").unwrap(),
            "rename did not take effect"
        );
        let error = run_one(&mut c).unwrap_err();
        assert!(
            error.to_string().contains("durable evidence"),
            "unexpected error: {error}"
        );
        let after = status(&c, "job").unwrap();
        assert_eq!(
            after["state"], "queued",
            "a claim whose evidence cannot be written is rolled back entirely"
        );
        assert!(after["result_sha256"].is_null());
    }

    #[test]
    fn claim_records_the_claim_instant_as_the_heartbeat() {
        let f = Fixture::new();
        init(&f.db()).unwrap();
        let mut c = open(&f.db()).unwrap();
        submit(&c, "job", &manifest(&[], &[], true), 10, 1).unwrap();
        claim(&mut c, "worker-a", 60_000, None).unwrap();
        let report = status(&c, "job").unwrap();
        let heartbeat = report["lease"]["heartbeat_utc_ms"].as_i64().unwrap();
        let expires = report["lease"]["expires_utc_ms"].as_i64().unwrap();
        assert_eq!(
            expires - heartbeat,
            60_000,
            "heartbeat is the claim instant, not the deadline"
        );
    }

    #[test]
    fn effectful_jobs_need_verified_stop_and_effect_before_retry() {
        let f = Fixture::new();
        init(&f.db()).unwrap();
        let mut c = open(&f.db()).unwrap();
        submit(&c, "effect", &manifest(&[], &["gpu"], true), 10, 1).unwrap();
        claim(&mut c, "worker-a", 1000, None).unwrap();
        // A verified stop is required before the effect itself can be signed off.
        worker_stopped(&c, "effect").unwrap();
        // The stop is verified, but the effect itself was never checked.
        let error = retry(&c, "effect", "effect-retry").unwrap_err();
        assert!(error.to_string().contains("verified effect"), "{error}");
        effect_verified(&c, "effect").unwrap();
        retry(&c, "effect", "effect-retry").unwrap();
        assert_eq!(status(&c, "effect-retry").unwrap()["retry_of"], "effect");
        assert_eq!(
            claim(&mut c, "worker-b", 1000, None).unwrap().unwrap().id,
            "effect-retry"
        );
    }

    #[test]
    fn evidence_failure_rolls_back_heartbeat_reconcile_and_cancellation() {
        let f = Fixture::new();
        init(&f.db()).unwrap();
        let mut c = open(&f.db()).unwrap();
        submit(&c, "job", &manifest(&[], &[], true), 10, 1).unwrap();
        claim(&mut c, "worker-a", 60_000, None).unwrap();
        // Force expiry so reconcile actually takes the write path with its evidence row.
        c.execute("UPDATE jobs SET lease_expires_ms=1 WHERE id='job'", [])
            .unwrap();
        c.execute_batch("ALTER TABLE lease_events RENAME TO lease_events_off")
            .unwrap();
        assert!(heartbeat(&c, "job", "worker-a", 1, 60_000).is_err());
        assert!(cancel_running(&c, "job").is_err());
        assert!(reconcile(&c).is_err());
        let after = status(&c, "job").unwrap();
        assert_eq!(after["state"], "running", "no partial lease change");
        assert_eq!(after["lease"]["cancel_requested"], false);
        c.execute_batch("ALTER TABLE lease_events_off RENAME TO lease_events")
            .unwrap();
        // With the trail restored the same operations succeed.
        assert_eq!(reconcile(&c).unwrap()["reconciled"][0]["id"], "job");
        assert_eq!(status(&c, "job").unwrap()["state"], "interrupted");
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
        drop(c);
        let mut c = open(&db).unwrap();
        assert_eq!(status(&c, "retry").unwrap()["retry_of"], "crashed");
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
