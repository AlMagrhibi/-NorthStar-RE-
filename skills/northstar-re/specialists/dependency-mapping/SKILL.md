---
name: dependency-mapping
description: Dependency graph for client, server, SDK and data for authorized Windows 11 engineering workflows.
---
# Dependency graph for client, server, SDK and data
## Entry gate
Read `../../references/evidence-contract.md`, `../../references/v7-engineering-workflow.md` and the environment catalog. Verify task-required tools; no assumption of installation, client release or architecture. Work on authorized copies; no protection bypass, credential extraction or unauthorized service access.
## Candidate tool IDs
dependencies, dumpbin, process-explorer, msbuild, dotnet, vcpkg, nuget. Choose by task and verified readiness; see references/usage.md.
## Procedure
1. Inventory static imports, managed references, project files, lockfiles, SDKs and database/service contracts.
2. Record edges with evidence and status; distinguish optional, dynamic, transitive, missing and unresolved edges.
3. Verify architecture, runtime and version constraints; observe loads only in approved owned fixtures.
4. Identify cycles and incompatible dependencies; prioritize fixes without arbitrary downloads or replacing system DLLs.
## Deliverable
Machine-readable dependency edge list and remediation plan.
## Acceptance gate
Known dependency fixture and missing-dependency negative case.

## V7 shared workflow
Follow `../../references/v7-engineering-workflow.md`: environment readiness, project evidence, scoped plan, fixture verification, bounded repair and redacted knowledge. Report blocked capabilities and actual test status; no fixed client, language or version.

## Mandatory V8 documentation
Follow `../../references/v8-project-governance.md` and `engineering-documentation`: record material actions, results, redacted evidence, decisions, blockers and handoff in project Markdown. Do not overwrite existing documentation or log secrets.

## V10 shared engine contract
For large file inventories, format hints, evidence exports and hash comparisons when relevant to this specialist, use `../../references/v10-engineering-engine.md` after local acceptance. Do not substitute the engine for specialist analysis or bypass existing evidence and security gates.
