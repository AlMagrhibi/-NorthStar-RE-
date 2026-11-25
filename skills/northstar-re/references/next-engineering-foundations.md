# Next engineering foundations — approved work, not a released capability

Status: development candidate. Keep the installed RC2 and project data intact until a new package passes CI, installer acceptance and explicit upgrade. Do not assume a particular application, client, dataset or protocol. No target binaries, recovered code, project databases, credentials, protected data or private paths belong in the public framework repository.

## FW-001 — tool discovery
Candidate source: explicit per-process NSRE_TOOL_<UPPERCASE_NAME> absolute file path, then absolute PATH entries, then bounded installation environment hints (CARGO_HOME, JAVA_HOME, GHIDRA_HOME, VSINSTALLDIR, VCToolsInstallDir and common ProgramFiles paths). Invalid explicit overrides fail closed, not fall back. A candidate is never a verified executable. Verify provenance, version, architecture and a safe smoke test separately. Do not execute a target or untrusted binary. Windows registry/vswhere enumeration remains planned. Test explicit precedence, invalid overrides, PATH, hints, missing tools, Unicode paths and Windows hosts.

## FW-002 — incremental indexing
Strict mode must continue hashing every file; never mark cached metadata evidence-grade. Proposed advisory mode: versioned cache key (root identity, normalized relative path, file identity where supported, size, high-resolution mtime), explicit invalidation on identity/change/uncertainty; handle renames, deletes, concurrent changes, reparse points and interrupted runs. Verify against full strict scan with synthetic fixtures including same-size/same-time mutations; strict evidence remains authoritative. No false claim of exact change detection from metadata alone.

## FW-003 — generic semantic adapter
Use trusted Ghidra Java exports or independently authorized tool outputs. Define versioned JSONL schema with full module SHA-256, architecture, address space, normalized entry address, producer/version, evidence references and confidence. Store functions independently from corpus files, distinguish unknown and inferred data, and attach relationships (imports/exports, strings, call edges and types) only for authorized analysis. Use bounded input, transactions, parameterized SQLite, migrations, idempotent imports and project-local databases. Validate on synthetic compiled fixtures; malformed rows, duplicate imports, unknown references and interrupted migration must be tested. Never claim original source recovery from decompiler output. The project-local semantic engine is not bundled here; request reviewed, generalized source separately before integrating it.

## FW-004 — performance and recovery
Measure wall time, CPU time, sampled peak resident memory, bytes, files, errors and database size; label OS, storage, corpus, tool version, job count and cache conditions. Compare identical workloads and check strict parity. Test 1k/10k/100k/1M synthetic files where resources permit. Verify interruption recovery, database integrity, consistent SQLite backup and restore to a separate directory. Never conflate partial checkpoints with complete backups.

## FW-005 — MCP/API host acceptance
Keep existing read-only stdio surface. Test initialization/version negotiation, tool discovery, supported read-only calls, malformed and oversized messages, cancellation/disconnect, isolation and protocol errors on actual host versions. No network listener, credentials or expanded mutation authority by default. Mark untested hosts UNVERIFIED.

## Additional approved engineering work
Prefer Rust for bounded, reusable parsers of documented open formats (INI, DDS and authorized archives). Use fixtures, truncated inputs, size limits and property/fuzz testing. Use independently developed server components with local synthetic identities, versioned password hashes, rate limiting, expiring sessions, single-use handoff, replay protection and persistence tests. Preserve existing C++/C# components; original launcher interoperability only through authorized supported interfaces. No alternative launcher by default, protection bypass, production service impersonation or Python fallback.

## Delivery gates
1. Preserve RC2 tag and release assets; no silent upgrade.
2. Verify fmt, locked check/test/clippy/release build on Linux and Windows CI; validate package manifest, links and hashes.
3. Run installer/rollback tests and actual host smoke checks before claiming support.
4. Record implementation status, commit, tests and limits per work item. Pending work is not shipped functionality. No project-specific data may be published.
