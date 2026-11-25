# Rust, C++ and C# boundary design: usage references
- Primary: https://doc.rust-lang.org/nomicon/ffi.html
- Secondary: https://learn.microsoft.com/en-us/dotnet/standard/native-interop/best-practices
- Shared workflow: ../../../references/v9-rust-workflow.md

Procedure: Prefer subprocess + versioned JSONL initially. For native ABI use repr(C), explicit sizes, ownership and allocator boundaries, no panic/exception across FFI; test x86/x64 separately. Preserve existing C++ servers.

Acceptance: inspect version/provenance, run a trusted fixture, record actual result and limitations in docs. Do not substitute an estimated performance result for measurement.
