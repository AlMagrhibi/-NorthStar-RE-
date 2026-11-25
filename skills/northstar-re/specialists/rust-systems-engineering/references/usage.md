# Rust/Cargo architecture and reproducible builds: usage references
- Primary: https://doc.rust-lang.org/book/
- Secondary: https://doc.rust-lang.org/cargo/
- Shared workflow: ../../../references/v9-rust-workflow.md

Procedure: Use stable MSVC target on Windows; discover rustup, rustc, Cargo and linker. Build a reviewed fixture, record target and compiler. Separate library and CLI, use typed errors; minimize unsafe. Pin resolved dependencies with Cargo.lock after resolution; review licenses and RustSec.

Acceptance: inspect version/provenance, run a trusted fixture, record actual result and limitations in docs. Do not substitute an estimated performance result for measurement.
