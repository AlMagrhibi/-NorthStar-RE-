# Streaming file inventory and SHA-256: usage references
- Primary: https://docs.rs/walkdir/
- Secondary: https://docs.rs/sha2/
- Shared workflow: ../../../references/v9-rust-workflow.md

Procedure: Use nsre scan on authorized immutable input; output outside source. Bound jobs and buffers; skip reparse points, report errors and incomplete runs. Compare path, size and SHA-256 against prior Python baseline; never use cache for final evidence hashes.

Acceptance: inspect version/provenance, run a trusted fixture, record actual result and limitations in docs. Do not substitute an estimated performance result for measurement.
