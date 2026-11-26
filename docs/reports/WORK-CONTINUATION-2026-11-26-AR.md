# استئناف التنفيذ — 2026-11-26

المرجع: NORTHSTAR-RE-V11-MASTER-HANDOVER.md، قُرئ بالكامل. استؤنف العمل من PR #9 الفعلي، دون إعادة البناء من البداية أو تعديل مشروع CO/SOR الخاص.

## خط الأساس المتحقق

- المستودع Public وصلاحيات push/admin متاحة. الفرع engineering/engine-orchestrator-foundation.
- HEAD عند الاستلام: `8d9a1a4529a0f533bd81d173c21475e8cefc27ff`، ويتضمن إصلاح PowerShell الموثق في التسليم. لم نكرر الإصلاح.
- CI لهذا الأساس: [45871621232](https://github.com/AlMagrhibi/-NorthStar-RE-/actions/runs/45871621232). نجح parser/integrity وRust build/tests/Clippy وSecurity أثناء الفحص؛ واكتملت Native/Security/Verified Windows distributions بنجاح؛ Publish skipped كما هو متوقع للـPR.
- البيئة المحلية Linux x86_64 وRust 1.91.1، وليست Windows 11. الاختبارات المحلية دليل تطوير فقط؛ لا تغير منصة المنتج المستهدفة.

## الباتش الفعلي

| المتطلب | التنفيذ | الدليل/الحد |
|---|---|---|
| NSO-02/04 | schema 2، ترحيل صريح transactional من v1، اعتماديات وظائف دائمة | v1 يبقى قابلًا للتنفيذ؛ يتطلب الترحيل توقف جميع العملاء ونسخة SQLite متسقة للرجوع |
| NSO-04 | منع الدورات بالمراجع إلى وظائف موجودة، رفض self/unknown/duplicate | لا forward references أو تعديل تبعيات بعد الإدراج |
| NSO-04 | حجز مفاتيح حصرية ضمن transaction، وتقديم تدريجي للمنتظرين | مفاتيح تعاونية داخل DB واحدة، وليست أقفال OS أو حجز موارد خارجية |
| NSO-05/07 | فشل التابع عند parent cancelled/failed/interrupted أو hash فاسد | لا تنفيذ تابع على نتيجة غير متحققة؛ لا automatic crash detection |
| NSO-06/07 | adapter typed لجمع الملفات المحلية داخل الطابور | SHA-256 فعلي عند التشغيل، حدود collector الحالية، لا shell/network/GUI |
| NSO-07 | إعادة حساب بصمة المصدر المتغير بدل إعادة استخدام دليل سابق | لا cache مطبق ولا ادعاء اكتمال سياسة cache |
| NSO-08 | إضافة Windows smoke لتسلسل الوظائف والجمع وتغيير المصدر | يلزم نجاح CI على commit الباتش قبل اعتباره CI_VERIFIED |

## التحقق المحلي

داخل `skills/northstar-re/engine/nsre`:

```text
cargo fmt --all -- --check
cargo check --workspace --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo build --release --locked
```

بعد الباتش: 22 unit + 3 engine_cli + 2 framework_cli + 4 security_regression = 31 اختبارًا فريدًا على Linux. النتائج المحلية لا تثبت عدد Windows، الذي يضم اختبارات خاصة بمنصته. ستُسجل أي تغييرات لاحقة في هذا التقرير قبل رفع الباتش.

## العوائق الحالية وحالة الإصدار

جميع engine complete=false محفوظة. لم يتحقق اكتمال المحركات الـ12 أو الإصدار النهائي. لا دمج أو نشر أو تثبيت تحديث ناقص في FreePi.

- NSO-05/06: لا تزال leases/reconciliation التلقائية، running cancellation، Job Objects وقياس CPU/RAM/IO الفعلي تحتاج تنفيذًا واختبارات أعطال.
- DSC-04/05: UIA semantic selectors وwindow capture وmulti-monitor/DPI لم تكتمل.
- DSC-01–06 وNSO-03/08: لا يوجد وصول متاح من هذه الجلسة إلى جهاز Windows 11 للمستخدم أو FreePi العامل عليه. أدوات التحكم المتاحة تخص بيئة Linux في الجلسة، ولا تثبت اتصالًا بجهاز المستخدم. لا تشغيل G04/G05 ولا تعديل عميل أصلي.
- قدرات Binary/Semantic/Algorithm/Data/Compatibility/Recovery ما زالت تحتاج إغلاق متطلباتها الخاصة الواردة في وثيقة التسليم؛ نجاح dispatch للـ12 لا يغلقها.
- قبول Windows 11 وFreePi والترقية/rollback للحزمة النهائية يبقى مطلوبًا بعد التنفيذ، ولا يُستبدل بنجاح Windows Server CI.

## واجهة الأيجنت الأصلية

أضيف nsre_capabilities وnsre_queue_status إلى stdio MCP، مع بقاء الواجهة قراءة فقط. قراءة الطابور مربوطة بالمسار .northstar/state/jobs.sqlite داخل المشروع المحدد، دون قبول مسار DB من الأيجنت. اختبار CLI فعلي يثبت handshake/discovery واستهلاك نتيجة موثقة ويرفض extra arguments وpath traversal ويثبت عزل مشروع ثانٍ دون إنشاء حالته. لا تعديل إعدادات FreePi أو اعتماداته؛ قبول المضيف نفسه UNVERIFIED. الأوامر والعقد في adapters/README.md.

باتش الجدولة والجمع: commit `96d5424b895681bd6b9491b35c563050b44fcf3c`، [CI 45872473167](https://github.com/AlMagrhibi/-NorthStar-RE-/actions/runs/45872473167). سجلات واجهة الأيجنت والإصلاحات التالية تخص أحدث commit، ويجب فحص CI الخاص به منفصلًا.

مراجعة التوافق: يُرفض collector input schema 2 في DB v1 حتى الترحيل الصريح، كي لا يستهلكه عامل قديم لا يدعم عقده. أضيف اختبار تنافس متزامن لثمانية عمال على مفتاح حصري واحد؛ حجز عامل واحد فقط، وبعد interruption موثق يصبح المورد متاحًا للعامل التالي. يعلن registry الآن عقد collector القابل للجدولة دون تغيير complete flags.

## سجل أدلة — إصلاح CI للـNative job (T03 / NSO-02)

| الحقل | القيمة |
|---|---|
| المعرّف | T03 (طابور دائم) + NSO-02 (اختبار تنافس/إعادة تشغيل) |
| ما تغيّر | `tests/queue_cli.rs`: تمييز مجلد الـfixture بعدّاد ذرّي داخل العملية، لأن pid+nanos يتكرر بين اختبارات متوازية على Windows فتفشل `create_dir` بـ`AlreadyExists` (os error 183) وتفشل `concurrent_processes_claim_distinct_durable_jobs`. أُعيد بناء `docs/SHA256SUMS.txt` على البايتات الفعلية. |
| الـcommit | `fa3f5a0fdee08d1aecffcf33978f65826c1270bb` (الإصلاح) و`5248409235974cc70c63397c251374dd2de10d25` (البيان) |
| أمر الاختبار | `cargo test --test queue_cli --locked` ثم `cargo test --locked` و`cargo clippy --locked --all-targets -- -D warnings` و`pwsh scripts/Validate-Package.ps1 -CheckHashes` |
| البيئة | Windows 11 x64 · Rust 1.91.1 MSVC · محلي عبر FreePi CLI |
| النتيجة | 4/4 لاختبارات queue_cli، ثم 49 اختبارًا ناجحًا عبر الحِزم، Clippy نظيف، Package PASS |
| CI | [45904671068](https://github.com/AlMagrhibi/-NorthStar-RE-/actions/runs/45904671068) — success على `5248409` (Native + Security) |
| حدود الدليل | Windows Server 2022 runner وليس جهاز Windows 11؛ لا يغلق NSO-02 كاملًا (lease/reconciliation ما زالت مفتوحة) |

## سجل أدلة — Queue Schema V3: الملكية والـlease والـfencing والمصالحة (ADD-003 / ADD-036 / ADD-037 / NSO-02 / NSO-05)

| الحقل | القيمة |
|---|---|
| ما تغيّر | `skills/northstar-re/engine/nsre/src/queue.rs`: schema v3 (ترحيل ذري من v1/v2 يرفض وجود عامل نشط)، هوية العامل، lease بمدة قابلة للضبط (1s–1h)، نبض، fencing token لكل مهمة، `lease_events` كأثر دائم، `reconcile` يحوّل الـlease المنتهي إلى `interrupted` ولا يعيد التشغيل تلقائيًا، `cancel --running` كطلب إلغاء، ورفض نشر أي نتيجة من عامل فقد الـlease. الحقول الجديدة داخل `status.lease`. |
| العقود | `adapters/README.md` (قسم Queue schema v3) و`src/rpc.rs` (`database_schemas` = 1,2,3). |
| الـcommits | هذه الدفعة (انظر `git log` أدناه) |
| أوامر الاختبار | `cargo test --locked` · `cargo clippy --locked --all-targets -- -D warnings` · `cargo fmt` · `pwsh scripts/Validate-Package.ps1 -CheckHashes` |
| البيئة | Windows 11 x64 · Rust 1.91.1 MSVC |
| النتيجة | 57 اختبارًا ناجحًا (42 وحدة + 15 تكامل/CLI)، منها 8 جديدة: نبض يرفض المالك الغريب، مصالحة الـlease المنتهي إلى `interrupted`، رفض نتيجة عامل متأخر بـ`LEASE_LOST`، إلغاء جارٍ يُهمل النتيجة ويحرّر الـlease، ترحيل v2→v3 دون فقد مهمة، وعقد CLI (`run-one --worker/--lease-ms`، `heartbeat`، `reconcile`، `cancel --running`) |
| حدود الدليل | لم تُختبر بعد: قتل عامل حقيقي أثناء التنفيذ، هوية إنشاء العملية (PID reuse)، Job Objects، مهلة تنفيذ صلبة، تقرير تقدّم من العامل. هذه بنود الدفعة التالية وليست مُنجزة. |
