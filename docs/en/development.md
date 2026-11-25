# Developer guide

[Documentation index](../README.md) · [العربية](../ar/development.md)

## Build from source and Cargo workspace
Install the pinned rust-toolchain.toml and MSVC/C++ Build Tools on Windows. From skills/northstar-re/engine/nsre run cargo fmt --all -- --check; cargo check --workspace --locked; cargo test --workspace --locked; cargo clippy --workspace --all-targets --locked -- -D warnings; cargo build --release --locked. Cargo.lock is reviewed and committed. Framework V11 and crate 0.3.0 are separate version domains.

## Dependencies
Direct crates: rayon, walkdir, sha2, serde_json and rusqlite with bundled SQLite. Review lockfile diffs, upstream advisories and SPDX metadata. The framework remains proprietary/unlicensed for redistribution pending owner licensing approval; dependency licenses do not license this framework. See [notices](../../THIRD-PARTY-NOTICES.md).

## Tests and benchmarks
Rust integration tests exercise inventory/cache/diff, unrelated DB refusal, project isolation, gates, checkpoints and unsafe paths. scripts/Test-Installer.ps1 uses a real built EXE in temporary isolated user directories on Windows. scripts/Validate-Package.ps1 checks manifests, registry, links, checksums and selected secret patterns. Benchmark tooling uses synthetic files only. Agent acceptance is separate from these tests.

## Extending engine and skills
Add a module behind an explicit CLI contract; maintain exit codes and bounded resource usage. Add a specialist with matching name/description frontmatter and update schemas/specialists.json and expected counts intentionally. Keep source research and operational references. New functionality needs regression evidence and an honest status.

## Adapter SDK and API
There is no compiled plugin ABI or adapter SDK yet. Adapter docs describe skill-root discovery only. Do not present a configuration example as a tested integration. Local stdio API currently exposes read-only doctor/status.

## Release and security
Follow [release engineering](../../RELEASE.md) and [security policy](../../SECURITY.md). Mandatory CI failures block publication. Never commit project state, targets, credentials, runtime logs or private analysis.
