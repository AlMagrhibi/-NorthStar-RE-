---
name: game-assets-analysis
description: Analyze authorized textures, models, animations, maps, UI and media without execution.
---
# game-assets-analysis
Mandatory first: read `../environment-discovery-setup/SKILL.md`, `../../references/evidence-contract.md`, `../../references/tool-selection.md` and local `references/usage.md`. Inventory required capabilities and validate tools with fixtures; do not assume installed.

## Workflow
Classify content first. DDS: compare dimensions, pixel format, mip levels and alpha, use DirectXTex documentation; a safe example on owned fixture is `texconv -ft png -o <scratch> <fixture.dds>`. C3/ANI: review community model and animation reader source, map IDs to assets and record missing dependencies. DMAP/PUL/scene: inspect dimensions, layer references and coordinate conventions; validate in Tiled only after safe conversion. SWF: inspect tags and authorized resources in JPEXS without playback or script execution. PE resources: inspect owned fixture via Resource Hacker. Preserve originals, record conversion settings, compare metadata and disclose lossy conversions. Do not redistribute proprietary resources.

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
