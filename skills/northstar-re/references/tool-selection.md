# Global tool-selection contract
Catalog: `specialists/environment-discovery-setup/references/tool-catalog.json` (108 entries including reference and source projects). Read this before every specialist. Inventory all candidates at first invocation; install only needed applications, with explicit scoped approval. Reference-only entries never require installation; source-only projects require license/commit/dependency review and independent approval before build. A found candidate is not VERIFIED. For community tools compare against sample format and known fixtures; never execute an arbitrary bundled binary. If a required tool is absent, propose an official source, compatible release, validation and alternative; stop dependent work until verified. Use offline isolated guest for untrusted data, no sample transfers, no protection bypass.

| Evidence | First-choice tools | Additional options |
|---|---|---|
| PE/DLL | Detect It Easy, PE-bear, DUMPBIN | Dependencies, Sigcheck, Resource Hacker |
| Managed .NET | ILSpy / ILSpyCmd | ILDasm, .NET SDK |
| Native PE | Ghidra, DUMPBIN | WinDbg on owned dumps |
| Unknown binary | ImHex, HxD | Kaitai Struct, Rust |
| Standard archive | 7-Zip | Bounded custom parser for unsupported formats |
| DDS | DirectXTex texdiag/texconv | ImHex |
| C3 or Conquer maps | Verified C3Studio/Tiled2Dmap source | Tiled, community wiki |
| SWF | JPEXS (static only) | ImHex |
| Documented crypto with supplied key | Approved .NET/OpenSSL libraries | Known-answer fixture; no key harvesting |
| Network diagnostics (sanitized local capture) | Wireshark, TShark | Packet Monitor (approved local capture only) |
| Owned account / secure transport | MSVC/.NET, Winsock, ASP.NET Core Identity | SslStream, CNG, OpenSSL |
| Database | Engine-specific CLI: SQLite, MySQL, sqlcmd, psql | Workbench, SSMS, MySQL Shell |
| Build/test | Visual Studio, MSVC, MSBuild, .NET SDK | CMake/Ninja/CTest/VSTest |

Every specialist must record exact tool IDs, required/optional status, local path, version, provenance, smoke tests, limitations and fallback. Tool catalog URLs are upstream references, not commands to install. Keep the knowledge base project-neutral; add per-project schemas/notes in approved workspace, with sources and confidence.

Human-readable full tool directory: `../specialists/environment-discovery-setup/references/TOOL-DIRECTORY.md`.

Rust inventory: read `v9-rust-workflow.md`; use source prototype only after compilation and parity acceptance.
