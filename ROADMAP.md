# Roadmap and implementation ledger

Status is evidence-based. All instruction modules require agent/tool execution; source code alone is not Windows acceptance. Current local reports are linked below. A new row is never complete merely because it appears here.

| Component | Status | Source | Tests / evidence | Limitations and acceptance criteria |
|---|---|---|---|---|
| Inventory, hashing, index/export/diff | IMPLEMENTED; Linux tested | main.rs, index.rs | 11-test native suite; [report](docs/reports/TESTS-AR.md) | Windows CI/desktop acceptance separate; cache advisory |
| Bounded PE/signature probe | IMPLEMENTED, limited | probe.rs | Header fixtures | Full PE/CLR parser planned; require bounded malformed-file corpus |
| Persistent engineering memory | IMPLEMENTED, local append records | framework.rs | Session/isolation/DB refusal tests | 100-record reads; no vector retrieval or legacy import |
| Evidence and quality gates | IMPLEMENTED | framework.rs | Hash/gate/checkpoint regressions | Semantic review remains human/agent responsibility |
| Crash recovery | PARTIAL | SQLite WAL + staged manager | Transaction/lifecycle fixtures | No exact resume or power-loss transaction; require kill/restart fault campaign |
| Multi-project isolation | IMPLEMENTED, organizational | --project and DB identity | Isolation/path tests | Not an OS tenant sandbox; hostile race defenses incomplete |
| Local API | EXPERIMENTAL | rpc.rs | JSON-RPC fixtures | Read-only stdio; no network authentication/listener |
| MCP server | EXPERIMENTAL | rpc.rs | Legacy initialize fixture | 2025-11-25 candidate; require real host interoperability |
| Plugin architecture | PLANNED | none | none | Define versioned ABI and isolated permission model |
| Adapter SDK | PLANNED | integration docs only | no live agents | Ship SDK contracts plus host/version fixtures |
| Distributed workers | PLANNED | none | none | Explicit isolation, cancellation and reproducible result protocol |
| Job scheduler | PLANNED execution; plan ledger exists | framework.rs jobs | DAG tests | Validate DAG is not scheduling; require resource-aware dispatch tests |
| Resource governor | PARTIAL | bounded batches, 1..64 jobs | CLI bounds | No global CPU/RAM scheduler; require enforceable per-job budgets |
| Advanced binary analysis | PLANNED native; specialist-guided now | specialists + probe | Header hints only | Full parsers and safe format fixtures before promotion |
| Performance optimization | MEASUREMENT IN PROGRESS | benchmark lab | [measurements](docs/reports/PERFORMANCE-AR.md) | No extrapolation or Windows claims from Linux |
| Knowledge base expansion | RESEARCH PRESERVED | references and 108-entry catalog | Catalog validation | Upstream links/licensing need ongoing review |
| Automated research integration | INSTRUCTION ONLY | research specialist | No autonomous fetch service | Require provenance, opt-in updates and safe content handling |
| Installer/update/rollback | IMPLEMENTED candidate | install/Manage.ps1 | Windows lifecycle CI required | Per-user; explicit updates; kill recovery and desktop agent tests separate |

## خارطة التطوير بالعربية

المنفذ فعليًا هو الجرد والتجزئة والفهرسة والحوكمة والذاكرة المحلية، ضمن الحدود الموثقة. الإضافات الموزعة وSDK والجدولة التنفيذية ومحلل PE/CLR الكامل أهداف تطوير وليست إمكانيات مكتملة. توجد حالات جزئية وتجريبية موضحة في الجدول، مع المصدر والاختبارات والحدود ومعيار القبول. راجع تقارير الأدلة قبل اعتماد أي حالة.

## Approved next-stage work (development branch)

See [engineering foundations](skills/northstar-re/references/next-engineering-foundations.md) for requirements and acceptance gates.

| Work item | Status |
|---|---|
| FW-001 Tool discovery | Candidate source + synthetic unit test; CI/Windows acceptance pending |
| FW-002 True incremental index | PLANNED; strict full hashing unchanged |
| FW-003 Generic semantic adapter | DESIGN; reviewed generalized source and synthetic acceptance pending |
| FW-004 Resource measurements and recovery | PLANNED extension to existing lab |
| FW-005 MCP/API host interoperability | PLANNED verification; existing read-only candidate unchanged |

No new published release or desktop acceptance is implied.

## FW-011–FW-020 — engineering validation lab

[Implementation and acceptance ledger](skills/northstar-re/references/engineering-validation-lab.md). All ten work items have initial offline Rust helpers on the development branch; full engines and runtime acceptance remain pending. No new release is implied.
