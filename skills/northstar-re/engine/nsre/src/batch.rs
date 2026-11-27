//! Bounded parallel execution of reviewed, offline lab operations.
use rayon::prelude::*;
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
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

fn engine_for(operation: &str) -> Option<&'static str> {
    static REGISTRY: std::sync::OnceLock<Value> = std::sync::OnceLock::new();
    let registry = REGISTRY.get_or_init(|| {
        serde_json::from_str(include_str!("engine-registry.json")).expect("embedded registry")
    });
    registry["engines"].as_array()?.iter().find_map(|engine| {
        engine["batch_operations"]
            .as_array()?
            .iter()
            .any(|value| value.as_str() == Some(operation))
            .then(|| engine["id"].as_str())
            .flatten()
    })
}

fn dependency_layers(input: &Value, jobs: usize) -> io::Result<Vec<Vec<usize>>> {
    if !(1..=8).contains(&jobs) || input["schema"].as_u64() != Some(1) {
        return Err(invalid("invalid worker count or schema"));
    }
    let tasks = input["tasks"]
        .as_array()
        .filter(|tasks| !tasks.is_empty() && tasks.len() <= MAX_TASKS)
        .ok_or_else(|| invalid("expected 1..=64 tasks"))?;
    let mut ids = BTreeMap::new();
    for (index, task) in tasks.iter().enumerate() {
        let id = task["id"]
            .as_str()
            .filter(|id| !id.is_empty() && id.len() <= 64)
            .ok_or_else(|| invalid("invalid task id"))?;
        if ids.insert(id, index).is_some() || !task["input"].is_object() {
            return Err(invalid("duplicate id or invalid task input"));
        }
        if let Some(engine) = task.get("engine") {
            let operation = task["input"]["operation"]
                .as_str()
                .ok_or_else(|| invalid("engine requires operation"))?;
            if engine.as_str().is_none() || engine.as_str() != engine_for(operation) {
                return Err(invalid("operation is not registered to requested engine"));
            }
        }
    }
    let dependencies = dependencies(tasks, &ids)?;
    let mut pending: BTreeSet<usize> = (0..tasks.len()).collect();
    let mut layers = Vec::new();
    while !pending.is_empty() {
        let ready: Vec<usize> = pending
            .iter()
            .copied()
            .filter(|i| dependencies[*i].iter().all(|d| !pending.contains(d)))
            .collect();
        if ready.is_empty() {
            return Err(invalid("dependency cycle"));
        }
        for index in &ready {
            pending.remove(index);
        }
        layers.push(ready);
    }
    Ok(layers)
}

fn dependencies(tasks: &[Value], ids: &BTreeMap<&str, usize>) -> io::Result<Vec<BTreeSet<usize>>> {
    tasks
        .iter()
        .enumerate()
        .map(|(index, task)| {
            let mut deps = BTreeSet::new();
            if let Some(value) = task.get("depends_on") {
                let entries = value
                    .as_array()
                    .ok_or_else(|| invalid("depends_on must be an array"))?;
                for entry in entries {
                    let dep = entry
                        .as_str()
                        .and_then(|id| ids.get(id))
                        .copied()
                        .ok_or_else(|| invalid("unknown dependency"))?;
                    if dep == index || !deps.insert(dep) {
                        return Err(invalid("self or duplicate dependency"));
                    }
                }
            }
            Ok(deps)
        })
        .collect()
}

pub(crate) fn validate(input: &Value, jobs: usize) -> io::Result<()> {
    dependency_layers(input, jobs).map(|_| ())
}

