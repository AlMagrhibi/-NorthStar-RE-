# V11 native command contract — 0.3.0 candidate
Use the absolute installed executable. No legacy interpreter or fallback. CLI is the primary agent interface; no automatic host delegation is assumed. Exit 0 means the command completed, 2 means partial/failed gate, 1 means error. A zero exit is not proof of semantic correctness. Inputs are local reviewed/redacted JSON, never secrets. Use stable trusted workspaces; path checks are not a security sandbox against concurrent filesystem mutation.

| Command | Interface |
|---|---|
| doctor / tools | `nsre doctor`; candidate PATH discovery, verify tools independently |
| init / resume / status | `nsre init --project DIR` (existing dir), then `nsre resume --project DIR` |
| scan | `nsre scan --root DIR --out NEW_FILE --jobs 4` (outside corpus) |
| index | `nsre index --root DIR --db DB_OUTSIDE_CORPUS --jobs 4 --mode strict` |
| export / diff | `nsre export --db DB --run 1`; `nsre diff --db DB --old 1 --new 2` |
| analyze / probe | `nsre probe FILE` or `nsre analyze FILE`; header hints only |
| hash | `nsre hash --project DIR --input JSON --reviewed-redacted` (`{"path":"relative/file"}`) |
| validate-docs / report | `nsre validate-docs --project DIR`; `nsre report --project DIR` |
| verify / quality-gate / progress | same project/input flags; use existing evidence, gate and task templates |
| dependency-audit | same flags; array of name/version/source/license/review_status; no vulnerability claim |
| session | same flags; session_id and next_action; no input lists last 100 records |
| checkpoint | same flags; explicit array of project-relative regular files; manual restoration |
| jobs | `--action validate`, `save` or `list`; task DAG template; no worker execution |
| memory | same flags; knowledge-record template; omit input to list last 100 |
| search | same flags; `{"query":"text"}` searches new SQLite records (bounded 100) |
| benchmark | same flags; `{"corpus":"CO","jobs":4}`; output under project reports, not inside corpus |
| api / mcp | `nsre api --project DIR` or `nsre mcp --project DIR`; stdio, read-only doctor/status |

`--input FILE --reviewed-redacted` required for mutating records; no input to list. Legacy JSON and old docs are preserved; no automatic import. New sessions and knowledge live in `.northstar/state/framework.sqlite`; status exposes both legacy session and new session history. Use latest reviewed session record when available. Existing V10 corpus index remains separate. Re-run strict hashes for evidence; metadata-cache is advisory only. Jobs is a persistent plan ledger, not autonomous scheduling or distributed execution. MCP implements legacy 2025-11-25 handshake only; compatibility with actual host must be tested; no HTTP listener. Installation/update/backup/uninstall are separate PowerShell wrappers, not engine commands. No project source is installed or changed by them.

Read specialist references before routing. Use Java Ghidra scripts and trusted native utilities for full PE/CLR analysis, diagnostics and format parsing; native probe does not replace full preflight. Do not bypass security controls or connect to production services. No project-specific version assumptions. Arabic reports by default.
