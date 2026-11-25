---
name: binary-diffing-version-migration
description: Version-aware binary and schema comparison for authorized Windows 11 engineering workflows.
---
# Version-aware binary and schema comparison
## Entry gate
Read `../../references/evidence-contract.md`, `../../references/v7-engineering-workflow.md` and the environment catalog. Verify task-required tools; no assumption of installation, client release or architecture. Work on authorized copies; no protection bypass, credential extraction or unauthorized service access.
## Candidate tool IDs
diaphora, ghidra, rizin, git, imhex, dumpbin. Choose by task and verified readiness; see references/usage.md.
## Procedure
1. Hash both copies, record build metadata and normalize comparison using symbols or public API contracts where available.
2. Compare exports, imports, section metadata, resource identifiers, data schemas and semantic changes using local tools.
3. Separate layout-only changes from behavioral changes; confirm candidate differences against synthetic test vectors.
4. Create migration plan and rollback with compatibility matrix; never patch third-party protections or licensing.
## Deliverable
Version delta, compatibility matrix and migration tests.
## Acceptance gate
Two locally built revisions show known API and behavior differences.

## V7 shared workflow
Follow `../../references/v7-engineering-workflow.md`: environment readiness, project evidence, scoped plan, fixture verification, bounded repair and redacted knowledge. Report blocked capabilities and actual test status; no fixed client, language or version.

## Mandatory V8 documentation
Follow `../../references/v8-project-governance.md` and `engineering-documentation`: record material actions, results, redacted evidence, decisions, blockers and handoff in project Markdown. Do not overwrite existing documentation or log secrets.

## V10 engine integration
Read `../../references/v10-engineering-engine.md` before choosing a verified nsre command. Treat the Rust engine as a source candidate until local acceptance; retain the existing tools and evidence contract.
