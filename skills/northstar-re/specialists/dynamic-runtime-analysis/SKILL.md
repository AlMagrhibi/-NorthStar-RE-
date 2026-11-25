---
name: dynamic-runtime-analysis
description: Isolated runtime observation of authorized fixtures for authorized Windows 11 engineering workflows.
---
# Isolated runtime observation of authorized fixtures
## Entry gate
Read `../../references/evidence-contract.md`, `../../references/v7-engineering-workflow.md` and the environment catalog. Verify task-required tools; no assumption of installation, client release or architecture. Work on authorized copies; no protection bypass, credential extraction or unauthorized service access.
## Candidate tool IDs
x64dbg, dnspyex, windbg, procmon, process-explorer, winobj, api-monitor, sandbox. Choose by task and verified readiness; see references/usage.md.
## Procedure
1. Prefer source debugging and static analysis; use only owned test applications or explicitly authorized samples in disposable offline VM.
2. Set baseline and resource/time limits; use synthetic inputs, no real accounts or production services. Disable guest sharing/network for untrusted material.
3. Observe exceptions, module load, call behavior and file operations; preserve minimal redacted traces. Do not capture credential-bearing memory or circumvent protections.
4. Stop on unexpected network, privilege or integrity changes; restore VM snapshot and compare observed output with fixture expectations.
## Deliverable
Reproducible runtime trace with scope and observations.
## Acceptance gate
Known test program returns expected result; no unauthorized process attachment or network.

## V7 shared workflow
Follow `../../references/v7-engineering-workflow.md`: environment readiness, project evidence, scoped plan, fixture verification, bounded repair and redacted knowledge. Report blocked capabilities and actual test status; no fixed client, language or version.

## Mandatory V8 documentation
Follow `../../references/v8-project-governance.md` and `engineering-documentation`: record material actions, results, redacted evidence, decisions, blockers and handoff in project Markdown. Do not overwrite existing documentation or log secrets.

## V10 shared engine contract
For large file inventories, format hints, evidence exports and hash comparisons when relevant to this specialist, use `../../references/v10-engineering-engine.md` after local acceptance. Do not substitute the engine for specialist analysis or bypass existing evidence and security gates.
