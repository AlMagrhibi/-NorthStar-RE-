# Engineering validation lab — FW-011 through FW-020

Development candidate, not a published release. The existing master and 52 specialists remain unchanged in count. The new offline Rust lab consumes reviewed local JSON (the existing 16 MiB input cap); it does not launch processes, inspect target executables, alter firewalls, or connect to services.

Run: nsre lab --input FILE --reviewed-redacted. Exit 0 = operation returned passed=true; 2 = negative result; 1 = malformed input. Each check has limited meaning, not end-to-end acceptance. Keep project data, secrets and private artifacts local.

| ID | Candidate operation | Actual scope | Remaining acceptance |
|---|---|---|---|
| FW-011 Algorithm Intelligence Engine | algorithm-catalog | SHA-256 reference catalog | General identification, confidence and independent evaluation |
| FW-012 Cryptographic Validation Lab | sha256-kat | Known-answer SHA-256 digest check | More public vectors and modern algorithms |
| FW-013 Assembly Intelligence Engine | assembly-summary | Counts externally supplied instruction categories | Generic instruction schema and indexed analysis on synthetic binaries |
| FW-014 Evidence Correlation Engine | evidence-check | Validates record IDs, digest syntax, confidence and references | Recompute artifact digests and persist provenance graph |
| FW-015 Format Round-trip Testing | json-roundtrip | JSON value serialization/deserialization | Independent open-format codecs and byte-level comparisons |
| FW-016 Runtime Compatibility Testing Framework | gate-summary | Aggregates reported gates; no live verification | Run independent fixtures in isolated lab |
| FW-017 Isolated Execution & Diagnostics Manager | isolation-plan | Checks declared isolation prerequisites only | Actual isolation, diagnostics, rollback and OS acceptance |
| FW-018 Protocol Schema & Version Management | protocol-check | Flat required-field types and exact version equality | Versioned registry, compatibility policy and owned protocol fixtures |
| FW-019 Configuration Compatibility Validator | config-check | Flat required-field validation and unknown-field detection | Nested schemas, optional fields, format-specific validation |
| FW-020 Automated Integration Regression Engine | regression-check | JSON baseline equality | Test runner, artifact verification and integration reports |

## Example inputs

Digest fixture (public known answer):
```json
{"operation":"sha256-kat","utf8":"abc","expected_sha256":"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"}
```

Configuration fixture:
```json
{"operation":"config-check","fields":{"enabled":"boolean","port":"integer"},"data":{"enabled":true,"port":1234}}
```

Protocol fixture:
```json
{"operation":"protocol-check","version":"1","expected_version":"1","fields":{"id":"integer"},"data":{"id":1}}
```

Evidence records: each requires id, sha256 (64 hexadecimal characters), confidence (observed/inference/unknown), producer, and references (array of IDs in the same input). Gate records require id and status (passed/blocked/failed/unverified); passed requires evidence_sha256. These checks validate metadata, not the underlying artifacts. Isolation-plan expects disposable_workspace, network_isolated, no_shared_credentials and rollback_defined booleans; enforcement is always false.

## Release gates

All ten are partial foundations, not complete engines. Run locked Rust check/test/clippy/fmt, package validation and Windows installer lifecycle in CI. Validate live host behavior separately. Never promote the candidate solely because source exists or a metadata check passes. Preserve RC2 and avoid putting target-specific artifacts in this public repository.
