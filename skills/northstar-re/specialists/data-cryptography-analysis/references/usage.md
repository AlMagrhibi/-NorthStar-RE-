# Tool selection and usage

## ImHex (`imhex`)
- Source: https://github.com/WerWolv/ImHex
- Usage reference: https://docs.werwolv.net/imhex
- Verification: Open a synthetic binary read-only; inspect offsets, bytes, entropy and an audited local pattern.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## HxD (`hxd`)
- Source: https://mh-nexus.de/en/hxd/
- Usage reference: https://mh-nexus.de/en/hxd/
- Verification: Open a synthetic file read-only and verify byte offsets and selected range.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## OpenSSL (`openssl`)
- Source: https://openssl-library.org/
- Usage reference: https://docs.openssl.org/
- Verification: Verify provenance of Windows distribution; compare digest of synthetic data against known answer; use documented cryptography only with authorized keys.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## Native engine (`nsre`)
- Use the installed Rust CLI and its V11 command contract.

## .NET SDK/runtime (`dotnet`)
- Source: https://dotnet.microsoft.com/download
- Usage reference: https://dotnet.microsoft.com/download
- Verification: Inspect --list-sdks and --list-runtimes, project global.json
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## Detect It Easy (`die`)
- Source: https://github.com/horsicq/Detect-It-Easy/releases
- Usage reference: https://github.com/horsicq/Detect-It-Easy/releases
- Verification: Check release/build and signature scan on synthetic fixture
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

Use read-only originals and isolated offline analysis for untrusted files. Confirm current CLI syntax and architecture from upstream before use. Source projects are reference candidates, not pre-approved installers.
