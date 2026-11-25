# Reproducible builds, dependencies, release and rollback — reference and verification guide

Read exact installed-version docs before use. No automatic downloads or execution of untrusted code.

## Git (`git`)
- Official: https://git-scm.com/download/win
- Usage: https://git-scm.com/download/win
- Local check: Git.Git candidate; confirm live

## Visual Studio Locator (`vswhere`)
- Official: https://github.com/microsoft/vswhere
- Usage: https://github.com/microsoft/vswhere
- Local check: Visual Studio Installer directory; query instances and workloads

## MSBuild (`msbuild`)
- Official: https://learn.microsoft.com/visualstudio/msbuild/msbuild
- Usage: https://learn.microsoft.com/visualstudio/msbuild/msbuild
- Local check: Developer PowerShell; build synthetic project

## MSVC compiler (`msvc`)
- Official: https://learn.microsoft.com/cpp/build/building-on-the-command-line
- Usage: https://learn.microsoft.com/cpp/build/building-on-the-command-line
- Local check: Developer PowerShell; compile trusted fixture for target arch

## CMake (`cmake`)
- Official: https://cmake.org/download/
- Usage: https://cmake.org/download/
- Local check: Validate project presets

## Ninja (`ninja`)
- Official: https://github.com/ninja-build/ninja/releases
- Usage: https://github.com/ninja-build/ninja/releases
- Local check: Optional generator

## .NET SDK/runtime (`dotnet`)
- Official: https://dotnet.microsoft.com/download
- Usage: https://dotnet.microsoft.com/download
- Local check: Inspect --list-sdks and --list-runtimes, project global.json

## LLVM / Clang (`llvm-clang`)
- Official: https://llvm.org/
- Usage: https://clang.llvm.org/docs/
- Local check: Verify toolchain version and target; compile a small owned source fixture.

## vcpkg (`vcpkg`)
- Official: https://github.com/microsoft/vcpkg
- Usage: https://learn.microsoft.com/vcpkg/
- Local check: Check baseline and manifest; restore a reviewed dependency in disposable project.

## NuGet (`nuget`)
- Official: https://www.nuget.org/
- Usage: https://learn.microsoft.com/nuget/
- Local check: Check nuget.exe or dotnet nuget; restore reviewed locked dependency in fixture.

## Doxygen (`doxygen`)
- Official: https://www.doxygen.nl/
- Usage: https://www.doxygen.nl/manual/
- Local check: Generate local documentation from tiny owned project.

## 7-Zip (`7zip`)
- Official: https://www.7-zip.org/
- Usage: https://www.7-zip.org/faq.html
- Local check: List and test a synthetic archive with installed CLI help; check results without extraction.

## Fixture and evidence
Clean build and fixture tests succeed for each claimed configuration.
Record expected versus observed results, exact tool version, exit status, source and output hashes.
