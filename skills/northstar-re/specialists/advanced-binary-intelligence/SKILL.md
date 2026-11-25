---
name: advanced-binary-intelligence
description: Static classification and corroborated binary understanding for authorized Windows 11 engineering workflows.
---
# Static classification and corroborated binary understanding
## Entry gate
Read `../../references/evidence-contract.md`, `../../references/v7-engineering-workflow.md` and the environment catalog. Verify task-required tools; no assumption of installation, client release or architecture. Work on authorized copies; no protection bypass, credential extraction or unauthorized service access.
## Candidate tool IDs
pe-spec, die, pe-bear, pestudio, yara, rizin, cutter, ida-free, binary-ninja, ghidra, dumpbin, sigcheck. Choose by task and verified readiness; see references/usage.md.
## Procedure
1. Hash authorized copy and inspect PE/CLR metadata, architecture, section bounds, imports/exports, resources, compiler clues and signatures.
2. Corroborate with a second parser; note unsupported or malformed structures and avoid loading unknown binaries. Entropy and packer signatures are hypotheses, not proof of encryption.
3. Map confidence and evidence by RVA, file offset or metadata token; identify ABI constraints and distinguish library code from inferred functionality.
4. Use benign fixture signatures to test classification; do not use detections to evade security products.
## Deliverable
Binary profile and cross-tool discrepancy report.
## Acceptance gate
Known x86/x64 and managed fixture matches expected machine, sections and exports.

## V7 shared workflow
Follow `../../references/v7-engineering-workflow.md`: environment readiness, project evidence, scoped plan, fixture verification, bounded repair and redacted knowledge. Report blocked capabilities and actual test status; no fixed client, language or version.

## Mandatory V8 documentation
Follow `../../references/v8-project-governance.md` and `engineering-documentation`: record material actions, results, redacted evidence, decisions, blockers and handoff in project Markdown. Do not overwrite existing documentation or log secrets.

## V10 engine integration
Read `../../references/v10-engineering-engine.md` before choosing a verified nsre command. Treat the Rust engine as a source candidate until local acceptance; retain the existing tools and evidence contract.
