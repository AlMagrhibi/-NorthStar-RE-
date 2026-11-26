//! Bounded parallel execution of reviewed, offline lab operations.
use rayon::prelude::*;
use serde_json::{json, Value};
use std::{
    collections::BTreeSet,
    ffi::OsString,
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    path::Path,
};

const MAX_BYTES: u64 = 1024 * 1024;
const MAX_TASKS: usize = 64;

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

fn evaluate(input: &Value, jobs: usize) -> io::Result<Value> {
    if !(1..=8).contains(&jobs) || input["schema"].as_u64() != Some(1) {
        return Err(invalid("invalid worker count or schema"));
    }
    let tasks = input["tasks"]
        .as_array()
        .filter(|tasks| !tasks.is_empty() && tasks.len() <= MAX_TASKS)
        .ok_or_else(|| invalid("expected 1..=64 tasks"))?;
    let mut ids = BTreeSet::new();
    for task in tasks {
        let id = task["id"]
            .as_str()
            .filter(|id| !id.is_empty() && id.len() <= 64)
            .ok_or_else(|| invalid("invalid task id"))?;
        if !ids.insert(id) || !task["input"].is_object() {
            return Err(invalid("duplicate id or invalid task input"));
        }
    }
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(jobs)
        .build()
        .map_err(io::Error::other)?;
    let results: Vec<Value> = pool.install(|| {
        tasks
            .par_iter()
            .map(|task| match crate::lab::evaluate(&task["input"]) {
                Ok(result) => json!({
                    "id":task["id"],"state":"completed","result":result
                }),
                Err(error) => json!({
                    "id":task["id"],"state":"failed","error":error.to_string()
                }),
            })
            .collect()
    });
    let passed = results.iter().all(|result| {
        result["state"] == "completed" && result["result"]["passed"] == true
    });
    Ok(json!({
        "schema":1,"complete":true,"passed":passed,"workers":jobs,
        "scope":"offline lab only","results":results
    }))
}

pub fn command(args: impl Iterator<Item = OsString>) -> io::Result<bool> {
    let args: Vec<_> = args.collect();
    if args.len() != 7
        || args[0] != "--input"
        || args[2] != "--out"
        || args[4] != "--jobs"
        || args[6] != "--reviewed-redacted"
    {
        return Err(invalid(
            "usage: nsre batch --input FILE --out NEW_FILE --jobs N --reviewed-redacted",
        ));
    }
    let path = Path::new(&args[1]);
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.is_file() || crate::unsafe_entry(&metadata) || metadata.len() > MAX_BYTES {
        return Err(invalid("expected regular input file up to 1 MiB"));
    }
    let mut bytes = Vec::new();
    File::open(path)?
        .take(MAX_BYTES + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > MAX_BYTES {
        return Err(invalid("input exceeds 1 MiB"));
    }
    let jobs = args[5]
        .to_str()
        .and_then(|value| value.parse().ok())
        .ok_or_else(|| invalid("invalid worker count"))?;
    let input = serde_json::from_slice(&bytes).map_err(io::Error::other)?;
    let result = evaluate(&input, jobs)?;
    let encoded = serde_json::to_vec(&result).map_err(io::Error::other)?;
    let mut out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[3])?;
    out.write_all(&encoded)?;
    out.write_all(b"\n")?;
    out.sync_all()?;
    Ok(result["passed"] == true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parallel_results_preserve_order_and_fail_closed() {
        let input = json!({"schema":1,"tasks":[
            {"id":"a","input":{"operation":"json-roundtrip","data":{"a":1}}},
            {"id":"b","input":{"operation":"regression-check","baseline":1,"actual":2}},
            {"id":"c","input":{"operation":"unknown"}}
        ]});
        let report = evaluate(&input, 2).unwrap();
        assert_eq!(report["passed"], false);
        assert_eq!(report["results"][0]["id"], "a");
        assert_eq!(report["results"][0]["result"]["passed"], true);
        assert_eq!(report["results"][1]["result"]["passed"], false);
        assert_eq!(report["results"][2]["state"], "failed");
        assert_eq!(report["results"], evaluate(&input, 1).unwrap()["results"]);
    }

    #[test]
    fn rejects_duplicate_ids_and_invalid_workers() {
        let input = json!({"schema":1,"tasks":[
            {"id":"a","input":{}},{"id":"a","input":{}}
        ]});
        assert!(evaluate(&input, 2).is_err());
        assert!(evaluate(&input, 0).is_err());
        assert!(evaluate(&input, 9).is_err());
        assert!(evaluate(&json!({"schema":1,"tasks":[]}), 1).is_err());
    }
}
