# Tool selection and usage

## 7-Zip (`7zip`)
- Source: https://www.7-zip.org/
- Usage reference: https://www.7-zip.org/faq.html
- Verification: List and test a synthetic archive with installed CLI help; check results without extraction.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## ImHex (`imhex`)
- Source: https://github.com/WerWolv/ImHex
- Usage reference: https://docs.werwolv.net/imhex
- Verification: Open a synthetic binary read-only; inspect offsets, bytes, entropy and an audited local pattern.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## Kaitai Struct Compiler (`kaitai`)
- Source: https://kaitai.io/
- Usage reference: https://doc.kaitai.io/user_guide
- Verification: Compile an original minimal .ksy to C# or C++ and parse valid/truncated fixtures.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## HxD (`hxd`)
- Source: https://mh-nexus.de/en/hxd/
- Usage reference: https://mh-nexus.de/en/hxd/
- Verification: Open a synthetic file read-only and verify byte offsets and selected range.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## CO2 Core DLL (`co2-core-dll`)
- Source: https://github.com/conquer-online/co2_core_dll
- Usage reference: https://github.com/conquer-online/co2_core_dll
- Verification: Source reference; review license, dependencies, commit and relevant file-format code before considering a build; no protection bypass.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## ConquerToolsKit (`conquer-tools-kit`)
- Source: https://github.com/darkfoxdeveloper/ConquerToolsKit
- Usage reference: https://github.com/darkfoxdeveloper/ConquerToolsKit
- Verification: Review individual utilities, license and source; test read-only on owned fixtures; do not run bundled unknown executables.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## C3Studio (`c3studio`)
- Source: https://github.com/abdouthematrix/C3Studio
- Usage reference: https://github.com/abdouthematrix/C3Studio
- Verification: Review project requirements, license and source; verify display of an owned C3 fixture in isolated environment.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

Use read-only originals and isolated offline analysis for untrusted files. Confirm current CLI syntax and architecture from upstream before use. Source projects are reference candidates, not pre-approved installers.
