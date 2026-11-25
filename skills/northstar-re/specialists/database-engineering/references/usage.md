# Operational reference — database-engineering

- SQLite CLI: https://www.sqlite.org/cli.html ; verify `sqlite3 --version`, use `sqlite3 :memory: "SELECT 1;"` for smoke test.
- MySQL Windows installation: https://dev.mysql.com/doc/refman/8.4/en/windows-installation.html ; MySQL Shell: https://dev.mysql.com/downloads/shell/ ; verify `mysql --version` / `mysqlsh --version`.
- MySQL Workbench: https://dev.mysql.com/doc/workbench/en/ ; confirm About and connect only to disposable instance.
- SQL Server sqlcmd: https://learn.microsoft.com/en-us/sql/tools/sqlcmd/sqlcmd-download-install ; SSMS: https://learn.microsoft.com/en-us/ssms/ ; verify exact installed variant and use synthetic database.
- PostgreSQL psql: https://www.postgresql.org/docs/current/app-psql.html ; verify `psql --version`.
- Deliver sanitized schema map, migration plan, restore test, transaction tests and DB compatibility matrix.

Use official sources, validate compatibility and publisher; do not execute unknown project downloads. Local test data only; redact secrets.
