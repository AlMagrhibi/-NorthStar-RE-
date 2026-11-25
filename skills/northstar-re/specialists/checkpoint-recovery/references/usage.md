# Create and validate checkpoints: operating guide

Create manifests and copies of explicitly approved regular files before modifications; verify hashes. Restore only after review and approval; do not reset unrelated changes.

- Inputs: authorized project, task ID, scope and acceptance criteria.
- Record tool version, invocation and redacted output in SESSION-LOG.md and relevant documents.
- Use `../../../references/v8-project-governance.md` for scripts and gates; consult the central tool catalog for official sources.
- Output: reviewed changes, evidence manifest, limitations and next action.
- Acceptance: verify artifacts and tests independently; record blocked state if missing.
