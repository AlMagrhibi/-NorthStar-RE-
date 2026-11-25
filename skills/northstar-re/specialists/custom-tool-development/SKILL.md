---
name: custom-tool-development
description: Build missing read-only analysis, conversion and validation tools in Visual Studio C++/C#.
---
# custom-tool-development
Mandatory first: read `../environment-discovery-setup/SKILL.md`, `../../references/evidence-contract.md`, `../../references/tool-selection.md` and local `references/usage.md`. Inventory required capabilities and validate tools with fixtures; do not assume installed.

## Workflow
Read visual-studio-dev and binary-format-engineering first. Define input/output contract and license boundary; implement a minimal read-only CLI with clear exit codes, explicit paths, bounded memory, checked arithmetic, safe file handling and structured diagnostics. Prefer C# for managed data/CLI and C++ for native integration where justified. Build in detected Visual Studio toolchain, use CMake/CTest or dotnet test as appropriate; pin approved dependencies. Test valid and malformed synthetic data, fuzz parser locally with bounded inputs, verify deterministic outputs and no overwrite of originals. Provide source, build instructions, tests, license/dependency list, example synthetic fixture, and versioned schema. Never implement protection bypass, injection or key extraction.

## Deliverables
Record scope, SHA-256, tool/version, steps, observed evidence, limitations and validation; write outputs only to the approved workspace. Arabic user-facing report by default. Never execute unknown target content.

## V6 integration
For authorized client/server work, read `../../references/server-workflow.md` and use the master routing for account, protocol, secure transport, game services, database and integration tasks. Preserve project-neutral discovery, tool verification and evidence boundaries.

## V7 shared workflow
Follow `../../references/v7-engineering-workflow.md`: environment readiness, project evidence, scoped plan, fixture verification, bounded repair and redacted knowledge. Report blocked capabilities and actual test status; no fixed client, language or version.

## Mandatory V8 documentation
Follow `../../references/v8-project-governance.md` and `engineering-documentation`: record material actions, results, redacted evidence, decisions, blockers and handoff in project Markdown. Do not overwrite existing documentation or log secrets.

## V10 shared engine contract
For large file inventories, format hints, evidence exports and hash comparisons when relevant to this specialist, use `../../references/v10-engineering-engine.md` after local acceptance. Do not substitute the engine for specialist analysis or bypass existing evidence and security gates.
