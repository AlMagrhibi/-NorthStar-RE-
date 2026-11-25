---
name: client-server-interoperability
description: Validate compatibility among owned clients, account services, game services and databases.
---
# client-server-interoperability

Mandatory: read `../environment-discovery-setup/SKILL.md`, `../../references/evidence-contract.md`, `../../references/tool-selection.md`, `references/usage.md` and the central tool catalog. Discover and verify required tools before work; install only after scoped approval. Use C, C++ or C# as indicated by actual project evidence.

## Workflow
Use client-compatibility-analysis and cross-version-analysis for inventory and comparison. Map service topology, expected handshake states, feature negotiation, transport, message schemas, DB compatibility and build dependencies from authorized evidence. Never infer that a component is C/C++ from its name alone. Use documented interfaces or test doubles for owned services; no third-party redirection, unauthorized private-service emulation, bypass, ticket forgery or reuse of real authentication traffic. Run localhost integration using synthetic identities and ephemeral data, separate guest for unknown binaries. Do not weaken authentication for compatibility. Record supported versions as observed test results, not assumptions.

## Candidate tool IDs
msvc, dotnet, wireshark, tshark, sqlite, ctest, vstest. Choose only task-required capabilities.

## Acceptance
Record authorization, input hashes when applicable, tool path/version/provenance, actual commands, fixture results, output paths, confidence and limitations. Do not claim runtime verification without execution. Arabic user-facing reports.

## V7 shared workflow
Follow `../../references/v7-engineering-workflow.md`: environment readiness, project evidence, scoped plan, fixture verification, bounded repair and redacted knowledge. Report blocked capabilities and actual test status; no fixed client, language or version.

## Mandatory V8 documentation
Follow `../../references/v8-project-governance.md` and `engineering-documentation`: record material actions, results, redacted evidence, decisions, blockers and handoff in project Markdown. Do not overwrite existing documentation or log secrets.

## V10 shared engine contract
For large file inventories, format hints, evidence exports and hash comparisons when relevant to this specialist, use `../../references/v10-engineering-engine.md` after local acceptance. Do not substitute the engine for specialist analysis or bypass existing evidence and security gates.
