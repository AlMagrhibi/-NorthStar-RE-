# تنفيذ إدخال سطح المكتب وUIA والتقاط النوافذ

استئناف من `70dd9e402caee2cbe5324b1e85a6c3f196bd1b0d` وفق تكليف المالك باستكمال التحكم الفعلي. يحافظ الباتش على المحركات الاثني عشر والماستر الواحد ولا يعدل تثبيت FreePi أو CO/SOR الخاص.

## ما أضيف فعليًا

- كتابة Unicode، اختصارات مفاتيح محددة، وتحريك المؤشر؛ التحقق من foreground/PID، الإيقاف مع Escape/تدخل المستخدم، وإطلاق المفاتيح بعد الإدخال الجزئي.
- UI Automation أصلي: شجرة محدودة، إخفاء بيانات password، اختيار AutomationId فريد، واستدعاء InvokePattern. لا قراءة Value/Text patterns.
- التقاط PNG لنافذة مختارة عبر PrintWindow في child مملوك بمهلة خمس ثوانٍ، وبصمة مستقلة ونشر دون overwrite. مراقبة بإطارات متتابعة؛ ليست WGC أو بث فيديو.
- تسلسل أوامر السطح بمسمّى mutex محلي وتطبيق DPI awareness على thread.
- واجهة الأيجنت الاختيارية `--desktop-control`: أوامر typed، apply صريح، ربط المخرجات والتدقيق بالمشروع، إرسال النص عبر stdin، مهلة وحجم مخرجات محددان. لا shell عشوائي ولا GUI writes من الطابور.
- fixture Windows موسع لزر UIA وpassword exclusions، صورة نافذة مغطاة بلون معروف، واختبار أثر الكتابة والنقر دون الاكتفاء بعدّ أحداث الإدخال.

## الأدلة وحدودها

اختبارات Linux مكملة فقط. استُخدم target Windows MSVC لفحص أنواع وحدة السطح وClippy في crate مؤقت يعتمد نفس ملفات المصدر؛ لا يساوي ذلك ربط executable كامل أو تشغيل Windows. CI الفعلي هو مرجع البناء/الربط/الحزم وفحص fixture.

تُحفظ نتائج HostedProbe في desktop-control.json. إذا كان سطح المكتب غير تفاعلي فالنتيجة BLOCKED، وإذا رفض foreground يبقى keyboard/click acceptance غير متحقق؛ لا تعتبر overall CI success إثباتًا لهما. لا توجد جلسة متاحة على Windows 11 للمستخدم أو FreePi، ولا اختبار على play.exe الخاص.

ما زال WGC/DirectX/minimized capture والهوية الدائمة بوقت إنشاء العملية وتكامل FreePi الفعلي بحاجة إلى قبول منفصل. اكتمال وظائف المحركات الأخرى وworker leases/Job Objects ليس ضمن الادعاء بهذا الباتش، وجميع complete=false محفوظة. لا إصدار نهائي أو تثبيت تحديث ناقص.

## قيود موارد فعلية وأوامر الماوس

أضيف governed-batch: عامل native مملوك يبدأ suspended ثم يُربط بـWindows Job Object قبل التنفيذ. تُفرض الذاكرة وCPU وعدد العمليات وkill-on-close، وتُقرأ الإعدادات من Windows للتحقق منها؛ timeout يوقف العامل المملوك ويتحقق من توقفه. القياسات من OS وليست أرقامًا يقدمها الأيجنت. لا تعني هذه الوظيفة وجود queue leases أو sandbox للملفات؛ العقد في [governed-batch](../architecture/governed-batch.md).

أضيف right/middle/double click وscroll وdrag مع إيقاف عند تغيّر الهدف ومحاولة إطلاق الزر عند الخروج. لم يُنسب نجاح اختبار left-click إلى هذه الأفعال الجديدة.

دليل الباتش الأول `9c1e116`: [CI 45878056470](https://github.com/AlMagrhibi/-NorthStar-RE-/actions/runs/45878056470)، ملف desktop-control.json من artifact evidence-windows-2022، SHA-256 للأرشيف `226dc872dd30a9f78f3bc1d2de32ab12485e07345ba8d2757b69a2bbeb94e2eb`. النتيجة PASS على Windows Server 2022: أثر keyboard/click/UIA، تصوير محتوى نافذة مغطاة، عزل password، وتحريك/تغيير حجم/استعادة النوافذ. os_is_windows11=false؛ لا قبول FreePi مستنتج. يُعاد CI على الباتش اللاحق ولا يعمم هذا النجاح عليه.

## مسار Windows Graphics Capture

أضيف capture-wgc/watch-wgc مع كشف دعم الجلسة. التنفيذ يستخدم GraphicsCaptureItem لنافذة HWND محددة وD3D11 frame pool، وينقل frame إلى staging texture ثم PNG، ويحفظ timestamp. لا يُستبدل مسار PrintWindow الناجح، ولا تعني قابلية بناء WGC أنه يعمل على كل DirectX أو على جلسة Windows Server. يفحص fixture الدعم ويسجل نتيجة WGC منفصلة؛ لا تُنسب نتيجة backend إلى الآخر. لا تزال اختبارات جهاز Windows 11 وFreePi غير متاحة هنا.

أُحدّث دليل الماستر المصدر ليستخدم اكتشاف القدرات الفعلي، UIA قبل النقر الإحداثي، صور النافذة، إدخال stdin، والتحقق المستقل من أثر الفعل. هذا تعديل مصدر للمراجعة والاختبار، وليس تثبيتًا على حساب المستخدم أو تسجيلًا في FreePi.

## صورة فعلية داخل رد الأيجنت

أضيف MCP image block لصورة PNG بدل الاكتفاء بمسار محلي. تُقارن البصمة قبل الإرسال، ويُمنع تحميل ملف غير عادي أو صورة متغيرة؛ الحد inline هو 8 MiB مع توضيح السبب عند الاكتفاء بالملف المحلي. أضيفت vectors معيارية RFC4648 لاختبار النقل واختبار fixture يقارن بايتات الصورة الواصلة إلى الأيجنت ببصمة الملف. يتطلب استخدام الصورة مضيفًا وموديلًا يدعمان image content.

فشل مسار WGC في CI 45879734712 و45880046526 رغم نجاح بناء Rust. أبقي الاختبار مفعّلًا وأضيف نقل خطأ المرحلة/HRESULT من worker إلى المشرف لتحديد السبب، مع تقرير جزئي يميز أفعال الماوس/الأيجنت المنفذة عن فشل WGC. لا يُعتبر نجاح PrintWindow قبولًا لـWGC.
