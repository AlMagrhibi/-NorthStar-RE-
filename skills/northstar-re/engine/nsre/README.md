# nsre 0.3.0 — source candidate (not compiled here)

Run on an approved Rust/MSVC development host. Review dependencies and build scripts first. Generate/review Cargo.lock; then `cargo test --locked` and `cargo build --release --locked`. No binary is shipped. See `../../references/v11-native-contract.md`.

```
nsre --version
nsre scan --root C:/owned-copy --out C:/reports/fresh.jsonl --jobs 4
nsre index --root C:/owned-copy --db C:/reports/index.sqlite --jobs 4 --mode strict
nsre index --root C:/owned-copy --db C:/reports/index.sqlite --jobs 4 --mode reuse-metadata
nsre export --db C:/reports/index.sqlite --run 1
nsre diff --db C:/reports/index.sqlite --old 1 --new 3
nsre probe C:/owned-copy/example.dll
```

Use a quiescent authorized corpus and a private local output directory outside it. `scan` preserves V9 JSONL schema; `index` commits batches; `export` streams schema 2 to stdout; `diff` compares complete strict runs of the same root only. Metadata reuse is opt-in, not evidence-grade: same size/time can conceal content changes. Interrupted runs remain `running`; a new run can reuse previously committed fresh hashes, but always enumerates the tree again. No exact-position resume, automatic deletion, network service, full PE parser, DLL unpacker or code execution. Exit 0 complete, 2 partial, 1 failure. No guarantee against concurrent malicious filesystem replacement; use isolated immutable snapshots. Index contents are sensitive; never upload them by default.

V11 adds native governance and project-local SQLite records, plus read-only stdio APIs. See the V11 contract for commands and limits.
