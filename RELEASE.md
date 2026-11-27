# Release engineering

The owner authorized initial candidate v11.0.0-rc.2 in .github/release-request.json. A push to main or the exact approved tag runs the complete pipeline. Windows-only native formatting, check, tests, Clippy, build and doctor, package validation, Windows installer lifecycle, dependency audit and license checks must pass before packaging. Only the final publish job has contents:write; pull requests cannot publish. Existing releases are never overwritten. Change the explicit release request for a new owner-authorized candidate.

Pinned action commit SHAs and Rust 1.91.1 are used. cargo-audit is pinned to 0.23.0 with its own locked build. Dependency-review SaaS availability depends on repository plan; locked dependency audit is mandatory independently. Framework licensing remains owner-controlled; no public Pages deployment or visibility change is configured.

Artifacts: NorthStar-RE-Framework-v11-Windows11-Source.zip, NorthStar-RE-Framework-v11-Windows11-x64.zip, nsre-windows-x64.exe, SHA256SUMS.txt, release-manifest.json, sbom.json, build-report.md, test-report.md, audit.json and installer.log. Package-Release.ps1 rejects binary assembly outside Windows and without required evidence. It regenerates internal checksums, extracts and verifies the binary archive. The publish job independently rehashes all assets and binds the release manifest to the triggering commit. Checksums do not replace publisher authentication or code signing; Authenticode signing is not configured.

A source-only package can be generated with install/Package-Release.ps1 -SourceOnly -Output NEW_DIR on a supported PowerShell host. It is explicitly not a Windows build or ready-to-install certification. No successful binary is published when mandatory checks fail. A hosted Windows Server runner does not certify an actual Windows 11 desktop or agent integration; the candidate remains prerelease.

## هندسة النشر بالعربية

النشر الأول مرخص كمرشح إصدار فقط، ويعتمد على نجاح الاختبارات والبناء وdoctor ودورة تثبيت Windows وفحص الاعتماديات والتراخيص. لا تعاد كتابة tag أو الإصدارات السابقة. توضح بصمات الحزم وmanifest رقم commit والدليل؛ لا توجد شهادة توقيع تنفيذي. قبول Windows 11 الحقيقي والأيجنتات يظل UNVERIFIED إلى أن توجد سجلات فعلية. إعداد workflow أو رفع السورس وحده لا يثبت نجاح النشر.

Future candidate pipelines run exclusively on Windows, including security and publication. Historical releases retain their original evidence. See [Windows platform decision](docs/architecture/windows-platform.md).
