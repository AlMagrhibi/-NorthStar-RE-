# Authoritative templates

The 20 project Markdown templates and JSON evidence/plan/session/knowledge examples remain in [skills/northstar-re/references/templates](../skills/northstar-re/references/templates). Rust embeds those paths directly. Do not duplicate or move them without updating include_str!, installer and regression tests.

Template placeholders are not evidence. init preserves existing files; quality gates refuse untouched document templates. Review inputs, redact private data and pass --reviewed-redacted when required. Legacy JSON knowledge is not automatically migrated into new SQLite records.
