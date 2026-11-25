# أرشيف حالة حزمة الإدخال — ليس تقرير التحقق الحالي

هذا المحتوى يصف التسليم الأصلي قبل هندسة المستودع. راجع [تقارير القبول الحالية](reports/ACCEPTANCE-AR.md).

# NorthStar RE Framework V11 — حالة التسليم

**Source Release Candidate / NOT WINDOWS VERIFIED**

- القرار المعتمد: محرك مركزي Native Rust مستقل عن أي مشروع، ماستر واحدة و52 تخصصًا، لا اعتماد على Python ولا fallback. حافظنا على C++/C# وJava لـGhidra.
- حزمة V10 الأصلية كانت تحتوي 15 سكريبت تشغيل و9 ملفات اختبارات Python؛ أزيلت من الحزمة الجديدة. لا تُعتبر اختبارات V10 السابقة إثباتًا لتكافؤ الإصدار الجديد.
- سورس V10 للـscan/index/export/diff/probe موجود؛ إضافة وحدة framework للحوكمة وذاكرة SQLite وواجهة RPC/MCP للقراءة فقط. أدوات تثبيت PowerShell لا تحتاج إلى Python. يوجد 20 قالب مستند مدمج في السورس.
- أضيف CI لبناء واختبار Windows/Linux. لم يُنفذ في هذه البيئة؛ Cargo/rustc غير متوفرين، ولا يمكن الوصول إلى مستودع Rust. لا EXE ولا Cargo.lock، ولا ادعاء نجاح compilation أو benchmark. لم نُثبت الحزمة على جهاز المستخدم ولم ننشرها على GitHub.
- الاستبدال الوظيفي ليس تكافؤًا كاملاً: تحليل PE/CLR التفصيلي واكتشاف 108 أداة واختبارات تثبيتها كانت أوسع في الأدوات القديمة؛ V11 يوفر probe واكتشاف PATH محدود ويحتاج أدوات Windows ومراجعة بشرية. وظائف إدارة المشروع جديدة ولم تُختبر على Windows. استرجاع البيانات القديمة من JSON يحتاج ترحيلًا يدويًا. لا يوجد scheduler تنفيذي أو plugin ABI أو distributed workers أو استئناف دقيق بعد الانقطاع. الـMCP مرشح لنسخة 2025-11-25، لا توافق مثبت مع FreePi أو إصدار 2026.
- الأداء: خطة اختبارات 10k/100k/1M ملف، baseline strict مع hashes، ثم metadata-cache للمقارنة الاسترشادية؛ تسجيل wall time وpeak RSS وCPU وسرعة القرص والأخطاء. لا تثبيت حدود زمنية وهمية.

## مراجع هندسية
- Cargo build/test وlockfile: https://doc.rust-lang.org/cargo/commands/cargo-build.html و https://doc.rust-lang.org/cargo/commands/cargo-test.html
- MCP legacy lifecycle: https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/docs/specification/2025-11-25/basic/lifecycle.mdx
- MCP versioning: https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/docs/specification/2026-07-28/basic/versioning.mdx
- Rust install: https://rustup.rs/
- بحث V10 محفوظ في V10-RESEARCH-AR.md كمرجع تاريخي، لا دليل اختبار V11.
- أضيف سكريبت CI لتجميع ZIP مستقل يتضمن EXE وCargo.lock بعد نجاح الاختبارات. لم يُشغّل هذا المسار، ولا يُعد هذا الملف الحالي نسخة EXE جاهزة.
