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

1. Validate bounded offline execution and stable report schema on Windows only.
2. Add durable job store and submit/status/wait/cancel/reconcile commands, then host background integration.
3. Register existing native collectors and read-only adapters, enforce scope, dependencies, budgets and write locks.
4. Grow each engine in independently reviewed slices, with actual measurement and negative/fault fixtures.
5. Verify simultaneous agent work and engine collection on Windows 11 with FreePi, preserving project state and RC2 rollback. Publish only after CI, package verification and explicit release review.

No proprietary project data, credentials or binaries belong in the framework repository.

## Discoverable capability registry

The candidate `nsre engines` prints a versioned JSON registry embedded in the binary. Each engine has a stable ID, current foundation, offline batch operation names and an explicit incomplete flag. Existing CLI foundations are not equivalent to native orchestrator adapters. The registry is a discovery contract, not a runtime permission grant or a promise of live execution.

## Durable queue prototype (candidate)

The new explicit queue CLI persists reviewed offline batch inputs in a dedicated SQLite database with application ID and schema version. `nsre queue init DB` creates a new database; `nsre queue submit DB ID INPUT PRIORITY WORKERS --reviewed-redacted` enqueues without executing; `nsre queue run-one DB` transactionally claims one job by priority; `nsre queue status DB ID` returns state and hash-checked results; `nsre queue cancel DB ID` cancels queued work; `nsre queue interrupt DB ID --worker-stopped` records an interrupted running job only after the operator establishes the worker stopped. The worker command is explicit: the host may launch it independently if supported. There is no autonomous daemon, lease expiry, automatic retry, running cancellation, cross-job DAG, OS quota enforcement, or live collection. Use a dedicated trusted project-local queue path and reviewed redacted inputs; no shared databases between projects. The current implementation is a candidate for NSO-02, not acceptance of NSO-02. A status query does not independently corroborate underlying lab findings.

## Windows platform scope

Future orchestrator work is Windows 11 x64 only. Prefer native Windows APIs and verified Windows adapters. Hosted Windows CI provides automated evidence; FreePi desktop acceptance is independent. No Linux or WSL prerequisite. Windows Job Objects, named pipes, and live diagnostics are target components, not current functionality.

## Bounded dependency execution and queue drain (candidate)

Schema 1 batch tasks may declare `depends_on` as a list of task IDs in the same manifest. Unknown IDs, duplicates, self-dependencies and cycles are rejected before execution. Independent tasks in a topological layer run in parallel within the existing 1–8 worker limit; later layers wait. A dependency that fails or returns a negative check blocks dependents transitively. Output order remains input order. This is a DAG within one offline batch, not a cross-job dependency scheduler, live collector or output-passing workflow. `nsre queue drain DB LIMIT` explicitly processes at most 1–64 queued jobs in one worker process, respecting transactional claims and priority; it stops when idle or the count is reached. It does not wait for new tasks or run autonomously, and returns a negative exit status if any processed job fails. This advances part of NSO-04; acceptance remains pending Windows CI and further lifecycle work.

## Twelve-engine typed routing (candidate)

Each of the twelve registered engines now has at least one bounded executable offline operation. New operations are `content-digest` (Native Core; SHA-256 over supplied UTF-8), `binary-header` (Binary Intelligence; hexadecimal byte signature and PE signature hint, not full parser), `semantic-reachability` (Semantic Intelligence; directed graph traversal on supplied nodes and edges, not extraction), and `checkpoint-compare` (Recovery; compare supplied expected/actual manifests, not restore or independent file hashing). Batch tasks optionally declare `engine`; mismatched operation ownership is rejected before execution. Results identify the registered engine. Tests cover registry ownership, graph cycles, malformed hex, digest vectors and negative checkpoint differences. This is a typed offline execution foundation; all engine completion flags remain false until full acceptance. No target execution, live data collection, or automatic recovery is implied.

## Local file evidence collector (candidate)

`nsre collect --root DIR --input FILE --out NEW_FILE --jobs N --reviewed-redacted` uses schema 1 `{ "schema":1, "files":[{"path":"relative/file.dat","expected_sha256":"optional 64 hex characters"}] }`. It independently streams SHA-256 and bounded signature/entropy hints from selected local files; up to 64 entries, 64 MiB per file, 1–8 workers. It reports mismatch and access errors as negative results and refuses output inside the source tree or existing output. It rejects reparse points, nonregular files and path traversal; use a trusted stable workspace, not an adversarial filesystem (TOCTOU is not eliminated). This shared CLI advances Native Core, Binary Intelligence, Knowledge, Recovery and QA, but is not yet callable through the offline queue and does not perform restore, full format validation, or live process collection. No engine completion flags change. Windows acceptance pending.

## Queue wait, provenance and reviewed recovery (candidate)

`nsre queue wait DB ID SECONDS` polls an existing job for at most 0–600 seconds (SQLite busy handling may extend a poll); returns the current report and terminal/timeout flags, exit 0 only for completed, exit 2 for timeout or unsuccessful terminal state. Status and wait recompute the stored input and result hashes, reject inconsistent state/result, and expose both digests. `nsre queue retry DB OLD_ID NEW_ID --reviewed-redacted` creates a new immutable job from an intact interrupted offline input, preserving the original record and original priority/worker settings. It requires a fresh ID and operator-reviewed interruption after verifying the previous worker stopped; it is not automatic crash detection, a lease, an autonomous retry service or proof of independent source evidence. Tests include duplicate retry, timeout, interrupted record preservation, and input corruption. Hosted Windows smoke checks wait and input/result digests. No completion flags are changed.
