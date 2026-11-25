# قاعدة المعرفة البحثية — NorthStar RE V10

تاريخ البحث: 2026-11-25. هذه قائمة مراجع فنية مع قرارات هندسية، لا نسخة من محتويات المواقع ولا ادعاء بتجربة كل الأدوات. راجع الإصدار والترخيص وسجل الثغرات عند التثبيت. تم التحقق من نتائج البحث على الإنترنت للمراجع ذات وسم «مراجع مراجعة»، والمراجع الإضافية نقاط انطلاق معروفة لمراجعة لاحقة. لا تعتمد على آخر إصدار دون تحقق على الجهاز المستهدف.

## مراجع مراجعة وقرارات مستخلصة

| المصدر | القرار الهندسي |
|---|---|
| [Rayon ThreadPoolBuilder](https://docs.rs/rayon/latest/rayon/struct.ThreadPoolBuilder.html) | تجمع عمال مستقل محدود العدد بدل إطلاق مهمة لكل ملف |
| [WalkDir](https://docs.rs/walkdir/latest/walkdir/struct.WalkDir.html) | تحديد عدد المقابض المفتوحة وعدم تتبع الروابط |
| [SQLite WAL](https://www.sqlite.org/wal.html) | كاتب واحد في اللحظة الواحدة؛ معاملات قصيرة؛ قاعدة محلية لا شبكة مشتركة؛ مراقبة نمو WAL |
| [rusqlite Connection](https://docs.rs/rusqlite/latest/rusqlite/struct.Connection.html) | اتصال الكاتب منفصل عن المعالجة المتوازية وbusy_timeout |
| [rusqlite Transaction](https://docs.rs/rusqlite/latest/rusqlite/struct.Transaction.html) | كتابة دفعات ذرية وتقليل overhead؛ كل دفعة ناجحة تبقى بعد التعطل |
| [Microsoft Reparse Points](https://learn.microsoft.com/en-us/windows/win32/fileio/reparse-points) | رفض الروابط وjunctions في المسح الافتراضي وعدم الوثوق بامتداد الملف |
| [Microsoft Long Paths](https://learn.microsoft.com/en-us/windows/win32/fileio/maximum-file-path-limitation) | اختبار مسارات Windows الطويلة فعليًا، بدون تعديل Registry تلقائي |
| [Microsoft FileIdFullDirectoryInformation](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-fscc/ab8e7558-899c-4be1-a7c5-3a9ae8ab76a0) | هوية الملفات ووقت التعديل وحجمها مهمة للفهرسة؛ تخزين هوية Windows القوية في مرحلة لاحقة |
| [object read API](https://docs.rs/object/latest/object/read/) | تقييم محلل PE/COFF/ELF مشترك بعد مرحلة فحص الترويسة؛ لا يعني أن المكتبة مدمجة الآن |
| [Cargo test](https://doc.rust-lang.org/cargo/commands/cargo-test.html) | اختبار وحدات وتكامل وإثبات عدد الاختبارات وتثبيت lockfile |
| [Cargo build](https://doc.rust-lang.org/cargo/commands/cargo-build.html) | build release قابل للتكرار بعد مراجعة القفل |
| [Cargo build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html) | مراجعة الاعتماديات قبل build لأن build.rs قد ينفذ كودًا |
| [Cargo bench](https://doc.rust-lang.org/cargo/commands/cargo-bench.html) | قياس متكرر؛ فصل warm cache وعمليات I/O عن microbenchmarks |
| [Cargo fuzz](https://rust-fuzz.github.io/book/cargo-fuzz.html) | اختبار parser على عينات اصطناعية تالفة؛ مراجعة دعم المنصة قبل تشغيله |

## مراجع إضافية للتوسع — يلزم التحقق من الإصدار والاستخدام قبل الاعتماد

| مجال | مصادر وأدوات مقترحة | الاستخدام |
|---|---|---|
| Rust وWindows | [Rust Book](https://doc.rust-lang.org/book/)، [MSVC targets](https://doc.rust-lang.org/rustc/platform-support/windows-msvc.html)، [windows crate](https://github.com/microsoft/windows-rs) | بناء وتوافق x86/x64/ARM64، Unicode، file identity |
| I/O | [std::fs](https://doc.rust-lang.org/std/fs/)، [std::io](https://doc.rust-lang.org/std/io/)، [memmap2](https://docs.rs/memmap2/) | القراءة المتدفقة؛ mmap فقط بعد قياس وعلى ملفات مستقرة |
| التوازي | [Crossbeam](https://docs.rs/crossbeam-channel/)، [Tokio](https://tokio.rs/) | backpressure؛ Tokio اختياري لواجهات I/O غير المتزامنة وليس شرطًا لتجزئة الملفات |
| التجزئة | [RustCrypto hashes](https://github.com/RustCrypto/hashes)، [SHA-2 NIST](https://csrc.nist.gov/pubs/fips/180-4/upd1/final) | SHA-256 للأدلة، وعدم الخلط بين التجزئة والتشفير |
| هياكل البيانات | [Serde](https://serde.rs/)، [serde_json](https://docs.rs/serde_json/)، [JSON Lines](https://jsonlines.org/) | عقود البيانات وإخراج streaming |
| CLI | [clap](https://docs.rs/clap/)، [indicatif](https://docs.rs/indicatif/)، [tracing](https://docs.rs/tracing/) | تطوير CLI مستقر وتقدم إلى stderr دون إفساد JSON stdout |
| صيغ الملفات | [Microsoft PE format](https://learn.microsoft.com/en-us/windows/win32/debug/pe-format)، [goblin](https://docs.rs/goblin/)، [Kaitai Struct](https://kaitai.io/)، [nom](https://docs.rs/nom/) | قراءة bounds والحقول، اختبارات malformed inputs |
| موارد الرسوم | [DirectXTex](https://github.com/microsoft/DirectXTex)، [DDS guide](https://learn.microsoft.com/en-us/windows/win32/direct3ddds/dx-graphics-dds) | تمييز وفحص DDS دون تعديل موارد الكلاينت |
| تحليل مرجعي | [Ghidra](https://ghidra-sre.org/)، [ILSpy](https://github.com/icsharpcode/ILSpy)، [Detect It Easy](https://github.com/horsicq/Detect-It-Easy) | فصل التحليل الدلالي عن الجرد، ومقارنة نتائج مستقلة |
| أدوات Windows | [Process Monitor](https://learn.microsoft.com/en-us/sysinternals/downloads/procmon)، [WinDbg](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/)، [WPR](https://learn.microsoft.com/en-us/windows-hardware/test/wpt/windows-performance-recorder) | تشخيص البرامج المصرح بها، قياس الأداء ومراقبة الملفات |
| أرشيفات | [7-Zip](https://www.7-zip.org/)، [libarchive](https://www.libarchive.org/) | فحص metadata مع حدود، استخراج مشروع فقط إلى staging، عدم اجتياز المسارات |
| تخزين | [SQLite Backup](https://www.sqlite.org/backup.html)، [SQLite transactions](https://www.sqlite.org/lang_transaction.html)، [SQLite query planner](https://www.sqlite.org/queryplanner.html) | نسخ متسقة، فهارس وخطط استعلام، الاحتفاظ بالنتائج |
| الاختبار | [Criterion](https://bheisler.github.io/criterion.rs/book/)، [proptest](https://docs.rs/proptest/)، [Rust Clippy](https://doc.rust-lang.org/clippy/) | قياس وتحليل جودة واختبارات properties |
| الإمداد البرمجي | [RustSec](https://rustsec.org/)، [cargo-deny](https://embarkstudios.github.io/cargo-deny/)، [SPDX](https://spdx.dev/) | مراجعة الثغرات والتراخيص وقائمة مكونات البناء |
| ABI | [Rust Nomicon FFI](https://doc.rust-lang.org/nomicon/ffi.html)، [Microsoft P/Invoke](https://learn.microsoft.com/en-us/dotnet/standard/native-interop/pinvoke) | اعتماد subprocess أولًا؛ FFI عند الحاجة ومع ملكية buffers واضحة |
| القياس | [Windows Performance Toolkit](https://learn.microsoft.com/en-us/windows-hardware/test/wpt/)، [Hyperfine](https://github.com/sharkdp/hyperfine) | زمن التنفيذ وتغيره؛ ليس معيارًا بديلًا عن صحة النتائج |
| إدارة المعرفة | [Mermaid](https://mermaid.js.org/)، [JSON Schema](https://json-schema.org/) | مخططات البنية وعقود البيانات؛ مراجعة بشرية للأدلة |

## قرارات التصميم حسب المشكلة

- المسح الكامل: تكلفة I/O مرتبطة بحجم البيانات وعدد الملفات؛ عند ثبات 11GB لا توجد سرعة مضمونة. القراءة المتوازية تتأثر بالقرص. اجعل مخرجات الملفات bounded streaming.
- الفحص التزايدي: استخدم metadata كإشارة فقط، لا كإثبات. احفظ سبب الاسترجاع وسجل run ID. عند توثيق SHA-256 مؤكد، أعد الحساب من المصدر المصرح به. لا تعتمد على NTFS timestamps وحدها لإثبات هوية المحتوى.
- SQLite: إدارة schema ونسخ احتياطية متسقة وretention؛ منع تقارير الحذف عند فشل الجرد. لا تحذف الملف من الكلاينت لمجرد أنه غير موجود في جرد جديد.
- PE: ترويسة صحيحة لا تعني سلامة الملف أو نجاح الـDecompiler. اجعل parser محدود القراءة والذاكرة؛ لا تنفذ أو تحمّل DLL أثناء التصنيف. التشفير والتغليف لا يستنتجان من entropy وحدها.
- Scale-out: البداية Single-host؛ بعد قياس الضغط اعزل العمال في عمليات محلية مع حدود واضحة. توزيع العمل على أجهزة أخرى يحتاج مصادقة وتفويضًا صريحًا وحماية البيانات. لا ترفع الكلاينت أو ملفات الذاكرة أو السورس الخارجي افتراضيًا.
- قابلية التوسع للمهارات: مسار CLI ثابت، schema version، routing موثق، adapters موثوقة، test fixtures، ومخرجات Markdown/JSON. الماستر منسقة وليست process scheduler فعليًا.
