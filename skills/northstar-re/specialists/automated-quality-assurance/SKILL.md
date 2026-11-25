---
name: automated-quality-assurance
description: Layered verification and regression prevention for authorized Windows 11 engineering workflows.
---
# Layered verification and regression prevention
## Entry gate
Read `../../references/evidence-contract.md`, `../../references/v7-engineering-workflow.md` and the environment catalog. Verify task-required tools; no assumption of installation, client release or architecture. Work on authorized copies; no protection bypass, credential extraction or unauthorized service access.
## Candidate tool IDs
googletest, xunit, ctest, vstest, dotnet, asan, rust. Choose by task and verified readiness; see references/usage.md.
## Procedure
1. Define acceptance matrix and test oracle before changing implementation.
2. Use unit, malformed-input, bounds, integration, ABI compatibility and regression tests on synthetic fixtures.
3. Record command, environment, exit code, expected and observed result; distinguish skipped and blocked from passed.
4. Gate release on required tests and evidence; diagnose failures and rerun after a scoped fix.
## Deliverable
Test matrix, machine-readable outcomes and release decision.
## Acceptance gate
Any required failed, skipped or unexecuted test blocks production-ready claims.

## V7 shared workflow
Follow `../../references/v7-engineering-workflow.md`: environment readiness, project evidence, scoped plan, fixture verification, bounded repair and redacted knowledge. Report blocked capabilities and actual test status; no fixed client, language or version.

## Mandatory V8 documentation
Follow `../../references/v8-project-governance.md` and `engineering-documentation`: record material actions, results, redacted evidence, decisions, blockers and handoff in project Markdown. Do not overwrite existing documentation or log secrets.

## V10 engine integration
Read `../../references/v10-engineering-engine.md` before choosing a verified nsre command. Treat the Rust engine as a source candidate until local acceptance; retain the existing tools and evidence contract.
