# Tool selection and usage

## Visual Studio IDE (`devenv`)
- Source: https://visualstudio.microsoft.com/downloads/
- Usage reference: https://visualstudio.microsoft.com/downloads/
- Verification: Check installed instance and About; not required for Build Tools
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## MSBuild (`msbuild`)
- Source: https://learn.microsoft.com/visualstudio/msbuild/msbuild
- Usage reference: https://learn.microsoft.com/visualstudio/msbuild/msbuild
- Verification: Developer PowerShell; build synthetic project
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## MSVC compiler (`msvc`)
- Source: https://learn.microsoft.com/cpp/build/building-on-the-command-line
- Usage reference: https://learn.microsoft.com/cpp/build/building-on-the-command-line
- Verification: Developer PowerShell; compile trusted fixture for target arch
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## .NET SDK/runtime (`dotnet`)
- Source: https://dotnet.microsoft.com/download
- Usage reference: https://dotnet.microsoft.com/download
- Verification: Inspect --list-sdks and --list-runtimes, project global.json
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

## Visual Studio Test (`vstest`)
- Source: https://learn.microsoft.com/visualstudio/test/vstest-console-options
- Usage reference: https://learn.microsoft.com/visualstudio/test/vstest-console-options
- Verification: Legacy or VS test workflows
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## Kaitai Struct Compiler (`kaitai`)
- Source: https://kaitai.io/
- Usage reference: https://doc.kaitai.io/user_guide
- Verification: Compile an original minimal .ksy to C# or C++ and parse valid/truncated fixtures.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## Git (`git`)
- Source: https://git-scm.com/download/win
- Usage reference: https://git-scm.com/download/win
- Verification: Git.Git candidate; confirm live
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

Use read-only originals and isolated offline analysis for untrusted files. Confirm current CLI syntax and architecture from upstream before use. Source projects are reference candidates, not pre-approved installers.
