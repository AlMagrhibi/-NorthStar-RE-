# Project-scoped evidence and reusable engineering memory — reference and verification guide

Read exact installed-version docs before use. No automatic downloads or execution of untrusted code.

## JSON Schema (`json-schema`)
- Official: https://json-schema.org/
- Usage: https://json-schema.org/learn/getting-started-step-by-step
- Local check: Validate sample record against schema; reject missing evidence fields.

## Native engine (`nsre`)
- Use the installed Rust CLI and its V11 command contract.

## Git (`git`)
- Official: https://git-scm.com/download/win
- Usage: https://git-scm.com/download/win
- Local check: Git.Git candidate; confirm live

## Fixture and evidence
Use `nsre memory --project DIR --input RECORD --reviewed-redacted` on reviewed redacted records; validate schema and reject unproven claims without evidence.
Record expected versus observed results, exact tool version, exit status, source and output hashes.
