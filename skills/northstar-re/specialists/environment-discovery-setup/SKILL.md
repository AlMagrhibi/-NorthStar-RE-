---
name: environment-discovery-setup
description: Mandatory Windows 11 environment discovery, acquisition planning, installation validation, architecture/workload checks and reproducible toolchain readiness for every NorthStar RE task.
---
# Mandatory environment gate (first, every session)
Read references/tool-catalog.json, setup-runbook.md and ../../references/evidence-contract.md. Inventory every catalog item, including optional tools; discovery does not imply installation. Check PATH, standard install roots, portable tools, user-supplied tool roots, installed application registry, Visual Studio instances/workloads, SDKs/runtimes/JDKs and architecture. Never search the entire disk indiscriminately or execute a file merely found by name. Run nsre doctor and nsre tools; discovery is read-only. Supplement with PowerShell Get-Command, Get-ItemProperty and vswhere. Record status for each tool: NOT_FOUND / PRESENT_UNVERIFIED / VERIFIED / FAILED / NOT_REQUIRED. Resolve candidates and provenance before invoking. Check versions, actual startup, dependencies and a synthetic fixture appropriate to the task. Check both installed and usable; PATH misses do not imply absence. Read actual project configuration, not a fixed client/version. Determine task-required capabilities; optional absence is not a blocker. Do not install every optional package by default. Offer one explicit scoped setup plan listing needed missing tools, publisher, source, compatible release, risk, elevation, network, disk and rollback. After approval, carry it out using the host agent's actual tools; do not merely list commands if authorized and able to execute. Do not silently upgrade or uninstall existing tools. Reopen developer shell after installation, rerun inventory and real smoke tests. Do not proceed if a required capability is unverified; give exact blocker. If no Windows execution access, say so. Never use target directories as tool search roots, or run unknown target DLLs. Keep guest disconnected for untrusted artifacts. Save reports/environment.json and reports/TOOLCHAIN_READINESS.md with command, version, path, exit status, source and fixture evidence.

## V5 shared routing
Read `../../references/tool-selection.md` and the central tool catalog. Verify task-required tools using their upstream usage references and synthetic fixtures; route directory, archive, format, asset and authorized data-recovery tasks to the appropriate V5 specialists. No fixed client version.

V5: catalog source projects may report SOURCE_REVIEW_REQUIRED and documentation REFERENCE_ONLY; neither means a required install. Read ../../references/tool-selection.md.

## V6 server environment
Discover installed network diagnostics, database clients/engines, Windows SDK, .NET workloads and local service dependencies; validate only task-required components with disposable fixtures. Never auto-start database services, install capture drivers or open firewall ports without scoped approval. Documentation-only SDK capabilities require a compile or runtime smoke test when needed.

## V7 shared workflow
Follow `../../references/v7-engineering-workflow.md`: environment readiness, project evidence, scoped plan, fixture verification, bounded repair and redacted knowledge. Report blocked capabilities and actual test status; no fixed client, language or version.

## Mandatory V8 documentation
Follow `../../references/v8-project-governance.md` and `engineering-documentation`: record material actions, results, redacted evidence, decisions, blockers and handoff in project Markdown. Do not overwrite existing documentation or log secrets.

## V9 Rust discovery
Inventory Rustup, rustc, Cargo, optional formatting/lint/supply-chain tools. Check `%USERPROFILE%\.cargo\bin`, `CARGO_HOME`, PATH and the verified MSVC developer shell. Read ../../references/v9-rust-workflow.md. Only mark VERIFIED after a real trusted compile/test fixture; discover but do not install optional tools indiscriminately.

## V10 engine integration
Read `../../references/v10-engineering-engine.md` before choosing a verified nsre command. Treat the Rust engine as a source candidate until local acceptance; retain the existing tools and evidence contract.
