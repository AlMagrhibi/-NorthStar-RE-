---
name: database-engineering
description: Inspect and design owned account/world schemas and safe migrations.
---
# database-engineering

Mandatory: read `../environment-discovery-setup/SKILL.md`, `../../references/evidence-contract.md`, `../../references/tool-selection.md`, `references/usage.md` and the central tool catalog. Discover and verify required tools before work; install only after scoped approval. Use C, C++ or C# as indicated by actual project evidence.

## Workflow
Identify actual DB engine from authorized configuration, schema and connector dependencies; do not assume MySQL. Redact credentials and personal data. Use least-privilege read-only access for inspection; prefer disposable database or approved sanitized backup. Document schema, keys, constraints, indexes, transactions and migration history. Use parameterized queries, secure password hashes, secret storage and separate roles. Back up and verify restore before an approved migration; never mutate live DB without separate approval. Validate schema drift, duplicate IDs, foreign keys, rollback and concurrent transactions with synthetic data. Never dump password hashes or real account records.

## Candidate tool IDs
sqlite, mysql, mysqlsh, mysql-workbench, sqlcmd, ssms, psql, dotnet. Choose only task-required capabilities.

## Acceptance
Record authorization, input hashes when applicable, tool path/version/provenance, actual commands, fixture results, output paths, confidence and limitations. Do not claim runtime verification without execution. Arabic user-facing reports.

## V7 shared workflow
Follow `../../references/v7-engineering-workflow.md`: environment readiness, project evidence, scoped plan, fixture verification, bounded repair and redacted knowledge. Report blocked capabilities and actual test status; no fixed client, language or version.

## Mandatory V8 documentation
Follow `../../references/v8-project-governance.md` and `engineering-documentation`: record material actions, results, redacted evidence, decisions, blockers and handoff in project Markdown. Do not overwrite existing documentation or log secrets.

## V10 shared engine contract
For large file inventories, format hints, evidence exports and hash comparisons when relevant to this specialist, use `../../references/v10-engineering-engine.md` after local acceptance. Do not substitute the engine for specialist analysis or bypass existing evidence and security gates.
