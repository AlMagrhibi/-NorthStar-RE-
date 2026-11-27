# Engineering roadmap and implementation ledger

**Development identity 0.012** — evidence snapshot 2026-11-27 at [`d917cb5`](https://github.com/AlMagrhibi/-NorthStar-RE-/commit/d917cb562e9248e9f5a924eda42e311899149acd). The technical migration from `12.0.0-dev.1` is pending. All engine registry completion flags are false. This ledger does not certify final acceptance.

| Track | Current evidence | Next acceptance obligation |
|---|---|---|
| A — Orchestration and supervisor | Queue V5, leases, DAG, real process fixture tests | SUP-001–008, owned process-tree stop, resource budgets, fault injection, durable evidence |
| B — WISM and connection mobility | Session and context discovery, fail-closed desktop guards | Authorized user worker, authenticated IPC, session identity, dynamic monitoring, reconnect and local parity |
| C — Adaptive input and vision | Existing native desktop/UIA/capture candidates | Layout mapping, verified effects, modal/readiness, DPI and cache; actual interactive fixture |
| D — Packaging | Development artifacts, protected publication; CI green at snapshot | Product SemVer migration, compatibility/rollback, acceptance before authorization |
| E — Evidence | Four-group matrix | Raw sanitized evidence, exact source/build hashes, independent verdicts |
| Engines | Twelve IDs, candidate offline operations | Each engine's full contract, fixtures, cross-engine integration |
| FreePi/SSH | Context detection reported; GitHub MCP separate | NorthStar CLI/MCP submission, real interactive effect, disconnect/reconnect |
| Unified release | Not authorized | All ADD-001–042, NSO/DSC/WISM/T01–42, actual Windows 11 and upgrade/rollback |

## Execution order

Independent tracks proceed in parallel where dependencies allow. Shared typed contracts and policy define integration boundaries. Test locally and on hosted CI; interactive and FreePi acceptance remain distinct. See the [unified directive](docs/NORTHSTAR-V12-EXECUTION-DIRECTIVE.md), [evidence matrix](docs/reports/TEST-EVIDENCE-MATRIX.json), [architecture](docs/architecture/overview.md), [version policy](docs/VERSIONING.md).

## Historical ledger

Earlier FW-001–025 and V-series plans remain preserved in the [previous immutable roadmap](https://github.com/AlMagrhibi/-NorthStar-RE-/blob/d917cb562e9248e9f5a924eda42e311899149acd/ROADMAP.md) and original reports. Their original evidence scope is unchanged; the unified directive is the active requirements baseline.

**العربية:** لا تعتمد حالة مكتمل بناءً على وجود السورس أو نجاح fixture واحدة. لا يُسقط أي متطلب معتمد عند إعادة التوثيق، ولا تُستخدم نتائج CI بدل القبول الفعلي.
