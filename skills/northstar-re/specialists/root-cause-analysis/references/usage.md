# Diagnose root causes: operating guide

Reproduce using trusted fixtures, gather logs, test competing hypotheses, isolate the smallest cause, patch minimally and add regression evidence. Stop repeated ineffective retries.

- Inputs: authorized project, task ID, scope and acceptance criteria.
- Record tool version, invocation and redacted output in SESSION-LOG.md and relevant documents.
- Use `../../../references/v8-project-governance.md` for scripts and gates; consult the central tool catalog for official sources.
- Output: reviewed changes, evidence manifest, limitations and next action.
- Acceptance: verify artifacts and tests independently; record blocked state if missing.
