# بوابات القبول والعوائق

| البوابة | الحالة الحالية | الدليل أو المطلوب |
|---|---|---|
| سلامة حزمة الإدخال وبصماتها | PASS | تدقيق المصدر |
| الماستر و52 تخصصًا و108 أدوات | PASS للمصدر | سجل specialists والكتالوج الأصلي |
| Cargo.lock وبناء واختبارات Linux | PASS | 11 اختبارًا وClippy وبناء release |
| Windows MSVC وnsre.exe وdoctor | PENDING | GitHub Actions وسجلات runner |
| Clean install/update/backup/rollback/uninstall | PENDING | Test-Installer.ps1 على Windows الحقيقي داخل CI |
| Windows 11 فعلي | UNVERIFIED | جلسة قبول على جهاز Windows 11 |
| FreePi/Codex/Pi/Claude | UNVERIFIED | مسار وإصدار واكتشاف واستدعاء فعلي |
| Zero Python | PASS للجرد | فحص CI يمنع إدخاله |
| فحص ثغرات الاعتماديات | PENDING | cargo-audit الحالي وقاعدة RustSec الحديثة |
| تراخيص الاعتماديات | PASS لتصريحات SPDX | حقوق التوزيع والإشعارات تحتاج مراجعة مستقلة |
| ترخيص المنظومة | OWNER DECISION | لم يعتمد ترخيص مفتوح المصدر |
| الروابط والتوثيق والبصمات الجديدة | PENDING | Validate-Package النهائي |
| GitHub Actions وRelease Assets | PENDING | لا يعتمد النجاح قبل قراءة النتائج |
| حماية الفرع وإعدادات About/social preview | BLOCKED BY AVAILABLE CAPABILITY | الموصل يتيح ملفات Git وليس تعديل إدارة المستودع؛ المتصفح بلا جلسة مسجلة |

الوظائف المطبقة والتجريبية والمخططة مفصلة في ROADMAP.md. نشر المرشح لا يعني اكتمال قبول Windows 11 أو الأيجنتات. لا يوجد قبول نهائي للإنتاج قبل إغلاق العوائق بالأدلة.
