//! Bounded read-only evidence collection from explicitly selected local files.
use rayon::prelude::*;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    ffi::OsString,
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    path::{Component, Path, PathBuf},
};

const MAX_FILE: u64 = 64 * 1024 * 1024;
const MAX_MANIFEST: u64 = 1024 * 1024;
const MAX_TASKS: usize = 64;

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

fn normal_dir(path: &Path) -> io::Result<PathBuf> {
    let meta = fs::symlink_metadata(path)?;
    if !meta.is_dir() || crate::unsafe_entry(&meta) {
        return Err(invalid("normal directory required"));
    }
    let path = path.canonicalize()?;
    for ancestor in path.ancestors() {
        let meta = fs::symlink_metadata(ancestor)?;
        if crate::unsafe_entry(&meta) {
            return Err(invalid("reparse directory rejected"));
        }
    }
    Ok(path)
}

fn scoped_file(root: &Path, relative: &str) -> io::Result<PathBuf> {
    let mut path = root.to_path_buf();
    let components: Vec<_> = Path::new(relative).components().collect();
    if components.is_empty() || components.len() > 64 || relative.len() > 4096 {
        return Err(invalid("invalid relative path"));
    }
    for (index, component) in components.iter().enumerate() {
        let Component::Normal(name) = component else {
            return Err(invalid("relative normal components required"));
        };
        let name = name.to_str().ok_or_else(|| invalid("invalid path text"))?;
        if name.contains(':') || name.ends_with(['.', ' ']) {
            return Err(invalid("ambiguous Windows path component"));
        }
        path.push(name);
        let meta = fs::symlink_metadata(&path)?;
        if crate::unsafe_entry(&meta)
            || (index + 1 == components.len() && !meta.is_file())
            || (index + 1 != components.len() && !meta.is_dir())
        {
            return Err(invalid("nonregular path or reparse point"));
        }
    }
    let path = path.canonicalize()?;
    if !path.starts_with(root) {
        return Err(invalid("path escaped collection root"));
    }
    Ok(path)
}

fn inspect(root: &Path, task: &Value) -> io::Result<Value> {
    let relative = task["path"]
        .as_str()
        .ok_or_else(|| invalid("file path required"))?;
    let expected = match task.get("expected_sha256") {
        Some(value) => Some(
            value
                .as_str()
                .filter(|s| s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit()))
                .ok_or_else(|| invalid("invalid expected hash"))?,
        ),
        None => None,
    };
    let path = scoped_file(root, relative)?;
    let before = fs::symlink_metadata(&path)?;
    if before.len() > MAX_FILE {
        return Err(invalid("file exceeds 64 MiB collector limit"));
    }
    let mut reader = File::open(&path)?.take(MAX_FILE + 1);
    let mut digest = Sha256::new();
    let mut header = Vec::with_capacity(65536);
    let mut buffer = vec![0u8; 65536];
    let mut bytes = 0u64;
    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        bytes += n as u64;
        if bytes > MAX_FILE {
            return Err(invalid("file grew beyond collector limit"));
        }
        digest.update(&buffer[..n]);
        let capture = n.min(65536 - header.len());
        header.extend_from_slice(&buffer[..capture]);
    }
    let after = fs::symlink_metadata(&path)?;
    if crate::unsafe_entry(&after)
        || bytes != before.len()
        || bytes != after.len()
        || before.modified()? != after.modified()?
        || scoped_file(root, relative)? != path
    {
        return Err(invalid("file changed during collection"));
    }
    let sha256 = format!("{:x}", digest.finalize());
    let passed = expected.is_none_or(|value| sha256.eq_ignore_ascii_case(value));
    Ok(json!({
        "passed":passed,"path":relative,"bytes":bytes,"sha256":sha256,
        "expected_sha256":expected,"hash_recomputed":true,
        "signature_hint":crate::probe::signature(&header),
        "sample_bytes":header.len(),"sample_entropy_bits":crate::probe::entropy(&header),
        "full_format_validated":false,"source_modified":false
    }))
}

