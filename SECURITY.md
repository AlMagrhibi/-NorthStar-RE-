# Security policy

Report suspected vulnerabilities privately to the owner through an existing trusted channel. Do not place secrets, exploit material or private target data in issues. No formal security certification, bounty or response SLA is asserted. Supported candidate: V11 / nsre 0.3.0; legacy snapshots are historical.

The engine never loads/executes target binaries. It uses bounded header reads, local SQLite and explicit project paths. Reparse/symlink paths are refused in relevant operations; these checks are not an adversarial filesystem sandbox and do not eliminate TOCTOU races or hardlink exposure. Use trusted directories and isolate untrusted analysis. Metadata cache is advisory. Checksums verify bytes, not authenticity or semantic correctness. Optional tools require independent verification.

Installer operations are per-user, ownership-gated, staged and logged. Backups retain old registration; no project state is removed. Windows process-kill/power-loss recovery requires manual inspection of retained directories. Online updates use authenticated GitHub acquisition and explicit approval, never token URLs.

CI runs locked tests, selected secret-pattern scanning, cargo-audit, SPDX allowlist checks and SBOM generation. Results can be pending or failed; a configured scan is not a clean bill of health. See [security report](docs/reports/SECURITY-AR.md).

## سياسة الأمان
أبلغ المالك سرًا عبر قناة موثوقة، دون رفع أهداف أو أسرار. لا توجد شهادة أمان أو ضمان استجابة. فحوص المسارات والبصمات لا تعوض عزل العينات أو مراجعة الثقة. التثبيت للمستخدم وبملكية واضحة ونسخ احتياطية؛ انقطاع العملية يحتاج مراجعة مستقلة. الحالة الفعلية للفحص في التقرير وليست مستنتجة من وجود CI.
