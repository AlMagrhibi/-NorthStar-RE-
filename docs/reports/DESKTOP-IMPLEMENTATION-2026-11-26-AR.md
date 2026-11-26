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