fn evaluate(root: &Path, input: &Value, jobs: usize) -> io::Result<Value> {
    if input["schema"].as_u64() != Some(1) || !(1..=8).contains(&jobs) {
        return Err(invalid("invalid schema or workers"));
    }
    let tasks = input["files"]
        .as_array()
        .filter(|items| !items.is_empty() && items.len() <= MAX_TASKS)
        .ok_or_else(|| invalid("expected 1..=64 files"))?;
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(jobs)
        .build()
        .map_err(io::Error::other)?;
    let results: Vec<Value> = pool.install(|| {
        tasks
            .par_iter()
            .map(|task| match inspect(root, task) {
                Ok(result) => result,
                Err(error) => json!({
                    "passed":false,"path":task["path"],"error":error.to_string()
                }),
            })
            .collect()
    });
    let passed = results.iter().all(|result| result["passed"] == true);
    Ok(json!({
        "schema":1,"scope":"explicit local files; read-only",
        "passed":passed,"workers":jobs,"results":results
    }))
}

pub fn command(args: impl Iterator<Item = OsString>) -> io::Result<bool> {
    let args: Vec<_> = args.collect();
    if args.len() != 9
        || args[0] != "--root"
        || args[2] != "--input"
        || args[4] != "--out"
        || args[6] != "--jobs"
        || args[8] != "--reviewed-redacted"
    {
        return Err(invalid(
            "usage: nsre collect --root DIR --input FILE --out NEW_FILE --jobs N --reviewed-redacted",
        ));
    }
    let root = normal_dir(Path::new(&args[1]))?;
    let manifest = Path::new(&args[3]);
    let meta = fs::symlink_metadata(manifest)?;
    if crate::unsafe_entry(&meta) || !meta.is_file() || meta.len() > MAX_MANIFEST {
        return Err(invalid("invalid manifest"));
    }
    let mut data = Vec::new();
    File::open(manifest)?
        .take(MAX_MANIFEST + 1)
        .read_to_end(&mut data)?;
    if data.len() as u64 > MAX_MANIFEST {
        return Err(invalid("manifest too large"));
    }
    let input = serde_json::from_slice(&data).map_err(io::Error::other)?;
    let jobs = args[7]
        .to_str()
        .and_then(|value| value.parse().ok())
        .ok_or_else(|| invalid("invalid worker count"))?;
    let output = Path::new(&args[5]);
    let parent = normal_dir(
        output
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new(".")),
    )?;
    if parent.starts_with(&root) {
        return Err(invalid("output must be outside source root"));
    }
    let name = output
        .file_name()
        .ok_or_else(|| invalid("output name required"))?;
    let report = evaluate(&root, &input, jobs)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(parent.join(name))?;
    serde_json::to_writer(&mut file, &report).map_err(io::Error::other)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(report["passed"] == true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn independent_hash_and_scope() {
        let dir = std::env::temp_dir().join(format!(
            "nsre-collect-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&dir).unwrap();
        let root = normal_dir(&dir).unwrap();
        fs::write(root.join("a.txt"), b"abc").unwrap();
        let expected = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
        let input = json!({"schema":1,"files":[
            {"path":"a.txt","expected_sha256":expected},
            {"path":"a.txt","expected_sha256":"0".repeat(64)},
            {"path":"../outside.txt"}
        ]});
        let report = evaluate(&root, &input, 2).unwrap();
        assert_eq!(report["results"][0]["passed"], true);
        assert_eq!(report["results"][0]["sha256"], expected);
        assert_eq!(report["results"][1]["passed"], false);
        assert_eq!(report["results"][2]["passed"], false);
        assert_eq!(report["passed"], false);
        assert!(scoped_file(&root, "a.txt:stream").is_err());
        fs::remove_dir_all(&dir).unwrap();
    }
}
