# Bounded parallel execution and backpressure: usage references
- Primary: https://docs.rs/rayon/
- Secondary: https://doc.rust-lang.org/std/thread/
- Shared workflow: ../../../references/v9-rust-workflow.md

Procedure: Use bounded work batches and one writer. Tune workers on actual storage; measure 1/2/4/8 workers, avoid oversubscription with Ghidra and builds. Cancellation or error must not produce verified status; use unique run directories.

Acceptance: inspect version/provenance, run a trusted fixture, record actual result and limitations in docs. Do not substitute an estimated performance result for measurement.
