# Windows paths, diagnostics and MSVC interoperability: usage references
- Primary: https://doc.rust-lang.org/rustc/platform-support/windows-msvc.html
- Secondary: https://microsoft.github.io/windows-docs-rs/
- Shared workflow: ../../../references/v9-rust-workflow.md

Procedure: Use documented Windows APIs for reparse point checks, Unicode and long paths. Discover x86/x64/ARM64 capabilities; do not infer target architecture from host. Diagnose trusted fixtures only; never load unknown DLLs.

Acceptance: inspect version/provenance, run a trusted fixture, record actual result and limitations in docs. Do not substitute an estimated performance result for measurement.
