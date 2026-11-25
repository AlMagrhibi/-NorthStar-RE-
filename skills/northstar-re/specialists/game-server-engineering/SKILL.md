---
name: game-server-engineering
description: Develop and diagnose owned game/world services in C, C++ and C# without assuming a game version.
---
# game-server-engineering

Mandatory: read `../environment-discovery-setup/SKILL.md`, `../../references/evidence-contract.md`, `../../references/tool-selection.md`, `references/usage.md` and the central tool catalog. Discover and verify required tools before work; install only after scoped approval. Use C, C++ or C# as indicated by actual project evidence.

## Workflow
Map actual services (world/game, account, optional NPC or matchmaking, persistence) and ownership. Discover build configuration, architecture, dependencies and data schemas; no fixed ports, names or version. Design server-authoritative validation, bounded queues, cancellation, connection lifecycle, deterministic state transitions and safe persistence. Use explicit integer widths and checked parsing; model concurrency, transaction boundaries and graceful shutdown. Implement modular interfaces and tests in trusted source. Use synthetic players and isolated local instances; never target a third-party service, abuse gameplay, or modify production state without approval. Record resource budgets, config defaults and rollback.

## Candidate tool IDs
msvc, dotnet, cmake, ctest, vstest, winsock, windbg, procmon. Choose only task-required capabilities.

## Acceptance
Record authorization, input hashes when applicable, tool path/version/provenance, actual commands, fixture results, output paths, confidence and limitations. Do not claim runtime verification without execution. Arabic user-facing reports.

## V7 shared workflow
Follow `../../references/v7-engineering-workflow.md`: environment readiness, project evidence, scoped plan, fixture verification, bounded repair and redacted knowledge. Report blocked capabilities and actual test status; no fixed client, language or version.

## Mandatory V8 documentation
Follow `../../references/v8-project-governance.md` and `engineering-documentation`: record material actions, results, redacted evidence, decisions, blockers and handoff in project Markdown. Do not overwrite existing documentation or log secrets.

## V10 shared engine contract
For large file inventories, format hints, evidence exports and hash comparisons when relevant to this specialist, use `../../references/v10-engineering-engine.md` after local acceptance. Do not substitute the engine for specialist analysis or bypass existing evidence and security gates.
