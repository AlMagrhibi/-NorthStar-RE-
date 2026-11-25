# إعدادات المستودع

تم التحقق من بقاء المستودع Private والفرع الافتراضي main، مع الحفاظ على التاريخ والفروع والإصدارات السابقة. طُبقت ملفات CI وقوالب Issues وPull Request وDependabot وسياسات SECURITY وSUPPORT والتوثيق والهوية البصرية عبر Git.

تعديل Description وTopics وإعداد Social Preview وحماية الفرع والـRequired Status Checks لم يُطبق: الأدوات المتصلة تدعم محتوى Git وقراءة البيانات، ولا تعرض عمليات كتابة الإدارة؛ جلسة المتصفح غير مسجلة على GitHub. لم تُطلب Tokens أو تُنقل صلاحيات لأطراف أخرى ولم يُنشر GitHub Pages عام.

جهّزت `.github/repository-settings.json` كإعدادات مقترحة بحالة DESIRED_NOT_APPLIED، وصورة `assets/branding/social-preview.png` بمقاس 1280×640 لتطبيقها في جلسة مالك مسجلة. وجود الملفين لا يعني تطبيق الإعدادات.

السياق المقترح للحماية بعد اعتماد المالك: مراجعة PR وفحوص Native (windows-2022) وNative (ubuntu-24.04) وSecurity and licenses وVerified Windows distributions، مع منع force push وحذف الفرع. لا تغير الخصوصية ولا تضف متعاونين.
