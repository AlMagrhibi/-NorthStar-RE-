# Agent integration contracts

Current adapters are documented filesystem skill registrations, not a compiled SDK. [Compatibility matrix](../docs/en/compatibility.md) records discovery roots, invocation, reload, unverified versions and limitations. Manage.ps1 accepts an explicit agent/root and never edits unrelated settings or provider credentials.

For a new adapter, establish official host/version documentation, discovery path, invocation/reload semantics and recursive discovery behavior. Test against a disposable profile, preserve existing skills, then record exact host/version and evidence. Until then use UNVERIFIED. A plugin ABI, worker protocol and automatic MCP registration are planned and not supplied here.

## Read-only native agent tools (candidate)

The stdio candidate `nsre mcp --project ABSOLUTE_PROJECT` now advertises four tools: `nsre_doctor`, `nsre_status`, `nsre_capabilities`, and `nsre_queue_status`. Capability discovery returns the embedded twelve-engine registry, explicit completion flags, producer version and supported queue contracts. It does not certify that an external tool or host is installed.

`nsre_queue_status` accepts exactly `{"id":"job-id"}` and reads only the existing `.northstar/state/jobs.sqlite` beneath the bound project. Queue database paths, SQL, shell commands and write operations are not accepted. Status validates input/result digests; it does not independently reobserve a previously collected source. The read opens SQLite with READ_ONLY and query_only; SQLite WAL bookkeeping may still use normal sidecars. Missing queues fail without initializing project state. The native JSON API equivalents are `capabilities` and `queue-status` (`params.id`). Existing doctor/status calls remain supported.

For a disposable Windows project, use the installed verified executable explicitly:

```powershell
& $Engine queue init (Join-Path $Project '.northstar/state/jobs.sqlite')
& $Engine queue submit (Join-Path $Project '.northstar/state/jobs.sqlite') 'job-1' $ReviewedInput '50' '2' --reviewed-redacted
# Start run-one/drain in a separately owned host process if supported, then query the ID.
& $Engine mcp --project $Project
```

Initialize only a new dedicated queue; do not reinitialize an existing database. `$Project` must already have its normal `.northstar/state` directory. MCP clients must initialize and send `notifications/initialized` before calling tools. An existing FreePi version may not support MCP registration; detect the actual host capabilities before configuring it. No provider credentials, host settings, skills or user installation are modified by this adapter. Submission remains through the reviewed native CLI. The integration test uses real stdio processes and disposable projects, verifies discovery/result consumption, rejects extra database/path arguments and checks isolation; it is not a FreePi host test or evidence that work survives host exit. Host versions and interactive Windows 11 acceptance remain UNVERIFIED.

## Queue schema v5 — worker ownership, leases, fencing and supervision

Queue databases now carry schema v5, and `queue migrate DB` upgrades v1 through v4 in one
transaction while refusing to run if any job is `running`. Existing jobs, dependencies and
exclusive keys are preserved. Writing to or running workers on an older database is refused
until it migrates; reads stay available.

```powershell
& $Engine queue run-one $Db --worker 'host:nsre:4321' --lease-ms 60000
& $Engine queue heartbeat $Db 'job-1' 'host:nsre:4321' 1 60000
& $Engine queue status $Db 'job-1'
& $Engine queue reconcile $Db
& $Engine queue cancel $Db 'job-1' --running
& $Engine queue interrupt $Db 'job-1' --worker-stopped
& $Engine queue cancel $Db 'job-1'
& $Engine queue retry $Db 'job-1' 'job-1-retry' --reviewed-redacted
```

Each claim increments a per-job fencing token, records the owner, the claim instant as
`heartbeat_ms` and a bounded deadline. Terminal results are published only by the current
owner when the same statement still sees a live lease, the matching token and no pending
cancellation; a worker whose lease already expired gets `LEASE_LOST` even if reconciliation
has not run yet, and `heartbeat` refuses to renew an already expired lease.

`reconcile` scans expired leases and then re-checks expiry, state and token inside each
conditional update, so a heartbeat that lands between the scan and the update wins instead
of being overwritten. Expired leases become `interrupted`, never silently replayed: lease
expiry does not prove the worker stopped. Status changes and their `lease_events` row commit
in one transaction, so evidence cannot drift from state.

Exclusive keys stay reserved by an interrupted job until `interrupt ... --worker-stopped`
records that the operator verified the owned worker exited. A cancellation requested on a
job whose owner died is recorded as `cancel_unconfirmed` and only becomes `cancelled` after
that verification, so the framework never claims a cancellation completed while the old
worker might still be acting. `status` exposes `lease.owner`, `lease.token`, `lease.attempt`,
`lease.expires_utc_ms`, `lease.cancel_requested`, `lease.worker_stop_verified` and
`lease.exclusive_keys_held`.

## Owned worker supervision

```powershell
& $Engine supervisor spawn $Db 'job-1' --worker 'host:nsre:4321' --lease-ms 60000 --deadline-ms 120000 --heartbeat-ms 5000
& $Engine supervisor verify $Db 'job-1'
```

`supervisor spawn` runs the worker as a child `nsre queue run-one --job ...` process inside
a Windows Job Object the supervisor owns (`JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`), so the whole
worker tree dies with it. The claim records the spawning process id and its creation time;
the deadline is measured monotonically and a passing deadline kills the tree, observes the
exit, and records a verified stop from that observation — not from a text statement.
`supervisor verify` re-checks the recorded (pid, creation-time) pair against the live system,
so a reused PID cannot pass for the original worker; it refuses while the process is alive.
Inside the worker, `run-one --heartbeat-ms N` runs an independent heartbeat thread that
renews the lease and writes `progress` while the evaluation runs, and `--deadline-ms N`
turns a late completion into an explicit `DEADLINE_EXCEEDED` failure. The `pause` batch
operation (50..=30000 ms) exists as a bounded fixture for these tests.

## Optional desktop session

The candidate server also accepts `--desktop-control` with an explicit project to add nsre_desktop_observe/nsre_desktop_act. It remains disabled by default. Observation supports list, capabilities, inspect, bounded UIA controls, capture and capture-wgc; GUI actions require a selected HWND/PID and apply=true. Captures/audit stay under existing project report/log directories. Typed text is passed to the owned CLI by stdin. A bounded owned child enforces action timeout and response size. No FreePi registration/configuration is changed. See the master's [native desktop workflow](../skills/northstar-re/references/desktop-native.md) and [desktop contract](../docs/architecture/desktop-control.md). This source capability does not certify host compatibility.
