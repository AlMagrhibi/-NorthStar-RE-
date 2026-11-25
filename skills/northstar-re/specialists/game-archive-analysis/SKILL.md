---
name: game-archive-analysis
description: Recognize and safely examine standard archives and custom game containers.
---
# game-archive-analysis
Mandatory first: read `../environment-discovery-setup/SKILL.md`, `../../references/evidence-contract.md`, `../../references/tool-selection.md` and local `references/usage.md`. Inventory required capabilities and validate tools with fixtures; do not assume installed.

## Workflow
Use header evidence rather than extension. For supported standard archives use `7z l <owned-archive>` then `7z t <owned-archive>` on a trusted isolated parser; check installed help. For custom WDF/WDB/DNP/TQ-like containers use source and schema references as hypotheses, never assume interchangeable layouts. Inspect entry count, offset, length, alignment and compression flags; verify bounds and non-overflow before reading entries. Reject absolute paths, traversal, symlinks, reparse points and duplicate/case-colliding output names. Limit total output, count, expansion ratio and nesting; never auto-run extracted files. Record manifest before extracting permitted content into a fresh directory, then hash each output. Unsupported or protected archive: report blocker; no access-control bypass.

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
