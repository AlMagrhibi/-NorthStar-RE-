use serde_json::{json, Value};
use std::{
    fs,
    path::PathBuf,
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};
static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "nsre-framework-{}-{}-{}",
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
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn cmd(t: &Temp, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_nsre"))
        .args(args)
        .arg("--project")
        .arg(&t.0)
        .output()
        .unwrap()
}
fn result(o: std::process::Output) -> Value {
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    serde_json::from_slice(&o.stdout).unwrap()
}
#[test]
fn init_preserves_and_isolates() {
    let a = Temp::new();
    let b = Temp::new();
    assert_eq!(
        result(cmd(&a, &["init"]))["created_docs"]
            .as_array()
            .unwrap()
            .len(),
        20
    );
    fs::write(a.0.join("docs/PROJECT.md"), "existing").unwrap();
    assert_eq!(result(cmd(&a, &["init"]))["created_docs"], json!([]));
    assert_eq!(
        fs::read_to_string(a.0.join("docs/PROJECT.md")).unwrap(),
        "existing"
    );
    result(cmd(&b, &["init"]));
    let f = a.0.join("record.json");
    fs::write(
        &f,
        json!({"session_id":"s1","next_action":"test"}).to_string(),
    )
    .unwrap();
    assert!(!cmd(&a, &["session", "--input", f.to_str().unwrap()])
        .status
        .success());
    result(cmd(
        &a,
        &[
            "session",
            "--input",
            f.to_str().unwrap(),
            "--reviewed-redacted",
        ],
    ));
    assert_eq!(result(cmd(&a, &["session"])).as_array().unwrap().len(), 1);
    assert_eq!(result(cmd(&b, &["session"])), json!([]));
}
#[test]
fn reject_cycle_and_path_escape() {
    let a = Temp::new();
    result(cmd(&a, &["init"]));
    let f = a.0.join("plan.json");
    fs::write(
        &f,
        json!({"tasks":[{"id":"a","status":"planned","depends_on":["a"],"acceptance":"test"}]})
            .to_string(),
    )
    .unwrap();
    assert!(!cmd(
        &a,
        &[
            "jobs",
            "--action",
            "validate",
            "--input",
            f.to_str().unwrap(),
            "--reviewed-redacted"
        ]
    )
    .status
    .success());
    fs::write(&f, json!({"path":"../escape"}).to_string()).unwrap();
    assert!(!cmd(
        &a,
        &[
            "hash",
            "--input",
            f.to_str().unwrap(),
            "--reviewed-redacted"
        ]
    )
    .status
    .success());
}
