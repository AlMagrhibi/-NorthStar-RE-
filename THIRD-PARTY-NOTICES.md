# Third-party notices and license audit

Framework licensing is UNRESOLVED / owner-controlled. No open-source license or redistribution permission is granted by this repository setup. Cargo publish=false and license=UNLICENSED are retained. Obtain owner approval before distribution beyond the authorized private repository.

Direct dependencies: rayon, walkdir, sha2, serde_json and rusqlite (bundled SQLite). Cargo.lock records the exact graph. scripts/Write-Sbom.ps1 generates CycloneDX package identities, versions, sources and declared SPDX license expressions from cargo metadata; these declarations require legal review and preservation of upstream notices for distribution. No unknown executable is bundled.

CI rejects missing/unapproved declared license expressions using scripts/Check-Licenses.ps1. Legacy slash-separated alternatives are retained as upstream metadata; an available permissive alternative is selected for technical review. SPDX allowlisting is a technical gate, not legal advice or a replacement for notice obligations. Tools in the 108-entry catalog are external references with independent licenses and terms; none is granted rights by this framework.

ترخيص المنظومة لم يعتمد كترخيص مفتوح المصدر. تراخيص الاعتماديات مستقلة ولا تمنح حق إعادة توزيع المشروع. يلزم الحفاظ على إشعارات upstream ومراجعة الحقوق قبل التوزيع خارج النطاق الخاص المصرح.

[Exact locked dependency notice files](third-party/README.md) are copied unchanged from crate distributions. SQLite is bundled through libsqlite3-sys; its public-domain provenance and upstream notices must be retained and reviewed as applicable.
