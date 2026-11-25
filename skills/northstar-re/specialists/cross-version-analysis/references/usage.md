# Tool selection and usage

## Native engine (`nsre`)
- Use the installed Rust CLI and its V11 command contract.

## Git (`git`)
- Source: https://git-scm.com/download/win
- Usage reference: https://git-scm.com/download/win
- Verification: Git.Git candidate; confirm live
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

## ILSpy CLI (`ilspycmd`)
- Source: https://www.nuget.org/packages/ilspycmd
- Usage reference: https://www.nuget.org/packages/ilspycmd
- Verification: Pin compatible release; use installed help
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## Ghidra (`ghidra`)
- Source: https://github.com/NationalSecurityAgency/ghidra/releases
- Usage reference: https://github.com/NationalSecurityAgency/ghidra/releases
- Verification: Read application.properties and installed docs; trusted fixture headless
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## Kaitai Struct Compiler (`kaitai`)
- Source: https://kaitai.io/
- Usage reference: https://doc.kaitai.io/user_guide
- Verification: Compile an original minimal .ksy to C# or C++ and parse valid/truncated fixtures.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

Use read-only originals and isolated offline analysis for untrusted files. Confirm current CLI syntax and architecture from upstream before use. Source projects are reference candidates, not pre-approved installers.
