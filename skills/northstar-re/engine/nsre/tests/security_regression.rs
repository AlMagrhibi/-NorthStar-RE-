use serde_json::{json, Value};
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};
static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "nsre-security-{}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&p).unwrap();
        Self(p)
    }
    fn run(&self, cmd: &str, input: Option<Value>) -> Output {
        let mut c = Command::new(env!("CARGO_BIN_EXE_nsre"));
        c.arg(cmd).arg("--project").arg(&self.0);
        if let Some(v) = input {
            let p = self.0.join("input.json");
            fs::write(&p, v.to_string()).unwrap();
            c.arg("--input").arg(p).arg("--reviewed-redacted");
        }
        c.output().unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
#[test]
fn templates_do_not_pass_quality_gate() {
    let f = Fixture::new();
    assert!(f.run("init", None).status.success());
    let gates = [
        "requirements",
        "review",
        "build",
        "tests",
        "regression",
        "evidence",
        "documentation",
        "memory",
        "checkpoint",
    ]
    .into_iter()
    .map(|k| {
        (
            k.to_owned(),
            json!({"status":"not_applicable","reason":"fixture"}),
        )
    })
    .collect::<serde_json::Map<_, _>>();
    let o = f.run("quality-gate", Some(json!({"gates":gates})));
    assert_eq!(o.status.code(), Some(2));
    let v: Value = serde_json::from_slice(&o.stdout).unwrap();
    assert_eq!(v["gate_passed"], false);
    assert!(v["blocked"]
        .as_array()
        .unwrap()
        .contains(&json!("docs_not_completed")));
}
#[test]
fn evidence_requires_matching_hash_and_checkpoint_is_non_destructive() {
    let f = Fixture::new();
    assert!(f.run("init", None).status.success());
    fs::write(f.0.join("a.txt"), b"abc").unwrap();
    assert!(!f
        .run("verify", Some(json!([{"path":"a.txt","sha256":"wrong"}])))
        .status
        .success());
    assert!(f.run("verify",Some(json!([{"path":"a.txt","sha256":"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"}]))).status.success());
    assert!(!f
        .run("checkpoint", Some(json!(["a.txt", "a.txt"])))
        .status
        .success());
    assert!(!f
        .run(
            "checkpoint",
            Some(json!([".northstar/state/session-state.json"]))
        )
        .status
        .success());
    let o = f.run("checkpoint", Some(json!(["a.txt"])));
    assert!(o.status.success());
    let v: Value = serde_json::from_slice(&o.stdout).unwrap();
    let copy =
        f.0.join(".northstar/checkpoints")
            .join(v["checkpoint"].as_str().unwrap())
            .join("files/a.txt");
    assert_eq!(fs::read(copy).unwrap(), b"abc");
    assert_eq!(fs::read(f.0.join("a.txt")).unwrap(), b"abc");
}
#[cfg(unix)]
#[test]
fn redirected_project_state_is_refused() {
    use std::os::unix::fs::symlink;
    let a = Fixture::new();
    let b = Fixture::new();
    symlink(&b.0, a.0.join(".northstar")).unwrap();
    assert!(!a.run("init", None).status.success());
    assert_eq!(fs::read_dir(&b.0).unwrap().count(), 0);
}
#[test]
fn unrelated_state_database_and_path_escape_are_refused() {
    let f = Fixture::new();
    assert!(f.run("init", None).status.success());
    let c = rusqlite::Connection::open(f.0.join(".northstar/state/framework.sqlite")).unwrap();
    c.execute("CREATE TABLE private(value TEXT)", []).unwrap();
    drop(c);
    assert!(!f
        .run(
            "session",
            Some(json!({"session_id":"s","next_action":"test"}))
        )
        .status
        .success());
    for p in ["../escape", "/absolute", "C:/drive", "a/../b", "a\\b"] {
        assert!(
            !f.run("hash", Some(json!({"path":p}))).status.success(),
            "{p}"
        );
    }
}
