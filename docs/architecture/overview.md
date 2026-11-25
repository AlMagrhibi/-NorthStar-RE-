# Architecture overview

```mermaid
flowchart TD
 A["Coding agent"] --> B["northstar-re master"]
 B --> C["52 internal specialist guides"]
 B --> D["Native nsre engine"]
 D --> E["Bounded inventory and index"]
 D --> F["Project governance and memory"]
 E --> G["Corpus SQLite and JSONL"]
 F --> H["Project .northstar state"]
 D --> I["Read-only stdio API / MCP candidate"]
```

[Module contracts](../en/architecture.md) · [العربية](../ar/architecture.md)

```mermaid
flowchart TD
 A["Authenticated package"] --> B["Integrity and compatibility checks"]
 B --> C["Staged central engine"]
 C --> D["Doctor and backup"]
 D --> E["Register master"]
 E --> F{"Registration succeeded?"}
 F -->|Yes| G["Reload and invoke"]
 F -->|No| H["Restore prior registration"]
```

Stages use owned paths and per-user installation. Recovery from a process kill remains an explicit acceptance concern.
