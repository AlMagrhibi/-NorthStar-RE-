# Resume from persisted state: operating guide

Read HANDOFF, task plan and current session state before changing files. Validate stale claims against actual files and Git diff; save reviewed state at boundaries. Never assume session memory is authoritative.

- Inputs: authorized project, task ID, scope and acceptance criteria.
- Record tool version, invocation and redacted output in SESSION-LOG.md and relevant documents.
- Use `../../../references/v8-project-governance.md` for scripts and gates; consult the central tool catalog for official sources.
- Output: reviewed changes, evidence manifest, limitations and next action.
- Acceptance: verify artifacts and tests independently; record blocked state if missing.
