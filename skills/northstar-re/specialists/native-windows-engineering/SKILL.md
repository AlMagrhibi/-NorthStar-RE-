---
name: native-windows-engineering
description: Windows paths, diagnostics and MSVC interoperability for authorized Windows engineering.
---
# Windows paths, diagnostics and MSVC interoperability
Read ../../SKILL.md, ../../references/v7-engineering-workflow.md, ../../references/v8-project-governance.md, ../../references/v9-rust-workflow.md and references/usage.md.
Use documented Windows APIs for reparse point checks, Unicode and long paths. Discover x86/x64/ARM64 capabilities; do not infer target architecture from host. Diagnose trusted fixtures only; never load unknown DLLs.
Use official reviewed tools and synthetic or authorized fixtures. Report exact commands, failures and evidence. No secret extraction, protection bypass or sample execution. Update project Markdown and HANDOFF; no unsupported completion claims.

## V10 engine integration
Read `../../references/v10-engineering-engine.md` before choosing a verified nsre command. Treat the Rust engine as a source candidate until local acceptance; retain the existing tools and evidence contract.
