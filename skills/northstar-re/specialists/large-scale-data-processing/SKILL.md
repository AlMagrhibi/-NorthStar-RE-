---
name: large-scale-data-processing
description: Incremental index and bounded corpus processing for authorized Windows engineering.
---
# Incremental index and bounded corpus processing
Read ../../SKILL.md, ../../references/v7-engineering-workflow.md, ../../references/v8-project-governance.md, ../../references/v9-rust-workflow.md and references/usage.md.
Design project-local SQLite index keyed by normalized path, identity, size, timestamps, schema and scanner version; metadata reuse is provisional, full rehash required for verified evidence. Use transactions, run IDs, resumable batches, invalidation on change and output outside corpus. Cache/resume not implemented in supplied prototype.
Use official reviewed tools and synthetic or authorized fixtures. Report exact commands, failures and evidence. No secret extraction, protection bypass or sample execution. Update project Markdown and HANDOFF; no unsupported completion claims.

## V10 engine integration
Read `../../references/v10-engineering-engine.md` before choosing a verified nsre command. Treat the Rust engine as a source candidate until local acceptance; retain the existing tools and evidence contract.
