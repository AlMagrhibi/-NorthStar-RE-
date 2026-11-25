---
name: parallel-computing
description: Bounded parallel execution and backpressure for authorized Windows engineering.
---
# Bounded parallel execution and backpressure
Read ../../SKILL.md, ../../references/v7-engineering-workflow.md, ../../references/v8-project-governance.md, ../../references/v9-rust-workflow.md and references/usage.md.
Use bounded work batches and one writer. Tune workers on actual storage; measure 1/2/4/8 workers, avoid oversubscription with Ghidra and builds. Cancellation or error must not produce verified status; use unique run directories.
Use official reviewed tools and synthetic or authorized fixtures. Report exact commands, failures and evidence. No secret extraction, protection bypass or sample execution. Update project Markdown and HANDOFF; no unsupported completion claims.

## V10 engine integration
Read `../../references/v10-engineering-engine.md` before choosing a verified nsre command. Treat the Rust engine as a source candidate until local acceptance; retain the existing tools and evidence contract.