pub(crate) fn evaluate(input: &Value, jobs: usize) -> io::Result<Value> {
    let layers = dependency_layers(input, jobs)?;
    let tasks = input["tasks"]
        .as_array()
        .ok_or_else(|| invalid("expected tasks"))?;
    let ids: BTreeMap<&str, usize> = tasks
        .iter()
        .enumerate()
        .filter_map(|(index, task)| task["id"].as_str().map(|id| (id, index)))
        .collect();
    let dependencies = dependencies(tasks, &ids)?;
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(jobs)
        .build()
        .map_err(io::Error::other)?;
    let mut results = vec![Value::Null; tasks.len()];
    for layer in &layers {
        let completed: Vec<(usize, Value)> = pool.install(|| {
            layer
                .par_iter()
                .map(|&index| {
                    let task = &tasks[index];
                    let blocked = dependencies[index].iter().any(|&d| {
                        results[d]["state"] != "completed"
                            || results[d]["result"]["passed"] != true
                    });
                    let report = if blocked {
                        json!({"id":task["id"],"state":"blocked","reason":"dependency did not pass"})
                    } else {
                        match crate::lab::evaluate(&task["input"]) {
                            Ok(result) => json!({
                                "id":task["id"],"state":"completed","result":result
                            }),
                            Err(error) => json!({
                                "id":task["id"],"state":"failed","error":error.to_string()
                            }),
                        }
                    };
                    let mut report = report;
                    if let Some(engine) = task["input"]["operation"].as_str().and_then(engine_for)
                    {
                        report["engine"] = json!(engine);
                    }
                    (index, report)
                })
                .collect()
        });
        for (index, report) in completed {
            results[index] = report;
        }
    }
    let passed = results
        .iter()
        .all(|result| result["state"] == "completed" && result["result"]["passed"] == true);
    Ok(json!({
        "schema":1,"complete":true,"passed":passed,"workers":jobs,
        "scope":"offline lab only","dependency_layers":layers.len(),"results":results
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
    fn registry_covers_all_twelve_engines_and_checks_routing() {
        let registry: Value = serde_json::from_str(include_str!("engine-registry.json")).unwrap();
        let engines = registry["engines"].as_array().unwrap();
        assert_eq!(engines.len(), 12);
        let mut operations = BTreeSet::new();
        for engine in engines {
            let id = engine["id"].as_str().unwrap();
            let list = engine["batch_operations"].as_array().unwrap();
            assert!(!list.is_empty());
            for op in list {
                let op = op.as_str().unwrap();
                assert!(operations.insert(op));
                assert_eq!(engine_for(op), Some(id));
            }
        }
        let mut input = json!({"schema":1,"tasks":[{"id":"a","engine":"native-core",
            "input":{"operation":"content-digest","utf8":"abc"}}]});
        assert_eq!(
            evaluate(&input, 1).unwrap()["results"][0]["engine"],
            "native-core"
        );
        input["tasks"][0]["engine"] = json!("recovery");
        assert!(validate(&input, 1).is_err());
    }

    #[test]
    fn dependency_order_and_negative_propagation() {
        let input = json!({"schema":1,"tasks":[
            {"id":"dependent","depends_on":["bad"],"input":{"operation":"json-roundtrip","data":1}},
            {"id":"bad","input":{"operation":"regression-check","baseline":1,"actual":2}},
            {"id":"independent","input":{"operation":"json-roundtrip","data":2}},
            {"id":"transitive","depends_on":["dependent"],"input":{"operation":"json-roundtrip","data":3}}
        ]});
        let report = evaluate(&input, 2).unwrap();
        assert_eq!(report["dependency_layers"], 3);
        assert_eq!(report["results"][0]["state"], "blocked");
        assert_eq!(report["results"][2]["result"]["passed"], true);
        assert_eq!(report["results"][3]["state"], "blocked");
        assert_eq!(report["passed"], false);
        let mut cycle = input.clone();
        cycle["tasks"][1]["depends_on"] = json!(["transitive"]);
        assert!(validate(&cycle, 2).is_err());
        let mut missing = input;
        missing["tasks"][0]["depends_on"] = json!(["missing"]);
        assert!(validate(&missing, 2).is_err());
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
