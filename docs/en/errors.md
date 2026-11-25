# Exit codes and troubleshooting reference

[Documentation index](../README.md) · [العربية](../ar/errors.md)

| Surface | Value | Meaning and next action |
|---|---|---|
| CLI | 0 | Completed command; inspect semantic output |
| CLI | 1 | Invalid input or runtime failure; read redacted stderr |
| CLI | 2 | Partial inventory or failed quality gate; review errors/blocked fields |
| API | -32700 | Invalid JSON |
| API | -32600 | Invalid request/ID |
| API | -32601 | Unknown method/tool |
| API | -32602 | Invalid parameters |
| API | -32000 | Initialization/runtime/project binding error |

Installer failures use a nonzero PowerShell exit with structured installation.jsonl evidence. No framework-specific stable E-number catalog is implemented. Ownership failures preserve the existing skill. A failed staged upgrade retains/restores prior registration; interrupted process/power loss still needs inspection of retained stage/prior directories. Atomic directory renames plus atomic registration replacement do not constitute a single filesystem transaction across different volumes.

Do not retry through a different runtime. Preserve diagnostics locally, redact paths/credentials before support, and never upload private targets.
