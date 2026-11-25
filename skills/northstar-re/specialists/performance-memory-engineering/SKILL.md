---
name: performance-memory-engineering
description: Reproducible performance and memory profiling for authorized Windows 11 engineering workflows.
---
# Reproducible performance and memory profiling
## Entry gate
Read `../../references/evidence-contract.md`, `../../references/v7-engineering-workflow.md` and the environment catalog. Verify task-required tools; no assumption of installation, client release or architecture. Work on authorized copies; no protection bypass, credential extraction or unauthorized service access.
## Candidate tool IDs
wpr, wpa, asan, dotnet-diagnostics, process-explorer, windbg, pix. Choose by task and verified readiness; see references/usage.md.
## Procedure
1. Define workload, baseline, warm-up, duration, repeat count and metrics before profiling.
2. Use WPR/WPA for local ETW traces, managed counters for owned .NET processes, sanitizer builds for native test code.
3. Separate latency, throughput, CPU, allocation, GPU and IO metrics; compare identical workload and environment.
4. Investigate regressions using minimal traces, document overhead and uncertainty; stop collection at budget.
## Deliverable
Baseline, repeat measurements, regression analysis and fix verification.
## Acceptance gate
Repeatable owned benchmark and clean sanitizer fixture; no real credentials in traces.

## V7 shared workflow
Follow `../../references/v7-engineering-workflow.md`: environment readiness, project evidence, scoped plan, fixture verification, bounded repair and redacted knowledge. Report blocked capabilities and actual test status; no fixed client, language or version.

## Mandatory V8 documentation
Follow `../../references/v8-project-governance.md` and `engineering-documentation`: record material actions, results, redacted evidence, decisions, blockers and handoff in project Markdown. Do not overwrite existing documentation or log secrets.

## V10 shared engine contract
For large file inventories, format hints, evidence exports and hash comparisons when relevant to this specialist, use `../../references/v10-engineering-engine.md` after local acceptance. Do not substitute the engine for specialist analysis or bypass existing evidence and security gates.
