---
name: source-code-reconstruction
description: Clean-room compatible implementation for authorized software for authorized Windows 11 engineering workflows.
---
# Clean-room compatible implementation for authorized software
## Entry gate
Read `../../references/evidence-contract.md`, `../../references/v7-engineering-workflow.md` and the environment catalog. Verify task-required tools; no assumption of installation, client release or architecture. Work on authorized copies; no protection bypass, credential extraction or unauthorized service access.
## Candidate tool IDs
ghidra, ilspy, llvm-clang, doxygen, googletest, xunit. Choose by task and verified readiness; see references/usage.md.
## Procedure
1. Document independently observable behavior and public interface, provenance, permitted scope and limitations.
2. Write original implementation against specification using appropriate C/C++/C#; never label decompiler pseudocode as recovered original source.
3. Define ABI, error handling and input bounds; preserve license and separate third-party material.
4. Build and run golden and negative fixtures, compare results and report remaining unknowns.
## Deliverable
Original source, behavior specification, build and tests.
## Acceptance gate
Fixture equivalence is demonstrated; no protected feature bypass or unauthorized copying.

## V7 shared workflow
Follow `../../references/v7-engineering-workflow.md`: environment readiness, project evidence, scoped plan, fixture verification, bounded repair and redacted knowledge. Report blocked capabilities and actual test status; no fixed client, language or version.

## Mandatory V8 documentation
Follow `../../references/v8-project-governance.md` and `engineering-documentation`: record material actions, results, redacted evidence, decisions, blockers and handoff in project Markdown. Do not overwrite existing documentation or log secrets.

## V10 shared engine contract
For large file inventories, format hints, evidence exports and hash comparisons when relevant to this specialist, use `../../references/v10-engineering-engine.md` after local acceptance. Do not substitute the engine for specialist analysis or bypass existing evidence and security gates.
