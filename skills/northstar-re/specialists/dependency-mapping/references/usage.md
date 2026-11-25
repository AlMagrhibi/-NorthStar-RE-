# Dependency graph for client, server, SDK and data — reference and verification guide

Read exact installed-version docs before use. No automatic downloads or execution of untrusted code.

## Dependencies (`dependencies`)
- Official: https://github.com/lucasg/Dependencies/releases
- Usage: https://github.com/lucasg/Dependencies/releases
- Local check: Optional import graph; static only, no profiling/loading target

## DUMPBIN (`dumpbin`)
- Official: https://learn.microsoft.com/cpp/build/reference/dumpbin-reference
- Usage: https://learn.microsoft.com/cpp/build/reference/dumpbin-reference
- Local check: Developer PowerShell; inspect trusted PE fixture

## Process Explorer (`process-explorer`)
- Official: https://learn.microsoft.com/sysinternals/downloads/process-explorer
- Usage: https://learn.microsoft.com/sysinternals/downloads/process-explorer
- Local check: Observe own fixture process and expected modules.

## MSBuild (`msbuild`)
- Official: https://learn.microsoft.com/visualstudio/msbuild/msbuild
- Usage: https://learn.microsoft.com/visualstudio/msbuild/msbuild
- Local check: Developer PowerShell; build synthetic project

## .NET SDK/runtime (`dotnet`)
- Official: https://dotnet.microsoft.com/download
- Usage: https://dotnet.microsoft.com/download
- Local check: Inspect --list-sdks and --list-runtimes, project global.json

## vcpkg (`vcpkg`)
- Official: https://github.com/microsoft/vcpkg
- Usage: https://learn.microsoft.com/vcpkg/
- Local check: Check baseline and manifest; restore a reviewed dependency in disposable project.

## NuGet (`nuget`)
- Official: https://www.nuget.org/
- Usage: https://learn.microsoft.com/nuget/
- Local check: Check nuget.exe or dotnet nuget; restore reviewed locked dependency in fixture.

## Fixture and evidence
Known dependency fixture and missing-dependency negative case.
Record expected versus observed results, exact tool version, exit status, source and output hashes.
