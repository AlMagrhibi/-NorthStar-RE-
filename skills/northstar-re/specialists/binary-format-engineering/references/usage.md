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

## Kaitai Struct Compiler (`kaitai`)
- Source: https://kaitai.io/
- Usage reference: https://doc.kaitai.io/user_guide
- Verification: Compile an original minimal .ksy to C# or C++ and parse valid/truncated fixtures.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## .NET SDK/runtime (`dotnet`)
- Source: https://dotnet.microsoft.com/download
- Usage reference: https://dotnet.microsoft.com/download
- Verification: Inspect --list-sdks and --list-runtimes, project global.json
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## MSVC compiler (`msvc`)
- Source: https://learn.microsoft.com/cpp/build/building-on-the-command-line
- Usage reference: https://learn.microsoft.com/cpp/build/building-on-the-command-line
- Verification: Developer PowerShell; compile trusted fixture for target arch
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## CMake (`cmake`)
- Source: https://cmake.org/download/
- Usage reference: https://cmake.org/download/
- Verification: Validate project presets
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## CTest (`ctest`)
- Source: https://cmake.org/cmake/help/latest/manual/ctest.1.html
- Usage reference: https://cmake.org/cmake/help/latest/manual/ctest.1.html
- Verification: Run tests only for trusted source
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

Use read-only originals and isolated offline analysis for untrusted files. Confirm current CLI syntax and architecture from upstream before use. Source projects are reference candidates, not pre-approved installers.
