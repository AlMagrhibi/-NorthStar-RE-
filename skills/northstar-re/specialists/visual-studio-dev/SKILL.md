---
name: visual-studio-dev
description: Windows 11 native and managed development with Visual Studio, MSVC C/C++, C#/.NET, MSBuild, CMake, debugging, testing and C++/C# interoperability.
---
# Visual Studio engineering

**Mandatory shared gate:** Read `../environment-discovery-setup/SKILL.md`, `../../references/evidence-contract.md`, and `references/operational-reference.md` before work. Verify required tools, install only approved missing dependencies, and use actual target evidence rather than a fixed client or version.

Discover `.sln`, `.slnx`, `.vcxproj`, `.csproj`, `global.json`, `CMakePresets.json`, dependency lockfiles and target framework. Use Visual Studio Installer to confirm Desktop development with C++ and .NET desktop development workloads, appropriate Windows SDK and MSVC toolset. Use vswhere (https://github.com/microsoft/vswhere) to locate actual instances, not a hardcoded year. Use Developer PowerShell or Developer Command Prompt for the selected architecture; `Get-Command cl,msbuild,cmake,dotnet` and `dotnet --info` verify availability. Missing workload: propose installation, do not silently install.

## C++
- Preserve existing build system, select x86/x64/ARM64 explicitly; use RAII, smart pointers, bounds checks, explicit ownership and error handling. Respect ABI, alignment, calling convention, CRT and DLL exports. Prefer CMake presets when supplied.
- For trusted project: `cmake --preset <actual-configure-preset>`; `cmake --build --preset <actual-build-preset>`; `ctest --preset <actual-test-preset>` if defined. MSBuild: `msbuild .\Project.sln /m /p:Configuration=Debug /p:Platform=x64` (replace names/config with actual project). Use warning levels, static analysis and AddressSanitizer when supported; inspect warnings rather than blanket suppression.
## C#
- Match SDK to global.json and project TFM, including legacy .NET Framework targeting packs where needed. For trusted SDK-style projects: `dotnet restore`, `dotnet build --no-restore`, `dotnet test --no-build`. Obtain approval before network restore. Legacy projects use matching MSBuild and test runner. Enable nullable where appropriate; dispose resources, avoid unsafe deserialization, test async cancellation and failure paths.
## Interop
Use explicit C ABI for native exports, `extern "C"`, fixed-width types, matched calling conventions, SafeHandle, documented string encoding, explicit allocator ownership and structure layout. Verify architecture match and test marshaling round trips, nulls, boundary values and failure paths. C++/CLI requires matching workload and target framework.
## Debugging
Use Visual Studio breakpoints, call stack, locals, modules and exception settings; configure symbol paths and verify PDB identity. Use mixed-mode debugging only on trusted authorized targets. Offline symbols first; approve symbol-server access. WinDbg is optional for authorized crash dumps; dumps may contain sensitive data. Never build or debug untrusted decompiled projects on the host.
## Acceptance
For owned source changes: review diff, build Debug/Release as relevant, run tests, record compiler/SDK, configuration, exit code and warnings. Separate build success from runtime success. Prefer small synthetic DLL fixtures for interop and static analysis, not third-party binaries. Report failures and rollback plan.

## V5 shared routing
Read `../../references/tool-selection.md` and the central tool catalog. Verify task-required tools using their upstream usage references and synthetic fixtures; route directory, archive, format, asset and authorized data-recovery tasks to the appropriate V5 specialists. No fixed client version.

## V6 native C and server work
For actual C projects, preserve .c compilation (MSVC /TC when appropriate), use fixed-width types, checked arithmetic, explicit ownership, socket cleanup and error handling; do not convert to C++ without approval. Route networking, account, game and persistence work through the V6 specialists. Test on owned loopback fixtures, never expose services by default.

## V6 integration
For authorized client/server work, read `../../references/server-workflow.md` and use the master routing for account, protocol, secure transport, game services, database and integration tasks. Preserve project-neutral discovery, tool verification and evidence boundaries.

## V7 shared workflow
Follow `../../references/v7-engineering-workflow.md`: environment readiness, project evidence, scoped plan, fixture verification, bounded repair and redacted knowledge. Report blocked capabilities and actual test status; no fixed client, language or version.

## Mandatory V8 documentation
Follow `../../references/v8-project-governance.md` and `engineering-documentation`: record material actions, results, redacted evidence, decisions, blockers and handoff in project Markdown. Do not overwrite existing documentation or log secrets.

## V10 shared engine contract
For large file inventories, format hints, evidence exports and hash comparisons when relevant to this specialist, use `../../references/v10-engineering-engine.md` after local acceptance. Do not substitute the engine for specialist analysis or bypass existing evidence and security gates.
