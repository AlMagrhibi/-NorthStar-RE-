# Engineering capabilities

[Documentation index](../README.md) · [العربية](../ar/capabilities.md)

## Binary and reverse engineering
Native scan/index compute inventory and SHA-256. probe/analyze provide signature, entropy and bounded PE header hints; they do not fully parse PE/CLR, decrypt, unpack or recover source. Use specialist-guided Ghidra Java automation, ILSpy, trusted native inspection and fixture comparisons for deeper authorized analysis. Original executables remain unchanged.

## Data and parallel computing
walkdir enumerates files without following links. Rayon uses 1..64 configured workers and bounded batches; SQLite serializes index writes. Strict indexing freshly hashes; reuse-metadata is advisory because size and mtime can be forged. Export streams JSONL and diff requires two complete strict runs of the same corpus. See [performance](performance.md).

## Windows, Rust, C++ and C#
Specialists cover MSVC/Windows SDK discovery, ABI boundaries, Visual Studio builds, C++ servers and managed tools. Doctor lists PATH candidates and does not certify installation. Preserve existing C++/C# components. No Python fallback; Ghidra automation uses Java when necessary.

## Account/game servers, databases and networks
Instruction modules guide owned authentication/service design, synthetic protocol fixtures, DB migrations and network diagnostics. There is no bundled game server or production-service access. Use synthetic identities, local isolated services and reviewed migration backups.

## Performance engineering
Measure wall time, resource counters, accuracy and cache reuse with reproducible synthetic input. A benchmark result applies only to its recorded host, corpus, version and cache conditions. No universal throughput or million-file claim is made.
