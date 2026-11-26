//! Project-local governance and native CLI API. No target execution or network.
use crate::{hash_file, unsafe_entry};
use rusqlite::{params, Connection, OpenFlags};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    env,
    ffi::OsString,
    fs::{self, OpenOptions},
    io::{self, Read, Write},
    path::{Component, Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
type R<T> = Result<T, Box<dyn std::error::Error>>;
fn err(s: &str) -> Box<dyn std::error::Error> {
    io::Error::other(s).into()
}
fn now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}
fn id() -> String {
    format!(
        "{}-{}-{}",
        now(),
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos()
    )
}
fn root(p: &Path) -> R<PathBuf> {
    let m = fs::symlink_metadata(p)?;
    if unsafe_entry(&m) || !m.is_dir() {
        return Err(err("normal project directory required"));
    }
    Ok(p.canonicalize()?)
}
fn safe(r: &Path, n: &str) -> R<PathBuf> {
    if n.is_empty()
        || n.contains('\\')
        || n.contains(':')
        || n.split('/').any(|s| s.is_empty() || s == "." || s == "..")
    {
        return Err(err("invalid relative path"));
    }
    let mut p = r.to_path_buf();
    for c in Path::new(n).components() {
        if let Component::Normal(v) = c {
            p.push(v);
        } else {
            return Err(err("invalid path"));
        }
        match fs::symlink_metadata(&p) {
            Ok(m) if unsafe_entry(&m) => return Err(err("redirected path")),
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e.into()),
        }
    }
    Ok(p)
}
fn read_json(p: &Path) -> R<Value> {
    let m = fs::symlink_metadata(p)?;
    if unsafe_entry(&m) || !m.is_file() || m.len() > 16 * 1024 * 1024 {
        return Err(err("JSON input must be regular and <=16 MiB"));
    }
    let mut b = Vec::new();
    fs::File::open(p)?
        .take(16 * 1024 * 1024 + 1)
        .read_to_end(&mut b)?;
    if b.len() > 16 * 1024 * 1024 {
        return Err(err("input too large"));
    }
    Ok(serde_json::from_slice(&b)?)
}
fn new_file(p: &Path, b: &[u8]) -> R<()> {
    let mut f = OpenOptions::new().write(true).create_new(true).open(p)?;
    f.write_all(b)?;
    f.sync_all()?;
    Ok(())
}
fn new_json(p: &Path, v: &Value) -> R<()> {
    new_file(p, &serde_json::to_vec_pretty(v)?)
}
const DOCS: &[(&str, &str)] = &[
    (
        "ANALYSIS.md",
        include_str!("../../../references/templates/project-docs/ANALYSIS.md"),
    ),
    (
        "ARCHITECTURE.md",
        include_str!("../../../references/templates/project-docs/ARCHITECTURE.md"),
    ),
    (
        "BUILD.md",
        include_str!("../../../references/templates/project-docs/BUILD.md"),
    ),
    (
        "CHANGELOG.md",
        include_str!("../../../references/templates/project-docs/CHANGELOG.md"),
    ),
    (
        "DATA-FORMATS.md",
        include_str!("../../../references/templates/project-docs/DATA-FORMATS.md"),
    ),
    (
        "DATABASE.md",
        include_str!("../../../references/templates/project-docs/DATABASE.md"),
    ),
    (
        "DECISIONS.md",
        include_str!("../../../references/templates/project-docs/DECISIONS.md"),
    ),
    (
        "DLL-MAPPING.md",
        include_str!("../../../references/templates/project-docs/DLL-MAPPING.md"),
    ),
    (
        "ENVIRONMENT.md",
        include_str!("../../../references/templates/project-docs/ENVIRONMENT.md"),
    ),
    (
        "ERRORS.md",
        include_str!("../../../references/templates/project-docs/ERRORS.md"),
    ),
    (
        "HANDOFF.md",
        include_str!("../../../references/templates/project-docs/HANDOFF.md"),
    ),
    (
        "PROGRESS.md",
        include_str!("../../../references/templates/project-docs/PROGRESS.md"),
    ),
    (
        "PROJECT.md",
        include_str!("../../../references/templates/project-docs/PROJECT.md"),
    ),
    (
        "PROTOCOLS.md",
        include_str!("../../../references/templates/project-docs/PROTOCOLS.md"),
    ),
    (
        "REFERENCES.md",
        include_str!("../../../references/templates/project-docs/REFERENCES.md"),
    ),
    (
        "ROADMAP.md",
        include_str!("../../../references/templates/project-docs/ROADMAP.md"),
    ),
    (
        "SECURITY.md",
        include_str!("../../../references/templates/project-docs/SECURITY.md"),
    ),
    (
        "SESSION-LOG.md",
        include_str!("../../../references/templates/project-docs/SESSION-LOG.md"),
    ),
    (
        "TASKS.md",
        include_str!("../../../references/templates/project-docs/TASKS.md"),
    ),
    (
        "TESTING.md",
        include_str!("../../../references/templates/project-docs/TESTING.md"),
    ),
];
fn init(r: &Path) -> R<Value> {
    for n in [
        "docs",
        ".northstar",
        ".northstar/state",
        ".northstar/knowledge",
        ".northstar/evidence",
        ".northstar/checkpoints",
        ".northstar/logs",
        ".northstar/manifests",
        ".northstar/reports",
    ] {
        fs::create_dir_all(safe(r, n)?)?;
    }
    let mut created = vec![];
    for (n, b) in DOCS {
        let p = safe(r, &format!("docs/{n}"))?;
        if !p.exists() {
            new_file(&p, b.as_bytes())?;
            created.push(*n);
        }
    }
    for (n, v) in [
        (
            "project-state",
            json!({"schema_version":1,"status":"planned"}),
        ),
        ("current-task", json!({"task_id":null})),
        ("task-queue", json!({"tasks":[]})),
        (
            "session-state",
            json!({"session_id":null,"next_action":"Review PROJECT.md and scope"}),
        ),
    ] {
        let p = safe(r, &format!(".northstar/state/{n}.json"))?;
        if !p.exists() {
            new_json(&p, &v)?;
        }
    }
    Ok(json!({"created_docs":created,"existing_preserved":true}))
}
fn docs(r: &Path) -> R<Value> {
    let mut missing = vec![];
    let mut unchanged = vec![];
    for (n, b) in DOCS {
        let p = safe(r, &format!("docs/{n}"))?;
        if !p.is_file() || p.metadata()?.len() == 0 {
            missing.push(*n);
        } else if p.metadata()?.len() < 2 * 1024 * 1024 && fs::read_to_string(p)?.trim() == b.trim()
        {
            unchanged.push(*n);
        }
    }
    Ok(
        json!({"missing":missing,"unchanged_templates":unchanged,"structure_ok":missing.is_empty(),"content_review_required":true}),
    )
}
fn evidence(r: &Path, v: &Value) -> R<Value> {
    let a = v
        .as_array()
        .filter(|a| !a.is_empty())
        .ok_or_else(|| err("nonempty evidence array required"))?;
    for e in a {
        let n = string(&e["path"])?;
        let h = string(&e["sha256"])?;
        if hash_file(&safe(r, n)?)?.1 != h {
            return Err(err("evidence hash mismatch"));
        }
    }
    Ok(json!({"verified_files":a.len(),"authenticity_review_required":true}))
}
fn string(v: &Value) -> R<&str> {
    v.as_str()
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| err("nonempty string required"))
}
const GATES: [&str; 9] = [
    "requirements",
    "review",
    "build",
    "tests",
    "regression",
    "evidence",
    "documentation",
    "memory",
    "checkpoint",
];
fn gate(r: &Path, v: &Value) -> R<Value> {
    let g = v["gates"]
        .as_object()
        .filter(|o| o.len() == 9)
        .ok_or_else(|| err("nine gates required"))?;
    let mut blocked = vec![];
    for k in GATES {
        let x = g.get(k).ok_or_else(|| err("missing gate"))?;
        let status = x["status"].as_str().unwrap_or("");
        let mandatory = matches!(
            k,
            "requirements" | "review" | "evidence" | "documentation" | "memory"
        );
        if string(&x["reason"]).is_err()
            || !matches!(status, "pass" | "not_applicable")
            || (status == "not_applicable" && mandatory)
            || (status == "pass" && evidence(r, &x["evidence"]).is_err())
        {
            blocked.push(k);
        }
    }
    let d = docs(r)?;
    if d["structure_ok"] != true || !d["unchanged_templates"].as_array().unwrap().is_empty() {
        blocked.push("docs_not_completed");
    }
    Ok(json!({"gate_passed":blocked.is_empty(),"blocked":blocked,"semantic_review_required":true}))
}
fn plan(v: &Value) -> R<Value> {
    let list = v["tasks"]
        .as_array()
        .ok_or_else(|| err("tasks array required"))?;
    let mut map = BTreeMap::new();
    for t in list {
        let k = string(&t["id"])?;
        let st = string(&t["status"])?;
        string(&t["acceptance"])?;
        if !matches!(st, "planned" | "running" | "blocked" | "verified")
            || map.insert(k, t).is_some()
        {
            return Err(err("invalid state or duplicate ID"));
        }
        let dep = t["depends_on"]
            .as_array()
            .ok_or_else(|| err("dependencies required"))?;
        let mut uniq = BTreeSet::new();
        for d in dep {
            if !uniq.insert(string(d)?) {
                return Err(err("duplicate dependency"));
            }
        }
        if st == "verified"
            && !t["evidence"]
                .as_array()
                .is_some_and(|a| !a.is_empty() && a.iter().all(|e| string(e).is_ok()))
        {
            return Err(err("verified task requires evidence references"));
        }
    }
    let mut done = BTreeSet::new();
    let mut ordered = vec![];
    let mut ready = vec![];
    for (k, t) in &map {
        for d in t["depends_on"].as_array().unwrap() {
            let dep = map
                .get(string(d)?)
                .ok_or_else(|| err("unknown dependency"))?;
            if matches!(t["status"].as_str(), Some("running" | "verified"))
                && dep["status"] != "verified"
            {
                return Err(err("unverified predecessor"));
            }
        }
        if t["status"] == "planned"
            && t["depends_on"]
                .as_array()
                .unwrap()
                .iter()
                .all(|d| map[d.as_str().unwrap()]["status"] == "verified")
        {
            ready.push(*k);
        }
    }
    while done.len() < map.len() {
        let layer: Vec<_> = map
            .iter()
            .filter(|(k, t)| {
                !done.contains(**k)
                    && t["depends_on"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .all(|d| done.contains(d.as_str().unwrap()))
            })
            .map(|(k, _)| *k)
            .collect();
        if layer.is_empty() {
            return Err(err("dependency cycle"));
        }
        for k in layer {
            done.insert(k);
            ordered.push(k);
        }
    }
    Ok(
        json!({"valid":true,"topological_order":ordered,"ready":ready,"release_ready":!map.is_empty()&&map.values().all(|t|t["status"]=="verified"),"structural_only":true}),
    )
}
fn state_db(r: &Path) -> R<Connection> {
    let p = safe(r, ".northstar/state/framework.sqlite")?;
    for suffix in ["-wal", "-shm", "-journal"] {
        safe(r, &format!(".northstar/state/framework.sqlite{suffix}"))?;
    }
    let c = Connection::open(p)?;
    c.busy_timeout(std::time::Duration::from_secs(10))?;
    let app: i64 = c.query_row("PRAGMA application_id", [], |x| x.get(0))?;
    if app == 0 {
        let n: i64 = c.query_row("SELECT count(*) FROM sqlite_master", [], |x| x.get(0))?;
        if n != 0 {
            return Err(err("unrelated database"));
        }
        c.execute_batch("BEGIN IMMEDIATE; PRAGMA application_id=1314083398; PRAGMA user_version=1; CREATE TABLE IF NOT EXISTS records(id TEXT PRIMARY KEY, kind TEXT NOT NULL, payload TEXT NOT NULL, created_ms TEXT NOT NULL); COMMIT;")?;
    } else if app != 1314083398 {
        return Err(err("unrelated database"));
    }
    let ver: i64 = c.query_row("PRAGMA user_version", [], |x| x.get(0))?;
    if ver != 1 {
        return Err(err("unsupported state schema"));
    }
    c.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=FULL;")?;
    Ok(c)
}
fn save_record(r: &Path, kind: &str, v: &Value) -> R<Value> {
    let c = state_db(r)?;
    let key = id();
    c.execute(
        "INSERT INTO records VALUES (?1,?2,?3,?4)",
        params![key, kind, serde_json::to_string(v)?, now().to_string()],
    )?;
    Ok(json!({"id":key,"kind":kind,"stored":true}))
}
fn records(r: &Path, kind: &str) -> R<Value> {
    let p = safe(r, ".northstar/state/framework.sqlite")?;
    if !p.exists() {
        return Ok(json!([]));
    }
    let c = Connection::open_with_flags(p, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    let app: i64 = c.query_row("PRAGMA application_id", [], |x| x.get(0))?;
    if app != 1314083398 {
        return Err(err("unrelated database"));
    }
    let mut s = c.prepare(
        "SELECT id,payload,created_ms FROM records WHERE kind=?1 ORDER BY rowid DESC LIMIT 100",
    )?;
    let mut q = s.query([kind])?;
    let mut out = vec![];
    while let Some(x) = q.next()? {
        out.push(json!({"id":x.get::<_,String>(0)?,"payload":serde_json::from_str::<Value>(&x.get::<_,String>(1)?)?,"created_ms":x.get::<_,String>(2)?}));
    }
    Ok(json!(out))
}
fn knowledge(r: &Path, v: &Value) -> R<Value> {
    let fields = [
        "category",
        "target_sha256",
        "claim",
        "confidence",
        "source",
        "locator",
        "method",
        "tool_version",
        "evidence",
        "limitations",
        "supersedes",
    ];
    if v.as_object().map(|x| x.len()) != Some(fields.len()) {
        return Err(err("knowledge template fields required"));
    }
    for k in fields {
        if !matches!(k, "evidence" | "supersedes") {
            if string(&v[k])?.len() > 16000 {
                return Err(err("field too long"));
            }
        } else if v.get(k).is_none() {
            return Err(err("missing field"));
        }
    }
    let h = string(&v["target_sha256"])?;
    if h.len() != 64
        || !h
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(err("invalid SHA-256"));
    }
    if !matches!(string(&v["confidence"])?, "proven" | "likely" | "unknown") {
        return Err(err("invalid confidence"));
    }
    let ev = v["evidence"]
        .as_array()
        .ok_or_else(|| err("evidence array required"))?;
    if ev.iter().any(|e| string(e).is_err()) || (v["confidence"] == "proven" && ev.is_empty()) {
        return Err(err("invalid evidence"));
    }
    save_record(r, "knowledge", v)
}
fn checkpoint(r: &Path, v: &Value) -> R<Value> {
    let a = v
        .as_array()
        .filter(|a| !a.is_empty())
        .ok_or_else(|| err("explicit files required"))?;
    let mut seen = BTreeSet::new();
    let mut entries = vec![];
    for e in a {
        let n = string(e)?;
        if n.starts_with(".northstar/") || n.starts_with(".git/") || !seen.insert(n) {
            return Err(err("internal or duplicate path"));
        }
        let h = hash_file(&safe(r, n)?)?.1;
        entries.push(json!({"path":n,"sha256":h}));
    }
    let key = id();
    let b = safe(r, &format!(".northstar/checkpoints/{key}"))?;
    fs::create_dir(&b)?;
    for e in &entries {
        let n = string(&e["path"])?;
        let dest = safe(&b, &format!("files/{n}"))?;
        fs::create_dir_all(dest.parent().unwrap())?;
        let mut source = fs::File::open(safe(r, n)?)?;
        let mut out = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&dest)?;
        io::copy(&mut source, &mut out)?;
        out.sync_all()?;
        if hash_file(&dest)?.1 != string(&e["sha256"])? {
            return Err(err("checkpoint copy mismatch; incomplete checkpoint"));
        }
    }
    new_json(
        &b.join("manifest.json"),
        &json!({"created_ms":now(),"files":entries}),
    )?;
    Ok(json!({"checkpoint":key,"files":entries.len(),"restore":"manual reviewed restore only"}))
}
// Candidate discovery is read-only. An explicit override is authoritative,
// including when invalid; never silently fall back to a different executable.
fn tool_candidate(
    name: &str,
    configured: Option<&Path>,
    paths: &[PathBuf],
    hints: &[PathBuf],
) -> (Option<PathBuf>, &'static str) {
    let valid = |p: &Path| p.is_absolute() && p.is_file();
    if let Some(p) = configured {
        return if valid(p) {
            (Some(p.to_path_buf()), "explicit")
        } else {
            (None, "invalid_override")
        };
    }
    for (dirs, source) in [(paths, "path"), (hints, "environment_hint")] {
        for d in dirs {
            if !d.is_absolute() {
                continue;
            }
            for suffix in ["", ".exe", ".bat", ".cmd"] {
                let candidate = d.join(format!("{name}{suffix}"));
                if valid(&candidate) {
                    return (Some(candidate), source);
                }
            }
        }
    }
    (None, "not_found")
}

