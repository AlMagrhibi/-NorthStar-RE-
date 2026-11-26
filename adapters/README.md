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

## Optional desktop session

The candidate server also accepts `--desktop-control` with an explicit project to add nsre_desktop_observe/nsre_desktop_act. It remains disabled by default. Observation supports list, capabilities, inspect, bounded UIA controls, capture and capture-wgc; GUI actions require a selected HWND/PID and apply=true. Captures/audit stay under existing project report/log directories. Typed text is passed to the owned CLI by stdin. A bounded owned child enforces action timeout and response size. No FreePi registration/configuration is changed. See the master's [native desktop workflow](../skills/northstar-re/references/desktop-native.md) and [desktop contract](../docs/architecture/desktop-control.md). This source capability does not certify host compatibility.
