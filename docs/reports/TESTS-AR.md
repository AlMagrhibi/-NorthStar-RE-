# تقرير الاختبارات

## نتائج محلية فعلية — Linux x64 / Rust 1.91.1

- PASS: cargo fmt --all -- --check بعد تنسيق المصدر.
- PASS: cargo check --workspace --locked.
- PASS: cargo test --workspace --locked — 11 اختبارًا: 3 وحدات، 2 دورة فهرسة، 2 حوكمة، 4 انحدار أمني.
- PASS: cargo clippy --workspace --all-targets --locked -- -D warnings.
- PASS: cargo build --release --locked.
- PASS: الجرد وبصمات الحزمة الأصلية. التحقق النهائي للحزمة الجديدة موثق منفصلًا.

التغطية الوظيفية تشمل hash معروف، منع كتابة scan داخل المصدر أو استبدال ناتج موجود، strict/cache/export/diff، رفض قاعدة غريبة، عزل المشاريع وحفظ المستندات، رفض دورة DAG وهروب المسارات، رفض أدلة ببصمة خاطئة، نسخ checkpoint مطابق دون تعديل الأصل، رفض التكرار والمسارات الداخلية، منع نجاح قوالب الجودة غير المكتملة، ورفض symlink لحالة المشروع على Unix.

لا يوجد ادعاء قياس تغطية الأسطر. Windows CI يشغل 10 اختبارات لأن اختبار symlink خاص بـUnix. scripts/Test-Installer.ps1 اختبار حقيقي لمحرك Windows يجرب clean install ورفض الملكية وidempotency وdoctor وbackup وupdate وrollback وuninstall. حالته PENDING حتى فحص سجلات GitHub Actions. Windows Server runner لا يساوي جهاز Windows 11 فعليًا. تكامل FreePi/Codex/Pi/Claude وكل إصدار مضيف UNVERIFIED.

## نتائج CI الأولى والإصلاحات

نجحت بوابات Rust وEXE وdoctor على Windows Server 2022 (10 اختبارات) وLinux (11). فشلت دورة المثبت بسبب تفسير Windows PowerShell 5.1 لملف ps1 عربي بلا UTF-8 BOM؛ عولجت جميع سكريبتات PowerShell بترميز UTF-8 BOM وقراءات UTF-8 صريحة. لا يعد بناء Windows وحده نجاح دورة التثبيت. أعيدت اختبارات Rust وClippy والبناء بعد تحديث SQLite الأمني؛ CI جديد مطلوب للقبول النهائي.
