# تقرير التجميع

تم إنتاج وتشغيل محرك Linux release من السورس باستخدام Rust 1.91.1 وCargo.lock، مع SQLite مدمج. نجاح Linux لا يثبت إنتاج nsre.exe. خط GitHub Actions يبني Windows MSVC ويشغّل --version وdoctor واختبارات المحرك والمثبت، ثم يجهز الحزم فقط بعد نجاح بوابات الأمان والاختبارات.

الحالة المحلية: PASS لبناء Linux. حالة Windows: PENDING إلى أن تؤكد سجلات CI النتيجة. لا توجد حزمة Windows محلية مجهولة المصدر أو محولة شكليًا. لا Authenticode signing؛ تحقق من المصدر الموثوق والبصمات. يحتوي release-manifest.json المنشور على commit الذي بُنيت منه الحزمة، وترافقه تقارير التنفيذ.
