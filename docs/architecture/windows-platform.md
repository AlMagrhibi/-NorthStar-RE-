# Windows-only engineering platform — ADR-NSO-001

Decision: approved by the owner on 2026-11-26. Windows 11 x64 is the sole supported development, deployment and desktop acceptance target for future NorthStar RE releases. No Linux or WSL requirement, compatibility gate or distribution. Historical reports remain unchanged. GitHub hosted Windows Server 2022 runners are used for automated build, tests, security, installer and package checks; they do not constitute Windows 11 desktop acceptance.

## Platform architecture
| Layer | Windows design | Status |
|---|---|---|
| Native runtime | Rust MSVC x64, pinned toolchain, existing SQLite and Rayon | Existing foundation; new branch unverified |
| Process supervisor | Dedicated per-user worker process, Windows Job Objects for controlled children, cancellation and cleanup | Planned |
| Resource governor | Job Object CPU/memory policies; process and I/O counters, adaptive worker budgets | Planned |
| Local IPC | Per-user named pipes with explicit ACL and versioned protocol; no public network listener | Planned |
| Diagnostics | Windows APIs and trusted adapters; ETW where applicable; UI Automation for desktop diagnostics | Planned |
| Durable state | Project-local SQLite, transactional claims, result hashes, explicit interrupted state | Candidate queue |
| Installer | PowerShell, staged per-user engine, backup and rollback | Existing; regression required |

Do not run arbitrary commands from job manifests. Start with typed read-only collectors and independently verified tools. Preserve original files, installed RC2, existing project state and separate per-project task databases.

## Validation gates
1. Windows MSVC formatting, check, unit/integration tests, Clippy, release build and doctor.
2. Windows process smoke: queue submit returns before execution; independent caller work, two workers, unique claims, cancellation and hash-checked results.
3. Windows supply-chain audit, licenses, SBOM, package integrity and installer lifecycle.
4. Real Windows 11 desktop: FreePi invocation, agent continues unrelated work during real collection, cancellation, crash recovery, resource enforcement, result provenance, upgrade and rollback.
5. All twelve engines must have functional tests for their published scope. Report unsupported or incomplete operations explicitly. Only publish a new version after its verified gates and release request.

## Delivery boundaries
The current candidate does not implement a Windows service, named pipe server, Windows Job Objects, live collectors, full DAG, automatic recovery or the full twelve engines. A passing process smoke is not a full orchestrator or desktop acceptance. Never convert planned features into release claims without evidence.

[Architecture](overview.md) · [Task lifecycle and acceptance](task-lifecycle.md)

## Desktop control capability

[ADR-NSO-002: Windows Desktop Control](desktop-control.md). Native Win32 candidate for explicit window discovery, inspection, restore, focus, position and geometry; UI Automation, DirectX/window capture, typed input, audit and interactive desktop acceptance remain separate gates.
