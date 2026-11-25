# NorthStar RE Skills V8 — Engineering Intelligence Suite / Windows 11
ماستر واحدة `northstar-re` + 45 تخصصًا داخليًا. انسخ مجلد `skills/northstar-re` كاملاً إلى مسار المهارات في برنامجك. استدعِ `/skill:northstar-re` إذا كان مدعومًا. أول خطوة إلزامية: اكتشاف البيئة والأدوات، تجهيز المطلوب بعد الموافقة، ثم التحقق الفعلي. 100 مدخلًا في كتالوج الأدوات والمراجع؛ منها تطبيقات ومشروعات سورس ومراجع معرفية. لا تقييد لنوع أو إصدار Client. Conquer تخصص اختياري فقط. الأدوات ليست مرفقة كبرامج مثبتة.

## Windows 11 PowerShell
```powershell
py -3 skills/northstar-re/specialists/environment-discovery-setup/scripts/inventory.py --out reports/environment.json
py -3 skills/northstar-re/specialists/client-data-discovery/scripts/file_inventory.py 'C:/samples/owned-client' --out reports/files.json
py -3 skills/northstar-re/specialists/reverse-engineering-preflight/scripts/preflight.py 'C:/samples/owned.dll' --authorized 'Owned test DLL' --out reports/preflight.json --md reports/PREFLIGHT.md
py -3 -m unittest discover -s tests -v
```
## References
- Master routing: `skills/northstar-re/SKILL.md`
- Global tool selection: `skills/northstar-re/references/tool-selection.md`
- Catalog and setup: `skills/northstar-re/specialists/environment-discovery-setup/references/`
- Conquer optional reference: `skills/northstar-re/specialists/conquer-domain-knowledge/references/conquer-formats.md`
- Every new specialist has a local tool usage guide with upstream links and fixture verification.

Inventory is read-only and cannot certify tool readiness. File inventory is bounded; output outside target; use only on authorized copies. No live Windows host, community builds or actual client sample tested during packaging. No universal decryptor or access-control bypass; documented recovery with authorized supplied keys only. Historical V2 documents are archival, not active instructions.

## V6 server expansion
ستة تخصصات جديدة: account-server-engineering، network-protocol-analysis، network-cryptography، game-server-engineering، database-engineering، client-server-interoperability. دعم C/C++/C# حسب المشروع الفعلي، ومراجع Winsock وWireshark/TShark وMySQL وSQLite وSQL Server وPostgreSQL. اقرأ `skills/northstar-re/references/server-workflow.md`. لا تشغيل أو تثبيت تلقائي غير مصرح به، ولا اختبار فعلي على Windows في بيئة إعداد الحزمة.

## V7 engineering suite
12 additional specialists, 84 catalog entries, project-local reviewed knowledge, task DAG validator, engineering workflow and live acceptance checklist. Python 3.10+ required for new helper scripts. The master routes through instructions, not a standalone scheduler.

```powershell
py -3 skills/northstar-re/specialists/intelligent-task-orchestrator/scripts/validate_plan.py skills/northstar-re/references/templates/task-plan.json
# Copy the record template, replace placeholders, review/redact, then:
py -3 skills/northstar-re/specialists/persistent-knowledge-base/scripts/knowledge_record.py C:/owned-project C:/reviewed-record.json --reviewed-redacted
```
Read `docs/WINDOWS11-ACCEPTANCE.md` and `skills/northstar-re/references/v7-engineering-workflow.md`. The local tests are not a live Windows certification. Tool URLs are references; installation and licensing require local verification.

## V8 — التوثيق والاستمرارية والجودة

45 تخصصًا + ماستر واحدة، 100 أداة ومرجع، 20 قالب Markdown و10 واجهات سكريبتات محلية. التوثيق إلزامي أثناء العمل، مع حالة جلسة قابلة للاستعادة، سجل الأدلة، ونقاط الاستعادة. لا تنفذ السكريبتات أوامر أدوات خارجية ولا تستعيد الملفات تلقائيًا.

- [عقد حوكمة المشروع](skills/northstar-re/references/v8-project-governance.md) — الاستخدام، شكل JSON، بوابات الجودة والحدود.
- [قوالب توثيق المشروع](skills/northstar-re/references/templates/project-docs/) — لا تستبدل الملفات الموجودة.
- [اختبار Windows 11](docs/WINDOWS11-ACCEPTANCE.md) — لم ينفذ على جهاز Windows فعلي بعد.
- [نشر GitHub](docs/GITHUB-PUBLISH.md) — خطوات إنشاء مستودع خاص.
- [الأمان](SECURITY.md) و[المساهمة](CONTRIBUTING.md) و[سجل التغييرات](CHANGELOG.md).

```powershell
py -3 skills/northstar-re/specialists/engineering-documentation/scripts/init_project.py C:/owned-project
py -3 skills/northstar-re/specialists/engineering-documentation/scripts/project_health.py C:/owned-project
py -3 -m unittest discover -s tests -v
```

لا تعادل سلامة SHA-256 صحة محتوى الأدلة. يلزم مراجعة هندسية حقيقية، والتحقق من الأدوات محليًا. CI مقترح ولم يعمل بعد؛ لا يوجد ادعاء تشغيل حي.

## التثبيت والترقية من GitHub
المصدر المعتمد: https://github.com/AlMagrhibi/-NorthStar-RE- . اتبع [دليل التثبيت والترقية](docs/INSTALL-UPGRADE.md) الذي يتضمن نصًا جاهزًا للأيجنت، مع مراجعة التغييرات والنسخ الاحتياطي واختبارات ما قبل وبعد التفعيل. المستودع خاص ويتطلب مصادقة مصرحًا بها.
