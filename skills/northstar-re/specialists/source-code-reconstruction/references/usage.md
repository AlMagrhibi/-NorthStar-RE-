# Clean-room compatible implementation for authorized software — reference and verification guide

Read exact installed-version docs before use. No automatic downloads or execution of untrusted code.

## Ghidra (`ghidra`)
- Official: https://github.com/NationalSecurityAgency/ghidra/releases
- Usage: https://github.com/NationalSecurityAgency/ghidra/releases
- Local check: Read application.properties and installed docs; trusted fixture headless

## ILSpy GUI (`ilspy`)
- Official: https://github.com/icsharpcode/ILSpy/releases
- Usage: https://github.com/icsharpcode/ILSpy/releases
- Local check: About/version and known managed fixture

## LLVM / Clang (`llvm-clang`)
- Official: https://llvm.org/
- Usage: https://clang.llvm.org/docs/
- Local check: Verify toolchain version and target; compile a small owned source fixture.

## Doxygen (`doxygen`)
- Official: https://www.doxygen.nl/
- Usage: https://www.doxygen.nl/manual/
- Local check: Generate local documentation from tiny owned project.

## GoogleTest (`googletest`)
- Official: https://github.com/google/googletest
- Usage: https://google.github.io/googletest/
- Local check: Build and run positive/negative assertions in owned test project.

## xUnit.net (`xunit`)
- Official: https://github.com/xunit/xunit
- Usage: https://xunit.net/docs/getting-started
- Local check: Restore reviewed package, run dotnet test on a minimal test project.

## Fixture and evidence
Fixture equivalence is demonstrated; no protected feature bypass or unauthorized copying.
Record expected versus observed results, exact tool version, exit status, source and output hashes.
