//! Process-level queue fixtures. Synthetic files only; no desktop or agent claims.
use rusqlite::Connection;
use serde_json::{json, Value};
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};
struct Fixture {
    root: PathBuf,
    db: PathBuf,
}
impl Fixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "nsre queue fixture-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&root).unwrap();
        let db = root.join("jobs.sqlite");
        let f = Self { root, db };
        f.ok(&["init"]);
        f
    }
    fn run(&self, args: &[&str]) -> Output {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_nsre"));
        cmd.args(["queue", args[0]])
            .arg(&self.db)
            .args(&args[1..])
            .output()
            .unwrap()
    }
    fn ok(&self, args: &[&str]) -> Value {
        let o = self.run(args);
        assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
        serde_json::from_slice(&o.stdout).unwrap_or(Value::Null)
    }
    fn submit(&self, id: &str, mut input: Value, deps: &[&str], priority: u32) -> Output {
        let p = self.root.join("input.json");
        input["queue"] = json!({"schema":1,"depends_on":deps});
        fs::write(&p, input.to_string()).unwrap();
        self.run(&[
            "submit",
            id,
            p.to_str().unwrap(),
            &priority.to_string(),
            "2",
            "--reviewed-redacted",
        ])
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
fn batch() -> Value {
    json!({"schema":1,"tasks":[{"id":"data","input":{"operation":"json-roundtrip","data":1}}]})
}
fn good(o: Output) {
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
}
#[test]
fn queued_collection_observes_changes_and_blocks_dependents() {
    let f = Fixture::new();
    let source = f.root.join("source");
    fs::create_dir(&source).unwrap();
    fs::write(source.join("a.txt"), b"abc").unwrap();
    let expected = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
    let collection = json!({"schema":2,"kind":"collect","root":source,"manifest":{"schema":1,"files":[{"path":"a.txt","expected_sha256":expected}]}});
    good(f.submit("collect", collection.clone(), &[], 0));
    good(f.submit("after", batch(), &["collect"], 100));
    assert_eq!(f.ok(&["run-one"])["id"], "collect");
    assert_eq!(f.ok(&["run-one"])["id"], "after");
    let parent = f.ok(&["status", "collect"]);
    let child = f.ok(&["wait", "after", "0"]);
    assert_eq!(parent["result"]["results"][0]["hash_recomputed"], true);
    assert_eq!(parent["result"]["results"][0]["sha256"], expected);
    assert_eq!(
        child["result"]["queue_provenance"]["dependencies"][0]["result_sha256"],
        parent["result_sha256"]
    );
    good(f.submit("changed", collection, &[], 0));
    good(f.submit("blocked", batch(), &["changed"], 100));
    fs::write(source.join("a.txt"), b"different").unwrap();
    assert_eq!(f.run(&["run-one"]).status.code(), Some(2));
    let changed = f.ok(&["status", "changed"]);
    assert_eq!(changed["state"], "failed");
    assert_ne!(changed["result"]["results"][0]["sha256"], expected);
    assert_eq!(f.run(&["run-one"]).status.code(), Some(2));
    assert_eq!(f.ok(&["status", "blocked"])["state"], "failed");
    assert_eq!(f.run(&["wait", "blocked", "0"]).status.code(), Some(2));
    assert_eq!(fs::read(source.join("a.txt")).unwrap(), b"different");
}
#[test]
fn collector_contract_rejects_escape_and_database_in_source() {
    let f = Fixture::new();
    let source = f.root.join("source");
    fs::create_dir(&source).unwrap();
    let mut input = json!({"schema":2,"kind":"collect","root":source,"manifest":{"schema":1,"files":[{"path":"../outside"}]}});
    assert_eq!(
        f.submit("escape", input.clone(), &[], 50).status.code(),
        Some(1)
    );
    input["manifest"]["files"][0]["path"] = json!("a.txt");
    input["root"] = json!(f.root);
    assert_eq!(
        f.submit("db-in-source", input.clone(), &[], 50)
            .status
            .code(),
        Some(1)
    );
    input["root"] = json!("relative");
    assert_eq!(f.submit("relative", input, &[], 50).status.code(), Some(1));
    assert_eq!(f.ok(&["run-one"])["state"], "idle");
}
#[test]
fn concurrent_processes_claim_distinct_durable_jobs() {
    let f = Fixture::new();
    for n in 0..8 {
        good(f.submit(&format!("job-{n}"), batch(), &[], 50));
    }
    let children: Vec<_> = (0..8)
        .map(|_| {
            Command::new(env!("CARGO_BIN_EXE_nsre"))
                .args(["queue", "run-one"])
                .arg(&f.db)
                .stdout(std::process::Stdio::piped())
                .spawn()
                .unwrap()
        })
        .collect();
    let mut ids = std::collections::BTreeSet::new();
    for child in children {
        let out = child.wait_with_output().unwrap();
        assert!(out.status.success());
        let v: Value = serde_json::from_slice(&out.stdout).unwrap();
        assert!(ids.insert(v["id"].as_str().unwrap().to_owned()));
    }
    assert_eq!(ids.len(), 8);
    for id in ids {
        assert_eq!(f.ok(&["status", &id])["state"], "completed");
    }
    assert_eq!(f.ok(&["run-one"])["state"], "idle");
}
#[test]
fn corrupted_dependency_fails_before_child_execution() {
    let f = Fixture::new();
    good(f.submit("parent", batch(), &[], 0));
    good(f.submit("child", batch(), &["parent"], 100));
    f.ok(&["run-one"]);
    let c = Connection::open(&f.db).unwrap();
    c.execute("UPDATE jobs SET result='{}' WHERE id='parent'", [])
        .unwrap();
    drop(c);
    assert_eq!(f.run(&["run-one"]).status.code(), Some(2));
    assert_eq!(f.run(&["status", "parent"]).status.code(), Some(1));
    assert_eq!(f.ok(&["status", "child"])["state"], "failed");
}

#[cfg(unix)]
#[test]
fn collector_refuses_reparse_ancestor_and_rechecks_scope_at_execution() {
    use std::os::unix::fs::symlink;
    let f = Fixture::new();
    let actual = f.root.join("actual");
    let source = actual.join("source");
    fs::create_dir_all(&source).unwrap();
    fs::write(source.join("a.txt"), b"abc").unwrap();
    let alias = f.root.join("alias");
    symlink(&actual, &alias).unwrap();
    let mut input = json!({"schema":2,"kind":"collect","root":alias.join("source"),"manifest":{"schema":1,"files":[{"path":"a.txt"}]}});
    assert_eq!(
        f.submit("ancestor", input.clone(), &[], 50).status.code(),
        Some(1)
    );
    input["root"] = json!(source);
    good(f.submit("replace-root", input, &[], 50));
    let moved = actual.join("original");
    fs::rename(&source, &moved).unwrap();
    symlink(&moved, &source).unwrap();
    assert_eq!(f.run(&["run-one"]).status.code(), Some(2));
    assert_eq!(f.ok(&["status", "replace-root"])["state"], "failed");
    assert_eq!(fs::read(moved.join("a.txt")).unwrap(), b"abc");
}