fn tool_hints(name: &str) -> Vec<PathBuf> {
    let mut hints = Vec::new();
    let mut add = |key: &str, suffix: &str| {
        if let Some(value) = env::var_os(key) {
            let base = PathBuf::from(value);
            if base.is_absolute() {
                hints.push(base.join(suffix));
            }
        }
    };
    match name {
        "cargo" | "rustc" => {
            add("CARGO_HOME", "bin");
            add("USERPROFILE", ".cargo/bin");
            add("HOME", ".cargo/bin");
        }
        "java" | "javac" => add("JAVA_HOME", "bin"),
        "ghidraRun" => add("GHIDRA_HOME", ""),
        "analyzeHeadless" => add("GHIDRA_HOME", "support"),
        "msbuild" => add("VSINSTALLDIR", "MSBuild/Current/Bin"),
        "cl" | "dumpbin" => {
            add("VCToolsInstallDir", "bin/Hostx64/x64");
            add("VCToolsInstallDir", "bin/Hostx64/x86");
            add("VCToolsInstallDir", "bin/Hostx86/x86");
        }
        "7z" => {
            add("ProgramFiles", "7-Zip");
            add("ProgramFiles(x86)", "7-Zip");
        }
        "tshark" => add("ProgramFiles", "Wireshark"),
        "git" => add("ProgramFiles", "Git/cmd"),
        "dotnet" => {
            add("DOTNET_ROOT", "");
            add("ProgramFiles", "dotnet");
        }
        "pwsh" => add("ProgramFiles", "PowerShell/7"),
        "ilspycmd" => add("USERPROFILE", ".dotnet/tools"),
        _ => {}
    }
    hints
}

