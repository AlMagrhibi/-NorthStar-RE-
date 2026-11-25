# Incremental index and bounded corpus processing: usage references
- Primary: https://docs.rs/rusqlite/
- Secondary: https://www.sqlite.org/lang_transaction.html
- Shared workflow: ../../../references/v9-rust-workflow.md

Procedure: Design project-local SQLite index keyed by normalized path, identity, size, timestamps, schema and scanner version; metadata reuse is provisional, full rehash required for verified evidence. Use transactions, run IDs, resumable batches, invalidation on change and output outside corpus. Cache/resume not implemented in supplied prototype.

Acceptance: inspect version/provenance, run a trusted fixture, record actual result and limitations in docs. Do not substitute an estimated performance result for measurement.
