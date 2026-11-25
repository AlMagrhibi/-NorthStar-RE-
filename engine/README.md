# Engine source layout

The buildable Cargo workspace remains at `skills/northstar-re/engine/nsre`.
This preserves `include_str!` template paths, master references, and central installer compatibility.
Run Cargo with `--manifest-path skills/northstar-re/engine/nsre/Cargo.toml` from the repository root.
There is one binary crate, `nsre` version 0.3.0, within framework generation V11.
The engine is not relocated merely to match a directory sketch. Benchmarks live in this directory;
source, tests and Cargo.lock stay together in the authoritative workspace.
