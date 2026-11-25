---
name: rust-cpp-interoperability
description: Rust, C++ and C# boundary design for authorized Windows engineering.
---
# Rust, C++ and C# boundary design
Read ../../SKILL.md, ../../references/v7-engineering-workflow.md, ../../references/v8-project-governance.md, ../../references/v9-rust-workflow.md and references/usage.md.
Prefer subprocess + versioned JSONL initially. For native ABI use repr(C), explicit sizes, ownership and allocator boundaries, no panic/exception across FFI; test x86/x64 separately. Preserve existing C++ servers.
Use official reviewed tools and synthetic or authorized fixtures. Report exact commands, failures and evidence. No secret extraction, protection bypass or sample execution. Update project Markdown and HANDOFF; no unsupported completion claims.

## V10 engine integration
Read `../../references/v10-engineering-engine.md` before choosing a verified nsre command. Treat the Rust engine as a source candidate until local acceptance; retain the existing tools and evidence contract.
