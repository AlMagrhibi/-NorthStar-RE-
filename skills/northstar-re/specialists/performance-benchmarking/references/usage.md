# Reproducible migration benchmarks: usage references
- Primary: https://bheisler.github.io/criterion.rs/book/
- Secondary: https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.utility/measure-command
- Shared workflow: ../../../references/v9-rust-workflow.md

Procedure: Measure same corpus, same coverage, same errors, same storage and versions; distinguish cold/warm and incremental. Record wall time, file count, bytes, throughput, CPU and peak RSS where available, repeated runs and medians. A speed claim requires output equivalence; keep native tool alternative.

Acceptance: inspect version/provenance, run a trusted fixture, record actual result and limitations in docs. Do not substitute an estimated performance result for measurement.
