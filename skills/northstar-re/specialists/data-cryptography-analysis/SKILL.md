---
name: data-cryptography-analysis
description: Distinguish encoding, compression, encryption and obfuscation; perform authorized documented data recovery.
---
# data-cryptography-analysis
Mandatory first: read `../environment-discovery-setup/SKILL.md`, `../../references/evidence-contract.md`, `../../references/tool-selection.md` and local `references/usage.md`. Inventory required capabilities and validate tools with fixtures; do not assume installed.

## Workflow
Work on copies only. Record headers, length, entropy as weak evidence, known format signatures, metadata and provenance. High entropy is not proof of encryption. Identify encoding or documented compression first. For confirmed encryption, require authorized algorithm, parameters and supplied key through a secure channel; never log keys. Use established crypto libraries and known-answer fixtures; verify authentication tags where applicable and do not present unauthenticated output as valid. If parameters/keys are missing or a protected access control is involved, stop recovery and report what is known. No key harvesting, brute-force cracking, anti-cheat/DRM/licensing bypass. Record method, input/output hashes, validation evidence and unresolved uncertainties. Reference .NET cryptography documentation https://learn.microsoft.com/en-us/dotnet/standard/security/cryptography-model and OpenSSL docs.

## Deliverables
Record scope, SHA-256, tool/version, steps, observed evidence, limitations and validation; write outputs only to the approved workspace. Arabic user-facing report by default. Never execute unknown target content.

## V6 integration
For authorized client/server work, read `../../references/server-workflow.md` and use the master routing for account, protocol, secure transport, game services, database and integration tasks. Preserve project-neutral discovery, tool verification and evidence boundaries.

## V7 shared workflow
Follow `../../references/v7-engineering-workflow.md`: environment readiness, project evidence, scoped plan, fixture verification, bounded repair and redacted knowledge. Report blocked capabilities and actual test status; no fixed client, language or version.

## Mandatory V8 documentation
Follow `../../references/v8-project-governance.md` and `engineering-documentation`: record material actions, results, redacted evidence, decisions, blockers and handoff in project Markdown. Do not overwrite existing documentation or log secrets.

## V10 shared engine contract
For large file inventories, format hints, evidence exports and hash comparisons when relevant to this specialist, use `../../references/v10-engineering-engine.md` after local acceptance. Do not substitute the engine for specialist analysis or bypass existing evidence and security gates.
