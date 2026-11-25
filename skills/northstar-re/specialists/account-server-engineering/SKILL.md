---
name: account-server-engineering
description: Design, inspect and test authorized account/login services in C, C++ or C#.
---
# account-server-engineering

Mandatory: read `../environment-discovery-setup/SKILL.md`, `../../references/evidence-contract.md`, `../../references/tool-selection.md`, `references/usage.md` and the central tool catalog. Discover and verify required tools before work; install only after scoped approval. Use C, C++ or C# as indicated by actual project evidence.

## Workflow
Inventory source projects, binaries, configuration, DB migrations and service dependencies. Determine authentication boundaries from evidence, not game branding. Document account identity, credential verification, session issuance, expiry, revocation, role checks and handoff to the game service. Keep authentication separate from world logic. Use established identity libraries or a reviewed password-hashing implementation; salted adaptive hashes, rate limits, generic errors, secure secret storage, session rotation and audited revocation. Never log passwords, tokens or raw authentication payloads. For owned service migration, preserve account IDs and schema compatibility; test with synthetic accounts, not real credentials. Do not bypass third-party login, forge tickets, recover credentials or impersonate users.

## Candidate tool IDs
msvc, dotnet, msbuild, winsock, aspnet-identity, windbg, sqlite. Choose only task-required capabilities.

## Acceptance
Record authorization, input hashes when applicable, tool path/version/provenance, actual commands, fixture results, output paths, confidence and limitations. Do not claim runtime verification without execution. Arabic user-facing reports.

## V7 shared workflow
Follow `../../references/v7-engineering-workflow.md`: environment readiness, project evidence, scoped plan, fixture verification, bounded repair and redacted knowledge. Report blocked capabilities and actual test status; no fixed client, language or version.

## Mandatory V8 documentation
Follow `../../references/v8-project-governance.md` and `engineering-documentation`: record material actions, results, redacted evidence, decisions, blockers and handoff in project Markdown. Do not overwrite existing documentation or log secrets.

## V10 shared engine contract
For large file inventories, format hints, evidence exports and hash comparisons when relevant to this specialist, use `../../references/v10-engineering-engine.md` after local acceptance. Do not substitute the engine for specialist analysis or bypass existing evidence and security gates.
