# Versioning and historical continuity / سياسة الترقيم والتاريخ

**Decision date:** 2026-11-27. **Status:** owner-approved display identity; technical migration pending.

| Dimension | Value |
|---|---|
| Product | NorthStar RE Framework |
| Public display label | 0.012 |
| Target development SemVer | 0.0.12-dev.1 |
| Target release SemVer | 0.0.12 (only after approval) |
| Current tree metadata at decision | 12.0.0-dev.1 |
| Independent engine crate | 0.3.0 |
| Historical installed baseline | v11.0.0-rc.2 |
| Engine domains | 12, unchanged |
| Queue schema | independently versioned; not product version |

The display label is not a SemVer identifier; Cargo, tags, dependency comparisons and installers must use valid three-component SemVer. The old V-series describes internal development iterations, not accepted production major versions. The 0.x series reflects pre-production maturity. No chronological release history is fabricated.

## Migration contract

The developer must migrate metadata, installer/fetch/version guards, workflow triggers, packaging manifest, release request and validators together. Preserve historic release access and rollback; use explicit migration logic, because 0.0.12 sorts below 11.x/12.x under SemVer. Separate build identity, product version and compatibility schema. Keep existing master and engine IDs and all 42 requirements/gates. Do not reset, force-push, delete tags, rebuild RC2 or enable publication. Validate negative paths, development artifact creation, upgrades/rollback in isolated profiles, and GitHub CI. Never silently downgrade an installed newer package.

## Historical continuity

- Retain original commits, authorship, timestamps and tags.
- Preserve old reports as dated evidence with original version names.
- Record new work as dated entries with tested SHA; never re-label historical test evidence.
- Keep 0.012 identity separate from the uncompleted final-release acceptance.
- Advance versions on meaningful development increments; pre-production labels do not certify completeness.

**بالعربية:** تم اعتماد 0.012 للعرض و0.0.12 للترقيم التقني بعد الترحيل. أرقام المحركات والمخططات مستقلة. لا تغيير لتاريخ Git أو الوسوم القديمة أو تقاريرها؛ يمنع النشر حتى قبول المالك والبوابات الموحدة.
