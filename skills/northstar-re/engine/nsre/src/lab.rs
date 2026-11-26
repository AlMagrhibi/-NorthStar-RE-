//! Generic, offline engineering validation helpers. No target execution or network.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, io};

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

fn text<'a>(v: &'a Value, key: &str) -> io::Result<&'a str> {
    v[key]
        .as_str()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| invalid("missing nonempty string"))
}

fn array<'a>(v: &'a Value, key: &str) -> io::Result<&'a [Value]> {
    v[key]
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| invalid("expected array"))
}

fn is_sha256(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit())
}

fn sha256_kat(v: &Value) -> io::Result<Value> {
    let input = v["utf8"]
        .as_str()
        .ok_or_else(|| invalid("utf8 string required"))?;
    let expected = text(v, "expected_sha256")?;
    if !is_sha256(expected) {
        return Err(invalid("invalid expected SHA-256"));
    }
    let actual = format!("{:x}", Sha256::digest(input.as_bytes()));
    Ok(json!({"passed":actual.eq_ignore_ascii_case(expected),"actual_sha256":actual}))
}

fn json_roundtrip(v: &Value) -> io::Result<Value> {
    let data = v
        .get("data")
        .ok_or_else(|| invalid("data required"))?;
    let bytes = serde_json::to_vec(data).map_err(io::Error::other)?;
    let decoded: Value = serde_json::from_slice(&bytes).map_err(io::Error::other)?;
    Ok(json!({"passed":decoded == *data,"encoded_bytes":bytes.len()}))
}

fn schema_check(v: &Value, protocol: bool) -> io::Result<Value> {
    let schema = v["fields"]
        .as_object()
        .ok_or_else(|| invalid("fields object required"))?;
    let data = v["data"]
        .as_object()
        .ok_or_else(|| invalid("data object required"))?;
    let mut issues = Vec::new();
    if protocol && text(v, "version")? != text(v, "expected_version")? {
        issues.push("version_mismatch".to_owned());
    }
    for (name, kind) in schema {
        let kind = kind
            .as_str()
            .ok_or_else(|| invalid("field type must be a string"))?;
        if !matches!(kind, "string" | "integer" | "boolean" | "object" | "array") {
            return Err(invalid("unsupported field type"));
        }
        let matches = data.get(name).is_some_and(|value| match kind {
            "string" => value.is_string(),
            "integer" => value.as_i64().is_some() || value.as_u64().is_some(),
            "boolean" => value.is_boolean(),
            "object" => value.is_object(),
            "array" => value.is_array(),
            _ => false,
        });
        if !matches {
            issues.push(format!("missing_or_wrong_type:{name}"));
        }
    }
    for name in data.keys() {
        if !schema.contains_key(name) {
            issues.push(format!("unknown_field:{name}"));
        }
    }
    Ok(json!({"passed":issues.is_empty(),"issues":issues,"scope":"flat required-field schema"}))
}

fn evidence_check(v: &Value) -> io::Result<Value> {
    let records = array(v, "records")?;
    let mut ids = BTreeSet::new();
    for record in records {
        if !ids.insert(text(record, "id")?)
            || !is_sha256(text(record, "sha256")?)
            || !matches!(
                text(record, "confidence")?,
                "observed" | "inference" | "unknown"
            )
            || text(record, "producer").is_err()
        {
            return Err(invalid("invalid or duplicate evidence record"));
        }
    }
    for record in records {
        for reference in array(record, "references")? {
            let id = reference
                .as_str()
                .ok_or_else(|| invalid("reference must be a string"))?;
            if !ids.contains(id) {
                return Err(invalid("unresolved evidence reference"));
            }
        }
    }
    Ok(json!({"passed":!records.is_empty(),"records":records.len(),"hashes_recomputed":false}))
}

fn assembly_summary(v: &Value) -> io::Result<Value> {
    let records = array(v, "instructions")?;
    let mut counts = std::collections::BTreeMap::new();
    for record in records {
        let category = text(record, "category")?;
        if !matches!(category, "arithmetic" | "branch" | "memory" | "other") {
            return Err(invalid("unknown instruction category"));
        }
        *counts.entry(category).or_insert(0usize) += 1;
    }
    Ok(json!({"passed":!records.is_empty(),"counts":counts,"disassembled":false}))
}

