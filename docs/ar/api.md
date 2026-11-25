# مرجع API المحلي وMCP

[فهرس التوثيق](../README.md) · [English](../en/api.md)

## النقل والطرق
يشغّل nsre api --project DIR اتصال JSON-RPC 2.0 بسطر JSON لكل رسالة عبر stdin/stdout، دون HTTP أو منفذ شبكة أو تحكم بعيد. الحد الأقصى للطلب 1 MiB. doctor وnsre_doctor لاكتشاف البيئة، وstatus وnsre_status لحالة المشروع، وping للاتصال. أخطاء التحليل -32700 والطلب -32600 والطريقة -32601 والتشغيل -32000. لا ترسل أسرارًا.

## MCP التجريبي
يشغّل nsre mcp --project DIR مصافحة initialize ثم notifications/initialized، وبعدها tools/list وtools/call. إصدار البروتوكول المرشح 2025-11-25، والأدوات nsre_doctor وnsre_status للقراءة فقط وبمعاملات فارغة. التوافق الفعلي مع الأيجنتات UNVERIFIED؛ نجاح اختبار مصافحة محلي لا يثبت توافق المضيف أو أحدث بروتوكول. لا يغير المحرك إعدادات الأيجنت.
