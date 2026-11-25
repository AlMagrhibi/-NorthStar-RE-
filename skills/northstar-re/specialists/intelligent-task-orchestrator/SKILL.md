---
name: intelligent-task-orchestrator
description: Evidence-based task routing and bounded recovery for authorized Windows 11 engineering workflows.
---
# Evidence-based task routing and bounded recovery
## Entry gate
Read `../../references/evidence-contract.md`, `../../references/v7-engineering-workflow.md` and the environment catalog. Verify task-required tools; no assumption of installation, client release or architecture. Work on authorized copies; no protection bypass, credential extraction or unauthorized service access.
## Candidate tool IDs
git, rust. Choose by task and verified readiness; see references/usage.md.
## Procedure
1. Establish authorized scope, workspace, baseline and user acceptance criteria. Split the task into small work packages with dependencies and deliverables.
2. Classify each package as environment, static analysis, source development, isolated runtime, server or validation. Select specialists by evidence, not file extension alone.
3. Run independent read-only tasks in parallel only when supported; serialize writes and installations. Give each worker scoped inputs and separate outputs.
4. Track status planned/running/blocked/verified; one retry after diagnosing a failure, then change the hypothesis or report the blocker. Never mark a package verified without artifacts.
## Deliverable
Task DAG, route reasons, owners, blockers and gate evidence.
## Acceptance gate
All dependent nodes require verified predecessors; explicit approvals for installation or runtime.

## V7 shared workflow
Follow `../../references/v7-engineering-workflow.md`: environment readiness, project evidence, scoped plan, fixture verification, bounded repair and redacted knowledge. Report blocked capabilities and actual test status; no fixed client, language or version.

## Mandatory V8 documentation
Follow `../../references/v8-project-governance.md` and `engineering-documentation`: record material actions, results, redacted evidence, decisions, blockers and handoff in project Markdown. Do not overwrite existing documentation or log secrets.

## V10 shared engine contract
For large file inventories, format hints, evidence exports and hash comparisons when relevant to this specialist, use `../../references/v10-engineering-engine.md` after local acceptance. Do not substitute the engine for specialist analysis or bypass existing evidence and security gates.
