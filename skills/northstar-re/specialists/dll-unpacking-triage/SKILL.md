---
name: dll-unpacking-triage
description: Classify authorized DLL packaging, encryption, compression and managed obfuscation, and validate legitimate owner-provided recovery workflows.
---
# DLL packing and encryption triage

**Mandatory shared gate:** Read `../environment-discovery-setup/SKILL.md`, `../../references/evidence-contract.md`, and `references/operational-reference.md` before work. Verify required tools, install only approved missing dependencies, and use actual target evidence rather than a fixed client or version.

- Require scope, hash and provenance; never load an unknown DLL on the host. Start with preflight and static PE headers, CLR metadata, section entropy, imports, exports, signatures and resources. Entropy is not proof of encryption. Distinguish native, managed, mixed, NativeAOT, compressed container and damaged PE.
- Use official Detect It Easy (https://github.com/horsicq/Detect-It-Easy) for read-only signatures in isolation. Record exact version, matching signature, section and confidence; conflicting detections remain unknown.
- Encryption recovery: for user-owned data with documented format and supplied legitimate key, identify algorithm, IV/nonce, authentication tag and format from documentation. Use standard cryptographic libraries, validate authentication before accepting plaintext, keep keys out of logs. Never guess keys or defeat licensing, anti-cheat or DRM. No universal decryptor exists.
- For a known public compression format, use the documented vendor-supported extraction method on an authorized copy in isolation; validate bounds, hashes and output type, then repeat preflight. Never overwrite the original.
- For own trusted debug builds, Visual Studio debugger and owner-provided diagnostic dumps may be used to inspect known test data. For unknown samples, do not execute or attempt to bypass protection; escalate to qualified isolated forensic review. Treat dumps as sensitive, avoid credentials, do not upload them.
- Record SHA-256 original and derived file, transformation provenance, PE/CLR validation, expected behavior and remaining unknowns. Route managed to ILSpy, native to Ghidra, mixed to both. Failure to recover plaintext is a valid result.

## V5 shared routing
Read `../../references/tool-selection.md` and the central tool catalog. Verify task-required tools using their upstream usage references and synthetic fixtures; route directory, archive, format, asset and authorized data-recovery tasks to the appropriate V5 specialists. No fixed client version.

## V6 integration
For authorized client/server work, read `../../references/server-workflow.md` and use the master routing for account, protocol, secure transport, game services, database and integration tasks. Preserve project-neutral discovery, tool verification and evidence boundaries.

## V7 shared workflow
Follow `../../references/v7-engineering-workflow.md`: environment readiness, project evidence, scoped plan, fixture verification, bounded repair and redacted knowledge. Report blocked capabilities and actual test status; no fixed client, language or version.

## Mandatory V8 documentation
Follow `../../references/v8-project-governance.md` and `engineering-documentation`: record material actions, results, redacted evidence, decisions, blockers and handoff in project Markdown. Do not overwrite existing documentation or log secrets.

## V10 shared engine contract
For large file inventories, format hints, evidence exports and hash comparisons when relevant to this specialist, use `../../references/v10-engineering-engine.md` after local acceptance. Do not substitute the engine for specialist analysis or bypass existing evidence and security gates.
