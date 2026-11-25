# Tool selection and usage

## Native engine (`nsre`)
- Use the installed Rust CLI and its V11 command contract.

## Detect It Easy (`die`)
- Source: https://github.com/horsicq/Detect-It-Easy/releases
- Usage reference: https://github.com/horsicq/Detect-It-Easy/releases
- Verification: Check release/build and signature scan on synthetic fixture
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

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

## 7-Zip (`7zip`)
- Source: https://www.7-zip.org/
- Usage reference: https://www.7-zip.org/faq.html
- Verification: List and test a synthetic archive with installed CLI help; check results without extraction.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## Resource Hacker (`resource-hacker`)
- Source: https://www.angusj.com/resourcehacker/
- Usage reference: https://www.angusj.com/resourcehacker/
- Verification: Open an owned PE fixture and compare version/icon resources to known values.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

Use read-only originals and isolated offline analysis for untrusted files. Confirm current CLI syntax and architecture from upstream before use. Source projects are reference candidates, not pre-approved installers.
