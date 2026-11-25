---
name: build-release-engineering
description: Reproducible builds, dependencies, release and rollback for authorized Windows 11 engineering workflows.
---
# Reproducible builds, dependencies, release and rollback
## Entry gate
Read `../../references/evidence-contract.md`, `../../references/v7-engineering-workflow.md` and the environment catalog. Verify task-required tools; no assumption of installation, client release or architecture. Work on authorized copies; no protection bypass, credential extraction or unauthorized service access.
## Candidate tool IDs
git, vswhere, msbuild, msvc, cmake, ninja, dotnet, llvm-clang, vcpkg, nuget, doxygen, 7zip. Choose by task and verified readiness; see references/usage.md.
## Procedure
1. Discover project build system, compiler, architecture, SDK and lockfiles; check developer shell.
2. Use reviewed manifests, pinned dependency versions and separate build outputs; do not change system packages without approval.
3. Build Debug/Release and supported architectures, capture logs and run tests. Produce SHA-256 manifest and dependency/license notes.
4. Create versioned archive and verify unpacking in clean workspace; retain rollback instructions and previous artifact.
## Deliverable
Build log, versioned package, hashes, license notes and rollback plan.
## Acceptance gate
Clean build and fixture tests succeed for each claimed configuration.

## V7 shared workflow
Follow `../../references/v7-engineering-workflow.md`: environment readiness, project evidence, scoped plan, fixture verification, bounded repair and redacted knowledge. Report blocked capabilities and actual test status; no fixed client, language or version.

## Mandatory V8 documentation
Follow `../../references/v8-project-governance.md` and `engineering-documentation`: record material actions, results, redacted evidence, decisions, blockers and handoff in project Markdown. Do not overwrite existing documentation or log secrets.

## V10 shared engine contract
For large file inventories, format hints, evidence exports and hash comparisons when relevant to this specialist, use `../../references/v10-engineering-engine.md` after local acceptance. Do not substitute the engine for specialist analysis or bypass existing evidence and security gates.
