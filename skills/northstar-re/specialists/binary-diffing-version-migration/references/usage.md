# Version-aware binary and schema comparison — reference and verification guide

Read exact installed-version docs before use. No automatic downloads or execution of untrusted code.

## Diaphora (`diaphora`)
- Official: https://github.com/joxeankoret/diaphora
- Usage: https://github.com/joxeankoret/diaphora
- Local check: Check IDA edition and native tooling compatibility; compare two locally built versions.

## Ghidra (`ghidra`)
- Official: https://github.com/NationalSecurityAgency/ghidra/releases
- Usage: https://github.com/NationalSecurityAgency/ghidra/releases
- Local check: Read application.properties and installed docs; trusted fixture headless

## Rizin (`rizin`)
- Official: https://rizin.re/
- Usage: https://book.rizin.re/
- Local check: Check version and identify architecture of known PE fixture.

## Git (`git`)
- Official: https://git-scm.com/download/win
- Usage: https://git-scm.com/download/win
- Local check: Git.Git candidate; confirm live

## ImHex (`imhex`)
- Official: https://github.com/WerWolv/ImHex
- Usage: https://docs.werwolv.net/imhex
- Local check: Open a synthetic binary read-only; inspect offsets, bytes, entropy and an audited local pattern.

## DUMPBIN (`dumpbin`)
- Official: https://learn.microsoft.com/cpp/build/reference/dumpbin-reference
- Usage: https://learn.microsoft.com/cpp/build/reference/dumpbin-reference
- Local check: Developer PowerShell; inspect trusted PE fixture

## Fixture and evidence
Two locally built revisions show known API and behavior differences.
Record expected versus observed results, exact tool version, exit status, source and output hashes.
