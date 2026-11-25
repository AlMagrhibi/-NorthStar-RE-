# Static classification and corroborated binary understanding — reference and verification guide

Read exact installed-version docs before use. No automatic downloads or execution of untrusted code.

## Microsoft PE/COFF format (`pe-spec`)
- Official: https://learn.microsoft.com/windows/win32/debug/pe-format
- Usage: https://learn.microsoft.com/windows/win32/debug/pe-format
- Local check: Cross-check headers and field sizes against known PE fixture.

## Detect It Easy (`die`)
- Official: https://github.com/horsicq/Detect-It-Easy/releases
- Usage: https://github.com/horsicq/Detect-It-Easy/releases
- Local check: Check release/build and signature scan on synthetic fixture

## PE-bear (`pe-bear`)
- Official: https://github.com/hasherezade/pe-bear/releases
- Usage: https://github.com/hasherezade/pe-bear/releases
- Local check: Optional static PE viewer, verify About and fixture

## PEStudio (`pestudio`)
- Official: https://www.winitor.com/
- Usage: https://www.winitor.com/
- Local check: Open known PE fixture offline; compare imports and sections; verify edition.

## YARA (`yara`)
- Official: https://github.com/VirusTotal/yara
- Usage: https://yara.readthedocs.io/
- Local check: Match a benign synthetic marker against one positive and one negative fixture.

## Rizin (`rizin`)
- Official: https://rizin.re/
- Usage: https://book.rizin.re/
- Local check: Check version and identify architecture of known PE fixture.

## Cutter (`cutter`)
- Official: https://cutter.re/
- Usage: https://cutter.re/docs/
- Local check: Open known PE fixture read-only; verify architecture and sections.

## IDA Free (`ida-free`)
- Official: https://hex-rays.com/ida-free
- Usage: https://docs.hex-rays.com/
- Local check: Confirm edition, licensing and offline restrictions; inspect owned arithmetic fixture without uploading.

## Binary Ninja (`binary-ninja`)
- Official: https://binary.ninja/
- Usage: https://docs.binary.ninja/
- Local check: Confirm edition and local-only processing; open locally built fixture and compare symbols.

## Ghidra (`ghidra`)
- Official: https://github.com/NationalSecurityAgency/ghidra/releases
- Usage: https://github.com/NationalSecurityAgency/ghidra/releases
- Local check: Read application.properties and installed docs; trusted fixture headless

## DUMPBIN (`dumpbin`)
- Official: https://learn.microsoft.com/cpp/build/reference/dumpbin-reference
- Usage: https://learn.microsoft.com/cpp/build/reference/dumpbin-reference
- Local check: Developer PowerShell; inspect trusted PE fixture

## Sigcheck (`sigcheck`)
- Official: https://learn.microsoft.com/sysinternals/downloads/sigcheck
- Usage: https://learn.microsoft.com/sysinternals/downloads/sigcheck
- Local check: Offline signature inspection; no VirusTotal upload

## Fixture and evidence
Known x86/x64 and managed fixture matches expected machine, sections and exports.
Record expected versus observed results, exact tool version, exit status, source and output hashes.
