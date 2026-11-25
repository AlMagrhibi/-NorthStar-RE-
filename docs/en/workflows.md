# Engineering workflows and multi-project operation

[Documentation index](../README.md) · [العربية](../ar/workflows.md)

## Authorized analysis
1. Confirm scope, discover environment and smoke-test required tools.
2. Preserve an authorized input copy; compute strict hashes.
3. Select specialists based on actual file/service evidence.
4. Analyze with bounded native tools; use Java for Ghidra automation.
5. Corroborate findings, record source hash/location/confidence/limitations.
6. Update project docs, reviewed memory, session handoff and quality gates.

## Multi-project use
One central engine can serve many explicit --project directories. Each owns its docs, state, evidence and checkpoints. Do not mix index DBs across roots. File scan output and index DB must be outside their corpus. Close conflicting writes and use trusted stable paths; there is no tenant security boundary against another process with the same OS permissions.

## Planning and recovery
jobs validates/saves a task DAG and progress checks quality evidence. It does not execute a scheduler. After a crash inspect running/partial runs, rerun strict indexing and review committed state; do not claim exact resume. Installation backup/rollback concerns only framework registration and central engine, never project contents.
