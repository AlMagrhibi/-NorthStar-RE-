# Review changes independently: operating guide

Inspect diff for correctness, test coverage, security, compatibility and error paths. Delegate to independent reviewer only if available; otherwise label self-review. Record findings and re-review fixes.

- Inputs: authorized project, task ID, scope and acceptance criteria.
- Record tool version, invocation and redacted output in SESSION-LOG.md and relevant documents.
- Use `../../../references/v8-project-governance.md` for scripts and gates; consult the central tool catalog for official sources.
- Output: reviewed changes, evidence manifest, limitations and next action.
- Acceptance: verify artifacts and tests independently; record blocked state if missing.
