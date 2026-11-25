# Layered verification and regression prevention — reference and verification guide

Read exact installed-version docs before use. No automatic downloads or execution of untrusted code.

## GoogleTest (`googletest`)
- Official: https://github.com/google/googletest
- Usage: https://google.github.io/googletest/
- Local check: Build and run positive/negative assertions in owned test project.

## xUnit.net (`xunit`)
- Official: https://github.com/xunit/xunit
- Usage: https://xunit.net/docs/getting-started/v3/getting-started
- Local check: Restore reviewed package, run dotnet test on a minimal test project.

## CTest (`ctest`)
- Official: https://cmake.org/cmake/help/latest/manual/ctest.1.html
- Usage: https://cmake.org/cmake/help/latest/manual/ctest.1.html
- Local check: Run tests only for trusted source

## Visual Studio Test (`vstest`)
- Official: https://learn.microsoft.com/visualstudio/test/vstest-console-options
- Usage: https://learn.microsoft.com/visualstudio/test/vstest-console-options
- Local check: Legacy or VS test workflows

## .NET SDK/runtime (`dotnet`)
- Official: https://dotnet.microsoft.com/download
- Usage: https://dotnet.microsoft.com/download
- Local check: Inspect --list-sdks and --list-runtimes, project global.json

## AddressSanitizer (`asan`)
- Official: https://learn.microsoft.com/cpp/sanitizers/asan
- Usage: https://learn.microsoft.com/cpp/sanitizers/asan
- Local check: Compile a clean owned fixture with sanitizer enabled; check compiler and architecture support.

## Native engine (`nsre`)
- Use the installed Rust CLI and its V11 command contract.

## Fixture and evidence
Any required failed, skipped or unexecuted test blocks production-ready claims.
Record expected versus observed results, exact tool version, exit status, source and output hashes.
