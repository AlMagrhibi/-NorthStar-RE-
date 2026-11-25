# Architecture and module boundaries

[Documentation index](../README.md) · [العربية](../ar/architecture.md)

## Layout decision
The authoritative Cargo workspace stays under skills/northstar-re/engine/nsre to preserve embedded template paths and installer contracts. Root engine/ contains benchmark tooling and a source-location guide. No duplicate engine source or empty decorative module folders are created.

## Master and router
The master routes tasks by observed binary, language and service evidence. Specialists are instruction modules, not independent processes. No automated worker scheduler or agent delegation is implied. The installed skill exposes only the master; guides remain addressable internally.

## Rust modules
main.rs handles scan, CLI dispatch and hashing; index.rs implements a bounded Rayon pipeline and one SQLite writer; probe.rs reads at most 64 KiB plus a 24-byte PE header; framework.rs owns documentation, evidence, gates, plans, sessions and knowledge; rpc.rs implements local newline JSON-RPC and an MCP candidate.

## Isolation and memory
Every project explicitly binds --project DIR and owns .northstar/state/framework.sqlite. Index DBs must be outside the corpus. Parameterized SQL, application IDs and schema checks reject unrelated DBs. WAL and FULL synchronous commits protect completed transactions; interrupted index runs remain running and are not valid strict evidence. There is no exact worker resume. Keep stable trusted directories; filesystem checks do not eliminate hostile concurrent path replacement.

## Evidence and gates
SHA-256 verifies bytes, not truth or publisher identity. Nine quality gates require reasons and matching file evidence; unchanged documentation templates block completion. Knowledge is append-only, reviewed and redacted, with confidence and limitations. Checkpoints copy explicit files and require reviewed manual restoration.
