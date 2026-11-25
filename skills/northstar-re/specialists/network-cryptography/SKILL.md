---
name: network-cryptography
description: Design and validate secure transport and authorized cryptographic fixtures.
---
# network-cryptography

Mandatory: read `../environment-discovery-setup/SKILL.md`, `../../references/evidence-contract.md`, `../../references/tool-selection.md`, `references/usage.md` and the central tool catalog. Discover and verify required tools before work; install only after scoped approval. Use C, C++ or C# as indicated by actual project evidence.

## Workflow
Identify whether an owned protocol uses TLS, application-level authenticated encryption or a legacy scheme based on documentation and synthetic fixtures. Prefer OS-supported TLS and reviewed libraries over custom crypto. Validate trust chain, service identity, expiration and revocation per policy; never disable certificate verification. Keep key material in approved secret storage; do not place it in source, logs, packet captures or knowledge base. Use known-answer tests with disposable supplied test keys, negative tamper tests and nonce uniqueness checks. Treat legacy cryptography as a compatibility risk requiring explicit review; no extraction of third-party session keys, defeating protections, credential decryption or live traffic interception.

## Candidate tool IDs
openssl, dotnet, sslstream, windows-cng, wireshark. Choose only task-required capabilities.

## Acceptance
Record authorization, input hashes when applicable, tool path/version/provenance, actual commands, fixture results, output paths, confidence and limitations. Do not claim runtime verification without execution. Arabic user-facing reports.

## V7 shared workflow
Follow `../../references/v7-engineering-workflow.md`: environment readiness, project evidence, scoped plan, fixture verification, bounded repair and redacted knowledge. Report blocked capabilities and actual test status; no fixed client, language or version.

## Mandatory V8 documentation
Follow `../../references/v8-project-governance.md` and `engineering-documentation`: record material actions, results, redacted evidence, decisions, blockers and handoff in project Markdown. Do not overwrite existing documentation or log secrets.

## V10 shared engine contract
For large file inventories, format hints, evidence exports and hash comparisons when relevant to this specialist, use `../../references/v10-engineering-engine.md` after local acceptance. Do not substitute the engine for specialist analysis or bypass existing evidence and security gates.