fn tool_paths() -> Value {
    let names = [
        "cargo",
        "rustc",
        "git",
        "java",
        "javac",
        "ghidraRun",
        "analyzeHeadless",
        "msbuild",
        "cl",
        "dotnet",
        "dumpbin",
        "windbg",
        "tshark",
        "7z",
        "ilspycmd",
        "pwsh",
    ];
    let paths: Vec<_> = env::var_os("PATH")
        .map(|p| env::split_paths(&p).collect())
        .unwrap_or_default();
    json!(names
        .iter()
        .map(|n| {
            let key = format!("NSRE_TOOL_{}", n.to_ascii_uppercase());
            let configured = env::var_os(&key).map(PathBuf::from);
            let hints = tool_hints(n);
            let (found, source) = tool_candidate(n, configured.as_deref(), &paths, &hints);
            json!({
                "name": n,
                "path": found,
                "verified": false,
                "source": source,
                "override_env": key
            })
        })
        .collect::<Vec<_>>())
}

pub fn doctor() -> Value {
    json!({"framework":"NorthStar RE V11","engine_version":env!("CARGO_PKG_VERSION"),"os":env::consts::OS,"arch":env::consts::ARCH,"parallelism":std::thread::available_parallelism().map(|n|n.get()).unwrap_or(1),"tools":tool_paths(),"tool_smoke_tests_required":true,"python_required":false})
}
fn status(r: &Path) -> R<Value> {
    let p = safe(r, ".northstar/state/session-state.json")?;
    Ok(
        json!({"project":r,"documentation":docs(r)?,"session":if p.is_file(){read_json(&p)?}else{Value::Null},"session_history":records(r,"session")?,"jobs":records(r,"plan")?,"engine_version":env!("CARGO_PKG_VERSION"),"live_windows_verified":false}),
    )
}
fn audit(v: &Value) -> R<Value> {
    let a = v
        .as_array()
        .ok_or_else(|| err("dependency array required"))?;
    let issues: Vec<_> = a
        .iter()
        .enumerate()
        .filter(|(_, d)| {
            ["name", "version", "source", "license", "review_status"]
                .iter()
                .any(|k| string(&d[*k]).is_err())
                || d["review_status"] != "reviewed"
        })
        .map(|(i, _)| i)
        .collect();
    Ok(json!({"count":a.len(),"unreviewed_indices":issues,"vulnerability_scan":false}))
}
fn progress(r: &Path, v: &Value) -> R<Value> {
    let p = plan(v)?;
    let list = v["tasks"].as_array().unwrap();
    let mut done = BTreeSet::new();
    for k in p["topological_order"].as_array().unwrap() {
        let k = string(k)?;
        let t = list.iter().find(|t| t["id"] == k).unwrap();
        if t["status"] == "verified"
            && t["depends_on"]
                .as_array()
                .unwrap()
                .iter()
                .all(|d| done.contains(d.as_str().unwrap()))
            && gate(r, &t["quality"]).is_ok_and(|g| g["gate_passed"] == true)
        {
            done.insert(k);
        }
    }
    Ok(
        json!({"total":list.len(),"verified":done.len(),"verified_ids":done,"percent":if list.is_empty(){None}else{Some(100.0*done.len() as f64/list.len() as f64)}}),
    )
}
pub const COMMANDS: &[&str] = &[
    "doctor",
    "tools",
    "lab",
    "init",
    "resume",
    "status",
    "report",
    "validate-docs",
    "verify",
    "quality-gate",
    "progress",
    "session",
    "checkpoint",
    "dependency-audit",
    "jobs",
    "memory",
    "search",
    "benchmark",
    "hash",
    "mcp",
    "api",
];
pub fn command(cmd: &str, args: impl Iterator<Item = OsString>) -> R<bool> {
    let mut a = args;
    let mut flags = BTreeMap::new();
    let mut reviewed = false;
    while let Some(k) = a.next() {
        let k = k.into_string().map_err(|_| err("invalid flag"))?;
        if k == "--reviewed-redacted" {
            if reviewed {
                return Err(err("duplicate flag"));
            }
            reviewed = true;
            continue;
        }
        let v = a.next().ok_or_else(|| err("missing value"))?;
        if flags.insert(k, v).is_some() {
            return Err(err("duplicate flag"));
        }
    }
    let project = flags.remove("--project").map(PathBuf::from);
    let input = flags.remove("--input").map(PathBuf::from);
    let action = flags
        .remove("--action")
        .map(|s| s.to_string_lossy().to_string());
    if !flags.is_empty() {
        return Err(err("unknown flags"));
    }
    if input.is_some() && !reviewed {
        return Err(err("review input and confirm --reviewed-redacted"));
    }
    let v = if let Some(p) = input {
        read_json(&p)?
    } else {
        Value::Null
    };
    if cmd == "lab" {
        let result = crate::lab::evaluate(&v)?;
        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(result["passed"] == true);
    }
    if cmd == "doctor" || cmd == "tools" {
        println!(
            "{}",
            if cmd == "doctor" {
                doctor()
            } else {
                tool_paths()
            }
        );
        return Ok(true);
    }
    if cmd == "mcp" || cmd == "api" {
        return crate::rpc::serve(project.as_deref(), cmd == "mcp");
    }
    let r = root(
        project
            .as_deref()
            .ok_or_else(|| err("--project required"))?,
    )?;
    let mut ok = true;
    let val = match cmd {
        "init" => init(&r)?,
        "resume" | "status" => status(&r)?,
        "validate-docs" => {
            let d = docs(&r)?;
            ok = d["structure_ok"] == true;
            d
        }
        "report" => {
            println!(
                "# NorthStar project status\n\n```json\n{}\n```",
                serde_json::to_string_pretty(&status(&r)?)?
            );
            return Ok(true);
        }
        "verify" => evidence(&r, &v)?,
        "quality-gate" => {
            let g = gate(&r, &v)?;
            ok = g["gate_passed"] == true;
            g
        }
        "progress" => progress(&r, &v)?,
        "session" => {
            if v.is_null() {
                records(&r, "session")?
            } else {
                string(&v["session_id"])?;
                string(&v["next_action"])?;
                save_record(&r, "session", &v)?
            }
        }
        "checkpoint" => checkpoint(&r, &v)?,
        "dependency-audit" => audit(&v)?,
        "jobs" => match action.as_deref() {
            Some("validate") => plan(&v)?,
            Some("save") => {
                plan(&v)?;
                save_record(&r, "plan", &v)?
            }
            None | Some("list") => records(&r, "plan")?,
            _ => return Err(err("jobs action must be validate/save/list")),
        },
        "memory" => {
            if v.is_null() {
                records(&r, "knowledge")?
            } else {
                knowledge(&r, &v)?
            }
        }
        "search" => {
            let q = string(&v["query"])?;
            let c = Connection::open_with_flags(
                safe(&r, ".northstar/state/framework.sqlite")?,
                OpenFlags::SQLITE_OPEN_READ_ONLY,
            )?;
            let app: i64 = c.query_row("PRAGMA application_id", [], |x| x.get(0))?;
            if app != 1314083398 {
                return Err(err("unrelated database"));
            }
            let mut s=c.prepare("SELECT id,kind,payload FROM records WHERE instr(lower(payload),lower(?1))>0 ORDER BY rowid DESC LIMIT 100")?;
            let mut rows = s.query([q])?;
            let mut result = vec![];
            while let Some(x) = rows.next()? {
                result.push(json!({"id":x.get::<_,String>(0)?,"kind":x.get::<_,String>(1)?,"payload":serde_json::from_str::<Value>(&x.get::<_,String>(2)?)?}));
            }
            json!(result)
        }
        "hash" => {
            let p = safe(&r, string(&v["path"])?)?;
            let (size, sha256) = hash_file(&p)?;
            json!({"size":size,"sha256":sha256})
        }
        "benchmark" => {
            let corpus = safe(&r, string(&v["corpus"])?)?;
            let dst = safe(&r, ".northstar/reports")?;
            let out = dst.join(format!("benchmark-{}.jsonl", id()));
            let jobs = v["jobs"].as_u64().unwrap_or(4);
            if !(1..=64).contains(&jobs) {
                return Err(err("jobs out of bounds"));
            }
            let start = std::time::Instant::now();
            ok = crate::scan(&corpus, &out, jobs as usize)?;
            json!({"elapsed_ms":start.elapsed().as_millis(),"complete":ok,"report":out,"cold_cache_not_guaranteed":true})
        }
        _ => return Err(err("unknown command")),
    };
    println!("{}", serde_json::to_string_pretty(&val)?);
    Ok(ok)
}
pub fn rpc_status(p: &Path) -> R<Value> {
    status(&root(p)?)
}