fn gate_summary(v: &Value) -> io::Result<Value> {
    let gates = array(v, "gates")?;
    let mut ids = BTreeSet::new();
    let mut passed = 0usize;
    for gate in gates {
        if !ids.insert(text(gate, "id")?) {
            return Err(invalid("duplicate gate"));
        }
        match text(gate, "status")? {
            "passed" if is_sha256(text(gate, "evidence_sha256")?) => passed += 1,
            "blocked" | "failed" | "unverified" => {}
            _ => return Err(invalid("invalid gate or missing evidence hash")),
        }
    }
    Ok(json!({"passed":!gates.is_empty() && passed == gates.len(),"total":gates.len(),"reported_passes":passed,"evidence_verified":false}))
}

fn isolation_plan(v: &Value) -> io::Result<Value> {
    let required = [
        "disposable_workspace",
        "network_isolated",
        "no_shared_credentials",
        "rollback_defined",
    ];
    let missing: Vec<_> = required
        .into_iter()
        .filter(|key| v[*key].as_bool() != Some(true))
        .collect();
    Ok(json!({"passed":missing.is_empty(),"missing":missing,"enforced":false}))
}

pub fn evaluate(v: &Value) -> io::Result<Value> {
    match text(v, "operation")? {
        "algorithm-catalog" => Ok(json!({
            "passed":true,
            "algorithms":[{"name":"SHA-256","kind":"digest","reference":"FIPS 180-4"}],
            "automatic_identification":false
        })),
        "sha256-kat" => sha256_kat(v),
        "assembly-summary" => assembly_summary(v),
        "evidence-check" => evidence_check(v),
        "json-roundtrip" => json_roundtrip(v),
        "gate-summary" => gate_summary(v),
        "isolation-plan" => isolation_plan(v),
        "protocol-check" => schema_check(v, true),
        "config-check" => schema_check(v, false),
        "regression-check" => {
            let baseline = v.get("baseline").ok_or_else(|| invalid("baseline required"))?;
            let actual = v.get("actual").ok_or_else(|| invalid("actual required"))?;
            Ok(json!({"passed":baseline == actual,"comparison":"JSON value equality"}))
        }
        _ => Err(invalid("unknown lab operation")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digest_and_roundtrip() {
        let expected = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
        let kat = json!({"operation":"sha256-kat","utf8":"abc","expected_sha256":expected});
        assert_eq!(evaluate(&kat).unwrap()["passed"], true);
        let wrong = json!({"operation":"sha256-kat","utf8":"abcd","expected_sha256":expected});
        assert_eq!(evaluate(&wrong).unwrap()["passed"], false);
        assert_eq!(
            evaluate(&json!({"operation":"json-roundtrip","data":{"x":[1,true,null]}}))
                .unwrap()["passed"],
            true
        );
    }

    #[test]
    fn schemas_and_version_mismatch() {
        let mut input = json!({
            "operation":"protocol-check","version":"1","expected_version":"1",
            "fields":{"id":"integer"},"data":{"id":7}
        });
        assert_eq!(evaluate(&input).unwrap()["passed"], true);
        input["expected_version"] = json!("2");
        assert_eq!(evaluate(&input).unwrap()["passed"], false);
        input["operation"] = json!("config-check");
        input["data"]["unexpected"] = json!(true);
        assert_eq!(evaluate(&input).unwrap()["passed"], false);
    }

    #[test]
    fn evidence_and_gates_do_not_claim_live_verification() {
        let digest = "a".repeat(64);
        let evidence = json!({"operation":"evidence-check","records":[{
            "id":"a","sha256":digest,"confidence":"observed","producer":"fixture",
            "references":["missing"]
        }]});
        assert!(evaluate(&evidence).is_err());
        let gates = json!({"operation":"gate-summary","gates":[{"id":"g","status":"blocked"}]});
        assert_eq!(evaluate(&gates).unwrap()["passed"], false);
        assert_eq!(evaluate(&gates).unwrap()["evidence_verified"], false);
        let plan = json!({"operation":"isolation-plan","network_isolated":true});
        assert_eq!(evaluate(&plan).unwrap()["passed"], false);
        assert_eq!(evaluate(&plan).unwrap()["enforced"], false);
    }

    #[test]
    fn categories_and_regression() {
        let input = json!({"operation":"assembly-summary","instructions":[
            {"category":"memory"},{"category":"memory"},{"category":"branch"}
        ]});
        assert_eq!(evaluate(&input).unwrap()["counts"]["memory"], 2);
        let regression = json!({"operation":"regression-check","baseline":1,"actual":2});
        assert_eq!(evaluate(&regression).unwrap()["passed"], false);
        assert_eq!(
            evaluate(&json!({"operation":"algorithm-catalog"})).unwrap()["automatic_identification"],
            false
        );
    }
}
