# Persistent memory and database schemas

[Documentation index](../README.md) · [العربية](../ar/memory.md)

## Project state
init creates 20 non-destructive documentation templates and legacy JSON state files. Existing docs remain intact. Reviewed session, knowledge and plan records are appended to .northstar/state/framework.sqlite. records(id TEXT PRIMARY KEY, kind TEXT NOT NULL, payload TEXT NOT NULL, created_ms TEXT NOT NULL); application_id=1314083398; user_version=1. Reads list at most 100 newest records. search is substring matching, not semantic/vector retrieval.

## Corpus index
[Authoritative SQL](../../skills/northstar-re/engine/nsre/src/schema.sql) defines runs, files and issues. application_id=0x4e535245 and user_version=1 identify indexes. A fresh strict run records fresh hashes; cache reuse is advisory. DB integrity is checked by schema identity and regression fixtures; there is no online repair service.

## Evidence and checkpoints
Evidence lists project-relative paths and expected SHA-256. Checkpoints copy a reviewed explicit list, reject internal .git/.northstar paths and verify copied hashes. They do not capture an entire project or automatically restore it. Review and restore manually. Back up project data separately from framework installation.

## Session continuity and migration
resume returns legacy session-state.json and new SQLite session history. Use the newest reviewed session record, document next action and known blockers. Legacy JSON knowledge has no automatic migration path. Keep all .northstar directories out of framework commits and release archives.
