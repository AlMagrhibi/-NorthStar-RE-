# Install NorthStar RE from this repository

## Scope and discovery
1. Confirm the user requested installation or update. Detect OS, architecture, host agent/version and user profile. Windows 11 x64 is the primary target; other OS installation is BLOCKED, even if Rust can build there.
2. Read [compatibility](docs/en/compatibility.md). Discover actual host settings and existing skill roots read-only. Confirm the path using installed host docs; conventions are candidates. Do not register 52 separate public skills or change unrelated agent configuration.
3. Inspect %LOCALAPPDATA%/NorthStarRE/registration.json, existing northstar-re and backups. An unowned existing skill is never overwritten; preserve it and resolve migration with the user.

## Acquire and verify
4. Use existing authenticated GitHub access for this private repository. Never print tokens, add them to URLs or persist credentials. Read [Releases](https://github.com/AlMagrhibi/-NorthStar-RE-/releases). Select a compatible V11 Windows x64 distribution with engine 0.3.0, required checks and SHA256SUMS.txt. Release candidates require explicit acceptance of their limitations. No published asset means do not invent a download link.
5. With an authenticated GitHub CLI, install/Fetch-Release.ps1 can explicitly fetch and verify a compatible asset without executing it. For source builds, acquire a fixed reviewed commit/archive, the committed Cargo.lock, Rust 1.91.1 MSVC and C++ Build Tools. Missing prerequisites must be installed only under user authorization.
6. Verify outer SHA-256 against the authenticated release, inspect release-manifest.json, extract to a new trusted directory, reject path traversal/reparse points and run scripts/Validate-Package.ps1 -CheckHashes. Checksums establish byte integrity; trust comes from authenticated publisher provenance.

## Install and verify
7. Close the agent to avoid simultaneous reads. Run Install.cmd -Agent HOST, or -SkillRoot ABSOLUTE_DIRECTORY with the confirmed discovery path. -NonInteractive may be used only for an already authorized path and operation. Source builds default offline; -AllowDependencyDownload is an explicit network choice.
8. The manager stages a central engine under %LOCALAPPDATA%/NorthStarRE/releases, checks version/doctor, backs up owned existing skills and registration, then places the master and internal guides. It writes the absolute engine path into the installed master. No admin elevation or Python runtime is used.
9. Run Doctor.cmd, confirm one discoverable master plus 52 GUIDE.md files, and verify nsre.exe --version reports 0.3.0. This does not certify optional tools.
10. Reload the actual host: FreePi/Pi /reload where supported; Codex automatic discovery/restart if needed; Claude restart if absent. Invoke /skill:northstar-re only on supporting hosts; Codex $northstar-re, Claude /northstar-re. Record actual host/version/result and keep UNVERIFIED until observed.
11. In an approved existing disposable project, run doctor, init and resume. Confirm docs/state exist, existing files are preserved and unrelated projects are untouched.

## Updates, failure and offline behavior
Use the authenticated acquisition step for an explicit update, then Update.cmd from the verified extracted package. The old engine remains available and project .northstar data is never touched. Installation catches restore prior skill placement on failure; retained stage/prior directories permit manual recovery after process/power interruption. Backup.cmd creates an owned backup; Rollback.cmd -BackupPath ABSOLUTE_CENTRAL_BACKUP restores skill/registration after verifying the retained engine. Uninstall.cmd unregisters only the owned master and retains central releases/backups and all project data. Offline installed engine operations remain local; online update failure must not disable them.

Report in Arabic: selected commit/tag/checksum, actual host path, engine version, doctor output, reload/invocation evidence, backups and any BLOCKED or UNVERIFIED gate. Never claim host certification from documentation alone.
