# Getting started

[Documentation index](../README.md) · [العربية](../ar/getting-started.md)

## Introduction
NorthStar RE combines one agent master, 52 specialist instruction modules and a native local Rust engine. Generation V11 uses engine 0.3.0. The source candidate does not imply Windows or agent certification.

## System requirements
Windows 11 x64, a compatible coding agent, writable LOCALAPPDATA and an approved skill directory. Ready packages need no Rust toolchain. Source builds require Rust 1.91.1 (MSRV 1.89), Cargo, Visual Studio C++ Build Tools with Windows SDK and MSVC x64. Windows PowerShell 5.1 or PowerShell 7 runs management scripts. No administrator rights or Python are required by the framework. Git/GitHub CLI is needed only for authenticated acquisition.

## Quick start
Read [installation](../../INSTALL.md), verify the authenticated release and SHA-256, extract the complete package, close the agent and run Install.cmd with its confirmed agent path. Source-only users must build first or explicitly permit Cargo dependency acquisition. Never execute a remote script piped to a shell.

## First run
Run Doctor.cmd. Discover the master in the agent; FreePi/Pi may support /skill:northstar-re, Codex uses $northstar-re and Claude uses /northstar-re. Use the absolute nsre.exe path recorded in the installed master. Run doctor, then init and resume against an existing approved project directory. Review docs/PROJECT.md and replace template text with actual evidence.

## Configuration
Management accepts -Agent FreePi|Codex|Pi|Claude|Other or -SkillRoot ABSOLUTE_DIRECTORY. Agent settings are not edited. Engine CLI flags are the configuration interface. There is no hidden global nsre.toml. Registration is local JSON; project state is separate SQLite.

## Troubleshooting
Missing Cargo/MSVC: install explicitly or use a verified binary distribution. Missing cached crates: authorize -AllowDependencyDownload. Execution policy restrictions: follow your organization policy, never bypass. Ambiguous agent path: pass -SkillRoot. Ownership refusal: preserve the existing directory and review a manual migration. Partial scan: inspect JSONL errors and skipped links, do not treat as complete. See [errors](errors.md).
