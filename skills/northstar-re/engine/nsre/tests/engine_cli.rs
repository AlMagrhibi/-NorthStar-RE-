use rusqlite::Connection;
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "nsre-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
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
