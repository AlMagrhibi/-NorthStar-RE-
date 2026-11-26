use rusqlite::Connection;
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};
static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "nsre-{}-{}-{}",
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
fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_nsre"))
        .args(args)
        .output()
        .unwrap()
}
fn okay(o: Output) -> String {
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout).unwrap()
}
#[test]
fn indexed_lifecycle() {
    let t = Temp::new();
    let root = t.0.join("input");
    fs::create_dir(&root).unwrap();
    let db = t.0.join("index.sqlite");
    let r = root.to_str().unwrap();
    let d = db.to_str().unwrap();
    fs::write(root.join("a"), b"abc").unwrap();
    fs::write(root.join("ب"), b"old").unwrap();
    let scan = |mode| {
        run(&[
            "index", "--root", r, "--db", d, "--jobs", "2", "--mode", mode,
        ])
    };
    assert!(okay(scan("strict")).contains("\"fresh\":2"));
    assert!(okay(scan("reuse-metadata")).contains("\"reused\":2"));
    fs::remove_file(root.join("a")).unwrap();
    fs::write(root.join("ب"), b"changed-longer").unwrap();
    fs::write(root.join("new"), b"new").unwrap();
    okay(scan("strict"));
    let diff = okay(run(&["diff", "--db", d, "--old", "1", "--new", "3"]));
    for s in ["added", "removed", "changed"] {
        assert!(diff.contains(s));
    }
    assert!(!run(&["diff", "--db", d, "--old", "2", "--new", "3"])
        .status
        .success());
    let export = okay(run(&["export", "--db", d, "--run", "1"]));
    assert!(export.contains("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"));
    let c = Connection::open(&db).unwrap();
    c.execute("UPDATE runs SET state='running' WHERE id=3", [])
        .unwrap();
    drop(c);
    assert!(!run(&["export", "--db", d, "--run", "3"]).status.success());
    assert!(!run(&[
        "index",
        "--root",
        r,
        "--db",
        root.join("bad.sqlite").to_str().unwrap()
    ])
    .status
    .success());
}
#[test]
fn unrelated_database_refused() {
    let t = Temp::new();
    let root = t.0.join("input");
    fs::create_dir(&root).unwrap();
    let db = t.0.join("other.sqlite");
    let c = Connection::open(&db).unwrap();
    c.execute("CREATE TABLE other(x)", []).unwrap();
    drop(c);
    assert!(!run(&[
        "index",
        "--root",
        root.to_str().unwrap(),
        "--db",
        db.to_str().unwrap()
    ])
    .status
    .success());
}

#[test]
fn agent_stdio_discovers_contract_and_reads_only_bound_queue() {
    use serde_json::{json, Value};
    use std::{io::Write, process::Stdio};
    let t = Temp::new();
    let other = Temp::new();
    fs::create_dir_all(t.0.join(".northstar/state")).unwrap();
    let db = t.0.join(".northstar/state/jobs.sqlite");
    let input = t.0.join("input.json");
    fs::write(
        &input,
        r#"{"schema":1,"tasks":[{"id":"a","input":{"operation":"content-digest","utf8":"abc"}}]}"#,
    )
    .unwrap();
    okay(run(&["queue", "init", db.to_str().unwrap()]));
    okay(run(&[
        "queue",
        "submit",
        db.to_str().unwrap(),
        "job",
        input.to_str().unwrap(),
        "50",
        "1",
        "--reviewed-redacted",
    ]));
    okay(run(&["queue", "run-one", db.to_str().unwrap()]));
    let exchange = |project: &std::path::Path, requests: &[Value]| -> Vec<Value> {
        let mut child = Command::new(env!("CARGO_BIN_EXE_nsre"))
            .args(["mcp", "--project"])
            .arg(project)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let mut pipe = child.stdin.take().unwrap();
        for request in requests {
            writeln!(pipe, "{request}").unwrap();
        }
        drop(pipe);
        let out = child.wait_with_output().unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8(out.stdout)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect()
    };
    let requests = vec![
        json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25"}}),
        json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
        json!({"jsonrpc":"2.0","id":2,"method":"tools/list"}),
        json!({"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"nsre_capabilities","arguments":{}}}),
        json!({"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"nsre_queue_status","arguments":{"id":"job"}}}),
        json!({"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"nsre_queue_status","arguments":{"id":"job","database":"elsewhere"}}}),
        json!({"jsonrpc":"2.0","id":6,"method":"tools/call","params":{"name":"nsre_queue_status","arguments":{"id":"../escape"}}}),
    ];
    let responses = exchange(&t.0, &requests);
    assert_eq!(responses[1]["result"]["tools"].as_array().unwrap().len(), 4);
    let capabilities: Value = serde_json::from_str(
        responses[2]["result"]["content"][0]["text"]
            .as_str()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(capabilities["queue"]["mcp_submit"], false);
    assert_eq!(
        capabilities["engines"]["engines"].as_array().unwrap().len(),
        12
    );
    let status: Value = serde_json::from_str(
        responses[3]["result"]["content"][0]["text"]
            .as_str()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(status["state"], "completed");
    assert_eq!(status["result"]["results"][0]["result"]["passed"], true);
    assert_eq!(responses[4]["error"]["code"], -32602);
    assert_eq!(responses[5]["error"]["code"], -32602);
    let isolated = exchange(&other.0, &requests[..5]);
    assert_eq!(isolated[3]["result"]["isError"], true);
    assert!(
        !other.0.join(".northstar").exists(),
        "read must not create project state"
    );
}
