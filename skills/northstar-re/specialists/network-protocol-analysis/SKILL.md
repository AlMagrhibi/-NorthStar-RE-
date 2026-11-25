---
name: network-protocol-analysis
description: Document authorized client-server transport and message schemas using local fixtures.
---
# network-protocol-analysis

Mandatory: read `../environment-discovery-setup/SKILL.md`, `../../references/evidence-contract.md`, `../../references/tool-selection.md`, `references/usage.md` and the central tool catalog. Discover and verify required tools before work; install only after scoped approval. Use C, C++ or C# as indicated by actual project evidence.

## Workflow
Determine transport, endpoints, protocol ownership and permission before any capture. Default to passive inspection of user-provided sanitized captures or traffic between owned synthetic local endpoints. Record framing, byte order, bounds, encoding and message direction from observed evidence. TCP is a stream; test split and coalesced frames, truncated input, unknown message types, version mismatch and timeout. Separate observation from inference. Do not intercept third-party traffic, capture credentials, inject/replay authentication packets or bypass encryption/access controls. Do not publish captures containing personal data. Build bounded parsers only for documented or owned protocol fixtures.

## Candidate tool IDs
wireshark, tshark, pktmon, winsock, dotnet, imhex. Choose only task-required capabilities.

## Acceptance
Record authorization, input hashes when applicable, tool path/version/provenance, actual commands, fixture results, output paths, confidence and limitations. Do not claim runtime verification without execution. Arabic user-facing reports.

## V7 shared workflow
Follow `../../references/v7-engineering-workflow.md`: environment readiness, project evidence, scoped plan, fixture verification, bounded repair and redacted knowledge. Report blocked capabilities and actual test status; no fixed client, language or version.

## Mandatory V8 documentation
Follow `../../references/v8-project-governance.md` and `engineering-documentation`: record material actions, results, redacted evidence, decisions, blockers and handoff in project Markdown. Do not overwrite existing documentation or log secrets.

## V10 shared engine contract
For large file inventories, format hints, evidence exports and hash comparisons when relevant to this specialist, use `../../references/v10-engineering-engine.md` after local acceptance. Do not substitute the engine for specialist analysis or bypass existing evidence and security gates.
