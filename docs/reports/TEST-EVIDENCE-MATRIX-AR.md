# مصفوفة أدلة الاختبار الرباعية — NorthStar V12

**الالتزام:** انظر `TEST-EVIDENCE-MATRIX.json` (`commit_sha`) · **التاريخ (UTC):** 2026-11-27
**البيئة المرجعية المعتمدة:** جهاز المالك الفعلي — Windows 11 Pro 26H2 (Build 26300) · Dell Latitude 5320 · i5-1145G7 (4C/8T) · 15.7GB · Rust `x86_64-pc-windows-msvc` 1.91.1 · **Native (ليس WSL/VM)** · Session 0 (سياق SSH/خدمة) · FreePi PID 13472

كل دفعة هندسية من الآن تُرفع مع هذه المجموعات الأربع منفصلة، مرتبطة بـCommit SHA وSHA-256 للملف التنفيذي وOS Build وSession ID ومدة التنفيذ والنتيجة الفعلية.

## 1) Native Windows 11 Tests (محلي، هذا الجهاز)

| الأمر | النتيجة | المدة |
|---|---|---|
| `cargo test --locked` | **75 اختبارًا ناجحًا، 0 فاشل** (54 وحدة + 3 queue_cli + 2 engine_cli + 13 queue_cli_process + 3 framework_cli) | ≈6.2s |
| `cargo clippy --locked --all-targets -- -D warnings` | نظيف | ضمن البناء |
| `nsre winsessions context` | تصنيف `session-zero`، session_id=0 | <1s |
| `nsre desktop list` | `SESSION_ROUTING_REFUSED` (مرفوض عمدًا من Session 0) | <1s |

## 2) GitHub Hosted CI Tests (windows-2022 runner — ليس هذا الجهاز)

| التشغيل | الالتزام | النتيجة |
|---|---|---|
| [45915383637](https://github.com/AlMagrhibi/-NorthStar-RE-/actions/runs/45915383637) | `ff8fc5d` | success |
| تشغيل هذه الدفعة | يُضاف بعد الرفع من نفس السجل (link) | — |

CI لا يُدمج أبدًا مع النتائج المحلية؛ بيئته صورة windows-2022.

## 3) Interactive Desktop Tests

| البند | الدليل | الحكم |
|---|---|---|
| WISM-01 كشف Session 0 | `ProcessIdToSessionId=0` → `classification=session-zero` | **PASS** |
| WISM-02 اكتشاف الجلسة التفاعلية | Console session 1 (user NSKM) من `WTSGetActiveConsoleSessionId` + `WTSEnumerateSessionsW` — بلا hardcode | **PASS** |
| WISM-11 حالة القفل | `WTSINFOEXW.SessionFlags` → console=unlocked، own=unknown | **PASS (قراءة)** |
| توجيه desktop من Session 0 | رفض `SESSION_ROUTING_REFUSED` مع التوجيه للجلسة 1 | **PASS (مسار الرفض)** |
| أفعال desktop مؤثرة من جلسة تفاعلية | **UNVERIFIED** — يتطلب عاملًا مصرحًا داخل session 1 (الحزمة التالية) | UNVERIFIED |

## 4) Actual FreePi/SSH End-to-End

| البند | الدليل | الحكم |
|---|---|---|
| ربط GitHub MCP عبر FreePi | `GITHUB-MCP-ACCEPTANCE-AR.md` (57 أداة، قراءة+كتابة+تنظيف) | **PASS** |
| كشف سياق FreePi/SSH بواسطة NorthStar | `nsre winsessions context` من جلسة FreePi → session-zero + 3 SSH markers | **PASS** |
| desktop عبر FreePi/SSH من طرف إلى طرف | **UNVERIFIED** — يتطلب العامل التفاعلي المصرح به في session 1 | UNVERIFIED |

## قياسات الأداء (مرجع الجهاز)
CPU: i5-1145G7 (4C/8T) · RAM 15.7GB · NVMe. أي قياس أداء يُسجَّل على هذه المواصفات؛ زيادة عدد العمال لا تعني تحسنًا دون قياس مقارن على نفس الجهاز.
