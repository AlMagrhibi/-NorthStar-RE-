# Source evaluation and version-aware reference research — reference and verification guide

Read exact installed-version docs before use. No automatic downloads or execution of untrusted code.

## Git (`git`)
- Official: https://git-scm.com/download/win
- Usage: https://git-scm.com/download/win
- Local check: Git.Git candidate; confirm live

## Native engine (`nsre`)
- Use the installed Rust CLI and its V11 command contract.

## Microsoft PE/COFF format (`pe-spec`)
- Official: https://learn.microsoft.com/windows/win32/debug/pe-format
- Usage: https://learn.microsoft.com/windows/win32/debug/pe-format
- Local check: Cross-check headers and field sizes against known PE fixture.

## Conquer Online Development Wiki (`conquer-wiki`)
- Official: https://conquer-online.github.io/wiki/
- Usage: https://conquer-online.github.io/wiki/files/formats/dat.html
- Local check: Reference only; record exact page, date and documented variant; compare to sample evidence.

## JSON Schema (`json-schema`)
- Official: https://json-schema.org/
- Usage: https://json-schema.org/learn/getting-started-step-by-step
- Local check: Validate sample record against schema; reject missing evidence fields.

## Fixture and evidence
At least one primary reference per material claim where available; explicitly flag inaccessible or unverified sources.
Record expected versus observed results, exact tool version, exit status, source and output hashes.
