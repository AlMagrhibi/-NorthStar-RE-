---
name: graphics-game-engine-engineering
description: Graphics API and asset pipeline diagnosis for authorized Windows 11 engineering workflows.
---
# Graphics API and asset pipeline diagnosis
## Entry gate
Read `../../references/evidence-contract.md`, `../../references/v7-engineering-workflow.md` and the environment catalog. Verify task-required tools; no assumption of installation, client release or architecture. Work on authorized copies; no protection bypass, credential extraction or unauthorized service access.
## Candidate tool IDs
renderdoc, pix, texconv, texdiag, c3studio, tiled. Choose by task and verified readiness; see references/usage.md.
## Procedure
1. Discover actual API, renderer, shader format and asset bindings; do not assume DirectX or OpenGL.
2. Start with offline asset metadata and owned demo; select RenderDoc or PIX only where supported by actual graphics stack.
3. Compare draw calls, resource formats, texture dimensions and frame timings on self-built scenes.
4. Record GPU/driver and feature levels; test rendering changes with known image or numerical tolerance.
## Deliverable
Graphics architecture, asset dependency map and capture evidence.
## Acceptance gate
Locally built triangle scene renders expected output; no capture of protected online games.

## V7 shared workflow
Follow `../../references/v7-engineering-workflow.md`: environment readiness, project evidence, scoped plan, fixture verification, bounded repair and redacted knowledge. Report blocked capabilities and actual test status; no fixed client, language or version.

## Mandatory V8 documentation
Follow `../../references/v8-project-governance.md` and `engineering-documentation`: record material actions, results, redacted evidence, decisions, blockers and handoff in project Markdown. Do not overwrite existing documentation or log secrets.

## V10 shared engine contract
For large file inventories, format hints, evidence exports and hash comparisons when relevant to this specialist, use `../../references/v10-engineering-engine.md` after local acceptance. Do not substitute the engine for specialist analysis or bypass existing evidence and security gates.
