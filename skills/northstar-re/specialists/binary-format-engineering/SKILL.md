---
name: binary-format-engineering
description: Build evidence-backed, bounds-checked parsers for unknown binary file formats.
---
# binary-format-engineering
Mandatory first: read `../environment-discovery-setup/SKILL.md`, `../../references/evidence-contract.md`, `../../references/tool-selection.md` and local `references/usage.md`. Inventory required capabilities and validate tools with fixtures; do not assume installed.

## Workflow
Inspect a small corpus of authorized files. Label observations (offset, bytes, endianness, length), candidate structures and verified invariants separately. Compare independently labeled samples; model headers, counts, strings and offsets. Use ImHex local patterns for visual verification; use Kaitai .ksy for reproducible parsers and generate C# or C++ with installed compiler help. Prefer .NET BinaryPrimitives and bounded spans or C++ std::span with checked arithmetic; never reinterpret an untrusted buffer as a native struct. Validate truncated, oversized, invalid count, overflow, endian and malformed-string fixtures. Preserve unknown fields, exact source offsets, and schema version. Produce format.md, schema, fixture tests and read-only parser with deterministic JSON output. No assumption of encryption from unfamiliar structures.

## Deliverables
Record scope, SHA-256, tool/version, steps, observed evidence, limitations and validation; write outputs only to the approved workspace. Arabic user-facing report by default. Never execute unknown target content.

## V6 integration
For authorized client/server work, read `../../references/server-workflow.md` and use the master routing for account, protocol, secure transport, game services, database and integration tasks. Preserve project-neutral discovery, tool verification and evidence boundaries.

## V7 shared workflow
Follow `../../references/v7-engineering-workflow.md`: environment readiness, project evidence, scoped plan, fixture verification, bounded repair and redacted knowledge. Report blocked capabilities and actual test status; no fixed client, language or version.

## Mandatory V8 documentation
Follow `../../references/v8-project-governance.md` and `engineering-documentation`: record material actions, results, redacted evidence, decisions, blockers and handoff in project Markdown. Do not overwrite existing documentation or log secrets.

## V10 shared engine contract
For large file inventories, format hints, evidence exports and hash comparisons when relevant to this specialist, use `../../references/v10-engineering-engine.md` after local acceptance. Do not substitute the engine for specialist analysis or bypass existing evidence and security gates.
