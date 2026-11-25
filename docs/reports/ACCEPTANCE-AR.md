# بوابات القبول والعوائق

اجتاز مرشح الإصدار الأول جميع وظائف GitHub Actions في [التشغيل 45692129487](https://github.com/AlMagrhibi/-NorthStar-RE-/actions/runs/45692129487)، ونُشرت حزم فعلية على [v11.0.0-rc.1](https://github.com/AlMagrhibi/-NorthStar-RE-/releases/tag/v11.0.0-rc.1). يجمع مرشح 2 القياسات والتوثيق وبيانات النسخة؛ بوابة نشره تعيد جميع الفحوص وتربط manifest بالـcommit، ولا تعتمد هذا التقرير بديلًا عن دليل التشغيل الخاص بإصداره.

| البوابة | الدليل المتحقق | الحدود |
|---|---|---|
| سلامة حزمة الإدخال وبصماتها | PASS، 175 ملفًا | البصمة محسوبة؛ لا بصمة ناشر مستقلة مزودة |
| ماستر و52 تخصصًا و108 أدوات | PASS، registry/catalog validation | التخصصات تعليمات داخلية وليست محللات Rust مستقلة |
| Cargo.lock والبناء والجودة | PASS Linux وWindows MSVC | Rust 1.91.1؛ MSRV 1.89 بعد إصلاح SQLite |
| اختبارات Rust | 11 على Linux، 10 على Windows؛ إعادة خلال source build | لا ادعاء قياس تغطية أسطر |
| nsre.exe وdoctor | PASS على Windows Server 2022 | لا يثبت تثبيت الأدوات الخارجية |
| تثبيت نظيف وidempotency | PASS عبر المثبت الحقيقي | في ملف مستخدم مؤقت معزول |
| رفض الملكية والعبث بالبصمة | PASS | الملفات غير المملوكة محفوظة |
| Backup/update/rollback/uninstall | PASS | لا تعديل لبيانات المشاريع |
| فشل upgrade واستعادته تلقائيًا | PASS، حقن فشل استبدال التسجيل | لا يمثل انقطاع طاقة في لحظة عشوائية |
| Build-Engine من السورس | PASS عبر rustup MSVC المحدد وبوضع offline | الاعتماديات كانت متاحة في cache CI |
| ربط CRT واعتماديات EXE | PASS، dumpbin | Windows 11 الحقيقي يحتاج قبولًا منفصلًا |
| Zero Python | PASS للجرد والفحوص | الأدوات الخارجية غير الموافقة خارج المسار Native |
| ثغرات وتراخيص وSBOM | PASS بعد تحديث SQLite | فحص تصريحات وتسجيل أدلة، لا شهادة أمان/مراجعة قانونية |
| سعات 1k/10k/100k/1M | PASS على Linux، strict وcache ودقة كل سجل | لا تعميم على Windows أو ملفات كبيرة |
| روابط ومراجع | تحقق محلي + 153 upstream URL | روابط محجوبة/خاصة موثقة UNVERIFIED |
| GitHub Actions وحزم الإصدار | PASS للمرشح الأول؛ مرشح 2 معلق على بوابته | راجع manifest وChecksums وCI للإصدار المختار |
| Windows 11 فعلي | UNVERIFIED | يتطلب جلسة على جهاز Windows 11 |
| FreePi/Codex/Pi/Claude | UNVERIFIED | لا إصدار مضيف مختبر فعليًا |
| About/Topics/Social Preview/حماية الفرع | BLOCKED BY AVAILABLE CAPABILITY | [التفاصيل والإعدادات الجاهزة](REPOSITORY-SETTINGS-AR.md) |

المستودع بقي Private والتاريخ محفوظ، ولم يضف ترخيص مفتوح المصدر. الوظائف المطبقة والتجريبية والمخططة في ROADMAP.md. النشر كمرشح لا يعني جاهزية إنتاج أو اكتمال التكامل الفعلي.
