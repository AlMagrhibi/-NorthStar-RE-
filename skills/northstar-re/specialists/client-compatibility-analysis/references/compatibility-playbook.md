# Flexible client analysis reference
| Domain | Evidence | Acceptance |
|---|---|---|
| DLL interface | Export names/ordinals, calling convention, architecture, documented ABI | Trusted test harness and known input/output |
| File parser | Header, length, endian, record boundaries, checksums | Valid and malformed synthetic fixtures |
| UI/rendering | Authorized screenshots and controlled scene state | Reproducible comparison with tolerance |
| Resources | Resource types, encoding, offsets, hashes | Verified extraction on owned data |
| Interop | Managed metadata, native exports, structure layout | Round-trip fixture; architecture match |
| Network | Published or explicitly authorized protocol contract | Local mock, no third-party endpoint testing |
Record observed version rather than inferring it from file names. Re-run preflight for each changed binary hash. Keep a compatibility matrix of observed builds; never claim support for untested variants. Use the project-native language and compiler, not a predetermined stack.
