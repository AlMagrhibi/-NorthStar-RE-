# NorthStar engineering engine architecture — owner-approved direction

Status: architecture approved; initial bounded parallel lab runner candidate. No new release, live scheduler, or twelve completed engines is implied.

## Engine registry and acceptance

| Engine | Existing foundation | Required next acceptance |
|---|---|---|
| Native Core | nsre, scan, index, tool discovery | typed capability registry, versioned dispatch, bounded resources |
| Binary Intelligence | probe, verified external tools | bounded parsers and normalized static analysis artifacts |
| Semantic Intelligence | specialist routing | function and relationship schema, incremental indexing, provenance |
| Algorithm Intelligence | offline lab validation | reproducible reference vectors and uncertainty-aware classification |
| Data Engineering | scan, index, JSON validation | versioned format adapters and lossless round-trip fixtures |
| Runtime Diagnostics | specialist workflows | isolated local observation, process-tree and timeout evidence |
| Compatibility | lab schema and regression checks | versioned adapters and reproducible interoperability fixtures |
| Security | evidence gates, dependency audits | scoped local security regression, explicit verification boundaries |
| Performance | benchmark lab | measured CPU/RAM/IO and bounded local workload tests |
| Knowledge | project SQLite memory | provenance-aware retrieval and safe project isolation |
| Recovery | staged install, SQLite WAL | kill/restart fault tests and resumable jobs |
| Quality Assurance | lab, CI, evidence gates | deterministic test matrix, independent evidence verification |

## Orchestrator contract

Keep one master and 52 existing specialist guides. Engines are native modules or verified external adapters, not additional discoverable skills or model sessions. Reuse existing modules; do not fork project-specific engines. Separate control plane (agent, submit/status/cancel/wait) from bounded execution plane. Define versioned task IDs, capability, input digest, explicit project scope, dependencies, priority, resource budget, deadline, state and output provenance. Default to read-only collection; no arbitrary shell or network access in the dispatcher. Separate exclusive writes from independent reads. One SQLite writer with transactional state transitions; artifacts isolated by project and task ID. Never mark interrupted work complete; validate output and hashes before consumption. Persistent worker lifecycle, cooperative cancellation, OS process-tree termination for approved adapters, restart/reconciliation and enforceable resource accounting require dedicated fault and desktop tests. No detached task is guaranteed to survive host exit until that lifecycle is implemented.

## Initial execution slice

The candidate command `nsre batch --input FILE --out NEW_FILE --jobs N --reviewed-redacted` accepts a reviewed redacted JSON manifest, schema 1, up to 64 independent lab tasks and 1 MiB input, with 1–8 Rayon workers. Each task has a unique id and input object with a supported lab operation. No paths, commands or network operations are delegated to the lab. Results preserve input order and distinguish failed evaluations from completed negative checks. Exit 0 means every operation reports passed, exit 2 means at least one did not pass, exit 1 means runner/input/output error. Require a valid full output and successful expected exit status; a partial file is not evidence. Output creation refuses overwrite. This is a synchronous CLI; a host with background-process support can run it separately while the agent works, but it is not a persistent job scheduler. Resource count limits are not OS memory/CPU quotas. Use explicit trusted project paths; hostile filesystem races are outside this initial slice.

Example manifest: `{"schema":1,"tasks":[{"id":"data","input":{"operation":"json-roundtrip","data":{"x":1}}},{"id":"qa","input":{"operation":"regression-check","baseline":1,"actual":1}}]}`.

## Delivery sequence

1. Validate bounded offline execution and stable report schema on Windows and Linux.
2. Add durable job store and submit/status/wait/cancel/reconcile commands, then host background integration.
3. Register existing native collectors and read-only adapters, enforce scope, dependencies, budgets and write locks.
4. Grow each engine in independently reviewed slices, with actual measurement and negative/fault fixtures.
5. Verify simultaneous agent work and engine collection on Windows 11 with FreePi, preserving project state and RC2 rollback. Publish only after CI, package verification and explicit release review.

No proprietary project data, credentials or binaries belong in the framework repository.

## Discoverable capability registry

The candidate `nsre engines` prints a versioned JSON registry embedded in the binary. Each engine has a stable ID, current foundation, offline batch operation names and an explicit incomplete flag. Existing CLI foundations are not equivalent to native orchestrator adapters. The registry is a discovery contract, not a runtime permission grant or a promise of live execution.
