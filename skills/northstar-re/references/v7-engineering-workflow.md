# V7 engineering workflow — mandatory across specialists

| Phase | Required evidence | Gate |
|---|---|---|
| Discover | Host, architecture, scope, project/target hashes, inventory | Required capabilities verified by trusted fixture |
| Research | Primary references, date, target/version relevance, license | No unsupported claim treated as fact |
| Plan | Work packages, dependencies, acceptance criteria, approvals | Writes and runtime explicitly scoped |
| Analyze | Static observations, locations, corroboration | Unknowns clearly separated from proven facts |
| Develop | Original source and reviewed changes, checkpoint | Only authorized workspace changes |
| Build | Exact toolchain, configs, locked dependencies, logs | Successful build for each claimed target |
| Test | Expected/observed results, negative and regression fixtures | Required tests executed and passed |
| Repair | Root cause, bounded fix and retest | No blind retries or widening scope |
| Verify | Artifacts, hashes, matrix, unresolved blockers | Required gates all verified |
| Remember | Reviewed, redacted records, prior evidence and supersession | Project-local, no secrets |

Task DAG: use `nsre jobs --project DIR --action validate --input PLAN --reviewed-redacted` to validate JSON. This is a validator, not an agent scheduler. Dispatch only where the host supports delegation. Use a single writer per output, separate worker directories and explicit scoped approvals for downloads, installation, elevation, network captures and runtime analysis. Do not mark planning as execution. A blocked dependency stops only its descendants. Set a retry budget of two attempts per work package unless explicitly approved; no endless loop. Save commands and results, not sensitive memory or authentication data. Checkpoint via authorized Git branch or backup; never reset an existing dirty workspace.

Read-only source files and samples; run unknown content only in an authorized, disposable offline guest with no writable host shares and no shared credentials. Do not weaken anti-cheat, licensing, DRM or authentication. Avoid any cloud analysis that uploads binaries or private source. Source reconstruction is original, authorized equivalent implementation, not recovery of exact original source.

Windows 11 acceptance: read `docs/WINDOWS11-ACCEPTANCE.md` in distribution root. Treat simulated and real host results separately. The master is instruction-driven; actual autonomy and tool access depend on the agent host.