#[cfg(test)]
mod discovery_tests {
    use super::*;

    #[test]
    fn explicit_path_precedes_path_and_hints_and_invalid_override_fails_closed() {
        let dir = env::temp_dir().join(format!("nsre-discovery-{}", id()));
        let path = dir.join("path");
        let hints = dir.join("hints");
        fs::create_dir_all(&path).unwrap();
        fs::create_dir_all(&hints).unwrap();
        let explicit = dir.join("explicit.exe");
        let candidate = path.join("testtool.exe");
        let hint = hints.join("testtool.exe");
        for p in [&explicit, &candidate, &hint] {
            fs::write(p, b"synthetic fixture; never executed").unwrap();
        }
        let paths = [path];
        let hint_dirs = [hints];
        assert_eq!(
            tool_candidate("testtool", Some(&explicit), &paths, &hint_dirs),
            (Some(explicit), "explicit")
        );
        assert_eq!(
            tool_candidate(
                "testtool",
                Some(Path::new("relative.exe")),
                &paths,
                &hint_dirs
            ),
            (None, "invalid_override")
        );
        assert_eq!(
            tool_candidate("testtool", None, &paths, &hint_dirs),
            (Some(candidate), "path")
        );
        assert_eq!(
            tool_candidate("testtool", None, &[], &hint_dirs),
            (Some(hint), "environment_hint")
        );
        assert_eq!(
            tool_candidate("missing", None, &[], &[]),
            (None, "not_found")
        );
        fs::remove_dir_all(dir).unwrap();
    }
}
