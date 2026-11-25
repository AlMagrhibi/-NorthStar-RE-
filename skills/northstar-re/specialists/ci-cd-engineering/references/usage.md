# Reproducible CI/CD: operating guide

Use Windows and Linux validation for portable scripts; pin reviewed action versions, least privilege, no secrets for untrusted PRs. Do not publish releases without approval; distinguish local tests from real host acceptance.

- Inputs: authorized project, task ID, scope and acceptance criteria.
- Record tool version, invocation and redacted output in SESSION-LOG.md and relevant documents.
- Use `../../../references/v8-project-governance.md` for scripts and gates; consult the central tool catalog for official sources.
- Output: reviewed changes, evidence manifest, limitations and next action.
- Acceptance: verify artifacts and tests independently; record blocked state if missing.
