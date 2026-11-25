# Installation, upgrade and removal

[Agent-first guide](INSTALL-FOR-AGENTS.md) · [Requirements and first run](docs/en/getting-started.md) · [التعليمات العربية](docs/ar/getting-started.md)

## Ready-to-install Windows edition
Download only an authenticated official Windows x64 release with successful mandatory CI. Verify SHA256SUMS.txt and release-manifest.json; extract the entire archive to a new directory. Close the agent and run Install.cmd. Supply -Agent FreePi|Codex|Pi|Claude or an explicit -SkillRoot absolute path. No administrator rights are needed. Do not use script policy bypass or silently install prerequisites.

## Source edition
Requires the pinned Rust toolchain, MSVC x64 and Visual Studio C++ Build Tools/Windows SDK. Install.cmd builds/tests the engine with Cargo.lock and defaults to cached/offline dependencies. Explicitly permit first-time dependency downloads with Install.cmd -Agent FreePi -AllowDependencyDownload. Build-Engine.cmd builds without registering a skill. Verify actual nsre.exe --version and doctor; source presence alone is not build success.

## Upgrade and rollback
Update.cmd uses the currently extracted verified package; it does not silently contact the internet. For explicit online discovery use install/Fetch-Release.ps1 -Destination NEW_DIR; opt into release candidates only knowingly. Review, verify and extract, then Update.cmd. Matching package hashes produce an idempotent verification. Backup.cmd saves skills and registration; Rollback.cmd -BackupPath ABSOLUTE_BACKUP restores a verified retained engine registration. Uninstall.cmd removes only owned skill registration and retains releases, backups and project data.

The central root is %LOCALAPPDATA%/NorthStarRE. Project .northstar directories are independent and never modified by management. Unowned skills and redirected paths are refused. Interrupted process recovery and real Windows 11/agent acceptance are recorded separately from ordinary exception rollback.

## Management commands
Install.cmd, Update.cmd, Doctor.cmd, Build-Engine.cmd, Backup.cmd, Rollback.cmd, Uninstall.cmd. Forward parameters to install/Manage.ps1; see [configuration](docs/en/configuration.md).
