# Complete tool and reference directory — V9
Candidates only; not installed or verified. Confirm compatibility and license.

## 1. WinGet — `winget`
- Type: application; group: foundation
- Upstream: https://learn.microsoft.com/en-us/windows/package-manager/winget/
- Usage reference: https://learn.microsoft.com/en-us/windows/package-manager/winget/
- Discovery names: winget.exe
- Verification: App Installer; inspect source and package publisher
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 3. Git — `git`
- Type: application; group: development
- Upstream: https://git-scm.com/download/win
- Usage reference: https://git-scm.com/download/win
- Discovery names: git.exe
- Verification: Git.Git candidate; confirm live
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 4. Visual Studio Locator — `vswhere`
- Type: application; group: development
- Upstream: https://github.com/microsoft/vswhere
- Usage reference: https://github.com/microsoft/vswhere
- Discovery names: vswhere.exe
- Verification: Visual Studio Installer directory; query instances and workloads
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 5. Visual Studio IDE — `devenv`
- Type: application; group: development
- Upstream: https://visualstudio.microsoft.com/downloads/
- Usage reference: https://visualstudio.microsoft.com/downloads/
- Discovery names: devenv.exe
- Verification: Check installed instance and About; not required for Build Tools
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 6. MSBuild — `msbuild`
- Type: application; group: development
- Upstream: https://learn.microsoft.com/visualstudio/msbuild/msbuild
- Usage reference: https://learn.microsoft.com/visualstudio/msbuild/msbuild
- Discovery names: MSBuild.exe
- Verification: Developer PowerShell; build synthetic project
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 7. MSVC compiler — `msvc`
- Type: application; group: native-dev
- Upstream: https://learn.microsoft.com/cpp/build/building-on-the-command-line
- Usage reference: https://learn.microsoft.com/cpp/build/building-on-the-command-line
- Discovery names: cl.exe
- Verification: Developer PowerShell; compile trusted fixture for target arch
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 8. MSVC linker — `link`
- Type: application; group: native-dev
- Upstream: https://learn.microsoft.com/cpp/build/reference/linker-options
- Usage reference: https://learn.microsoft.com/cpp/build/reference/linker-options
- Discovery names: link.exe
- Verification: Verify with trusted fixture
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 9. DUMPBIN — `dumpbin`
- Type: application; group: native-analysis
- Upstream: https://learn.microsoft.com/cpp/build/reference/dumpbin-reference
- Usage reference: https://learn.microsoft.com/cpp/build/reference/dumpbin-reference
- Discovery names: dumpbin.exe
- Verification: Developer PowerShell; inspect trusted PE fixture
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 10. CMake — `cmake`
- Type: application; group: native-dev
- Upstream: https://cmake.org/download/
- Usage reference: https://cmake.org/download/
- Discovery names: cmake.exe
- Verification: Validate project presets
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 11. Ninja — `ninja`
- Type: application; group: native-dev
- Upstream: https://github.com/ninja-build/ninja/releases
- Usage reference: https://github.com/ninja-build/ninja/releases
- Discovery names: ninja.exe
- Verification: Optional generator
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 12. .NET SDK/runtime — `dotnet`
- Type: application; group: managed-dev
- Upstream: https://dotnet.microsoft.com/download
- Usage reference: https://dotnet.microsoft.com/download
- Discovery names: dotnet.exe
- Verification: Inspect --list-sdks and --list-runtimes, project global.json
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 13. ILSpy GUI — `ilspy`
- Type: application; group: managed-analysis
- Upstream: https://github.com/icsharpcode/ILSpy/releases
- Usage reference: https://github.com/icsharpcode/ILSpy/releases
- Discovery names: ILSpy.exe
- Verification: About/version and known managed fixture
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 14. ILSpy CLI — `ilspycmd`
- Type: application; group: managed-analysis
- Upstream: https://www.nuget.org/packages/ilspycmd
- Usage reference: https://www.nuget.org/packages/ilspycmd
- Discovery names: ilspycmd.exe
- Verification: Pin compatible release; use installed help
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 15. IL Disassembler — `ildasm`
- Type: application; group: managed-analysis
- Upstream: https://learn.microsoft.com/dotnet/framework/tools/ildasm-exe-il-disassembler
- Usage reference: https://learn.microsoft.com/dotnet/framework/tools/ildasm-exe-il-disassembler
- Discovery names: ildasm.exe
- Verification: Optional SDK tool; compare IL
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 16. Java runtime — `java`
- Type: application; group: native-analysis
- Upstream: https://adoptium.net/
- Usage reference: https://adoptium.net/
- Discovery names: java.exe
- Verification: Match Ghidra release requirement
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 17. Java compiler — `javac`
- Type: application; group: native-analysis
- Upstream: https://adoptium.net/
- Usage reference: https://adoptium.net/
- Discovery names: javac.exe
- Verification: Full JDK matching Java runtime
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 18. Ghidra — `ghidra`
- Type: application; group: native-analysis
- Upstream: https://github.com/NationalSecurityAgency/ghidra/releases
- Usage reference: https://github.com/NationalSecurityAgency/ghidra/releases
- Discovery names: ghidraRun.bat
- Verification: Read application.properties and installed docs; trusted fixture headless
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 19. Ghidra headless — `ghidra-headless`
- Type: application; group: native-analysis
- Upstream: https://github.com/NationalSecurityAgency/ghidra
- Usage reference: https://github.com/NationalSecurityAgency/ghidra
- Discovery names: analyzeHeadless.bat
- Verification: Use local help and known fixture
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 20. Detect It Easy — `die`
- Type: application; group: dll-triage
- Upstream: https://github.com/horsicq/Detect-It-Easy/releases
- Usage reference: https://github.com/horsicq/Detect-It-Easy/releases
- Discovery names: die.exe, diec.exe
- Verification: Check release/build and signature scan on synthetic fixture
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 21. Sigcheck — `sigcheck`
- Type: application; group: dll-triage
- Upstream: https://learn.microsoft.com/sysinternals/downloads/sigcheck
- Usage reference: https://learn.microsoft.com/sysinternals/downloads/sigcheck
- Discovery names: sigcheck.exe, sigcheck64.exe, sigcheck64a.exe
- Verification: Offline signature inspection; no VirusTotal upload
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 22. PE-bear — `pe-bear`
- Type: application; group: dll-triage
- Upstream: https://github.com/hasherezade/pe-bear/releases
- Usage reference: https://github.com/hasherezade/pe-bear/releases
- Discovery names: PE-bear.exe
- Verification: Optional static PE viewer, verify About and fixture
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 23. Dependencies — `dependencies`
- Type: application; group: native-analysis
- Upstream: https://github.com/lucasg/Dependencies/releases
- Usage reference: https://github.com/lucasg/Dependencies/releases
- Discovery names: DependenciesGui.exe, Dependencies.exe
- Verification: Optional import graph; static only, no profiling/loading target
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 24. WinDbg — `windbg`
- Type: application; group: debugging
- Upstream: https://learn.microsoft.com/windows-hardware/drivers/debugger/
- Usage reference: https://learn.microsoft.com/windows-hardware/drivers/debugger/
- Discovery names: WinDbgX.exe, windbg.exe
- Verification: Trusted dump or owned fixture; no unknown execution
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 25. Process Monitor — `procmon`
- Type: application; group: debugging
- Upstream: https://learn.microsoft.com/sysinternals/downloads/procmon
- Usage reference: https://learn.microsoft.com/sysinternals/downloads/procmon
- Discovery names: Procmon.exe, Procmon64.exe
- Verification: Optional trusted diagnostic; approval for driver/elevation
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 26. Windows Sandbox — `sandbox`
- Type: application; group: isolation
- Upstream: https://learn.microsoft.com/windows/security/application-security/application-isolation/windows-sandbox/
- Usage reference: https://learn.microsoft.com/windows/security/application-security/application-isolation/windows-sandbox/
- Discovery names: WindowsSandbox.exe
- Verification: Feature presence is not isolation proof; verify configuration
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 27. CTest — `ctest`
- Type: application; group: native-dev
- Upstream: https://cmake.org/cmake/help/latest/manual/ctest.1.html
- Usage reference: https://cmake.org/cmake/help/latest/manual/ctest.1.html
- Discovery names: ctest.exe
- Verification: Run tests only for trusted source
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 28. Visual Studio Test — `vstest`
- Type: application; group: managed-dev
- Upstream: https://learn.microsoft.com/visualstudio/test/vstest-console-options
- Usage reference: https://learn.microsoft.com/visualstudio/test/vstest-console-options
- Discovery names: vstest.console.exe
- Verification: Legacy or VS test workflows
- Acquisition: Follow setup runbook; validate exact package and publisher before installing.

## 29. ImHex — `imhex`
- Type: application; group: binary
- Upstream: https://github.com/WerWolv/ImHex
- Usage reference: https://docs.werwolv.net/imhex
- Discovery names: imhex.exe
- Verification: Open a synthetic binary read-only; inspect offsets, bytes, entropy and an audited local pattern.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## 30. Kaitai Struct Compiler — `kaitai`
- Type: application; group: binary
- Upstream: https://kaitai.io/
- Usage reference: https://doc.kaitai.io/user_guide
- Discovery names: kaitai-struct-compiler.bat, kaitai-struct-compiler.exe, kaitai-struct-compiler
- Verification: Compile an original minimal .ksy to C# or C++ and parse valid/truncated fixtures.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## 31. DirectXTex texconv — `texconv`
- Type: application; group: assets
- Upstream: https://github.com/microsoft/DirectXTex
- Usage reference: https://github.com/microsoft/DirectXTex/wiki/texconv
- Discovery names: texconv.exe
- Verification: Convert an owned test DDS to PNG in scratch; check dimensions and output.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## 32. DirectXTex texdiag — `texdiag`
- Type: application; group: assets
- Upstream: https://github.com/microsoft/DirectXTex
- Usage reference: https://github.com/microsoft/DirectXTex/wiki
- Discovery names: texdiag.exe
- Verification: Inspect metadata of a known DDS fixture; compare dimensions, format, mip count.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## 33. 7-Zip — `7zip`
- Type: application; group: archive
- Upstream: https://www.7-zip.org/
- Usage reference: https://www.7-zip.org/faq.html
- Discovery names: 7z.exe, 7za.exe, 7zz.exe
- Verification: List and test a synthetic archive with installed CLI help; check results without extraction.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## 34. Resource Hacker — `resource-hacker`
- Type: application; group: binary
- Upstream: https://www.angusj.com/resourcehacker/
- Usage reference: https://www.angusj.com/resourcehacker/
- Discovery names: ResourceHacker.exe
- Verification: Open an owned PE fixture and compare version/icon resources to known values.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## 35. HxD — `hxd`
- Type: application; group: binary
- Upstream: https://mh-nexus.de/en/hxd/
- Usage reference: https://mh-nexus.de/en/hxd/
- Discovery names: HxD.exe, HxD64.exe
- Verification: Open a synthetic file read-only and verify byte offsets and selected range.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## 36. JPEXS Free Flash Decompiler — `jpexs`
- Type: application; group: assets
- Upstream: https://github.com/jindrapetrik/jpexs-decompiler
- Usage reference: https://github.com/jindrapetrik/jpexs-decompiler/wiki/Commandline-arguments
- Discovery names: ffdec.exe, ffdec.bat, ffdec.jar
- Verification: Inspect tags in a trusted SWF without playback or script execution; export permitted resources to scratch.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## 37. Conquer Online Development Wiki — `conquer-wiki`
- Type: reference; group: domain-reference
- Upstream: https://conquer-online.github.io/wiki/
- Usage reference: https://conquer-online.github.io/wiki/files/formats/dat.html
- Discovery names: Not a standalone executable
- Verification: Reference only; record exact page, date and documented variant; compare to sample evidence.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## 38. CO2 Core DLL — `co2-core-dll`
- Type: source; group: community-source
- Upstream: https://github.com/conquer-online/co2_core_dll
- Usage reference: https://github.com/conquer-online/co2_core_dll
- Discovery names: Not a standalone executable
- Verification: Source reference; review license, dependencies, commit and relevant file-format code before considering a build; no protection bypass.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## 39. ConquerToolsKit — `conquer-tools-kit`
- Type: source; group: community-source
- Upstream: https://github.com/darkfoxdeveloper/ConquerToolsKit
- Usage reference: https://github.com/darkfoxdeveloper/ConquerToolsKit
- Discovery names: Not a standalone executable
- Verification: Review individual utilities, license and source; test read-only on owned fixtures; do not run bundled unknown executables.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## 40. C3Studio — `c3studio`
- Type: source; group: community-source
- Upstream: https://github.com/abdouthematrix/C3Studio
- Usage reference: https://github.com/abdouthematrix/C3Studio
- Discovery names: C3Studio.exe
- Verification: Review project requirements, license and source; verify display of an owned C3 fixture in isolated environment.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## 41. Tiled2Dmap — `tiled2dmap`
- Type: source; group: community-source
- Upstream: https://github.com/SantasCode/Tiled2Dmap
- Usage reference: https://github.com/SantasCode/Tiled2Dmap
- Discovery names: Tiled2Dmap.exe
- Verification: Review project and dependency paths (including 7z library); verify map dimensions on an authorized fixture.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## 42. Tiled Map Editor — `tiled`
- Type: application; group: assets
- Upstream: https://www.mapeditor.org/
- Usage reference: https://doc.mapeditor.org/
- Discovery names: tiled.exe
- Verification: Open a synthetic TMX map; validate tile dimensions and layer count.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## 43. OpenSSL — `openssl`
- Type: application; group: crypto
- Upstream: https://openssl-library.org/
- Usage reference: https://docs.openssl.org/
- Discovery names: openssl.exe
- Verification: Verify provenance of Windows distribution; compare digest of synthetic data against known answer; use documented cryptography only with authorized keys.
- Acquisition: Review upstream documentation, license and dependencies; obtain approved official release or audited source; do not assume a WinGet ID.

## 44. Wireshark — `wireshark`
- Type: application; group: network
- Upstream: https://www.wireshark.org/download.html
- Usage reference: https://www.wireshark.org/docs/wsug_html_chunked/
- Discovery names: Wireshark.exe
- Verification: Open sanitized offline synthetic capture; no live capture by default.
- Acquisition: Confirm exact source, publisher, compatibility and scoped approval; no automatic installation.

## 45. TShark — `tshark`
- Type: application; group: network
- Upstream: https://www.wireshark.org/download.html
- Usage reference: https://www.wireshark.org/docs/man-pages/tshark.html
- Discovery names: tshark.exe
- Verification: Read sanitized pcapng fixture; verify frame count.
- Acquisition: Confirm exact source, publisher, compatibility and scoped approval; no automatic installation.

## 46. Windows Packet Monitor — `pktmon`
- Type: application; group: network
- Upstream: https://learn.microsoft.com/en-us/windows-server/networking/technologies/pktmon/pktmon
- Usage reference: https://learn.microsoft.com/en-us/windows-server/networking/technologies/pktmon/pktmon
- Discovery names: pktmon.exe
- Verification: Built-in candidate; help only; capture needs scoped approval.
- Acquisition: Confirm exact source, publisher, compatibility and scoped approval; no automatic installation.

## 47. SQLite CLI — `sqlite`
- Type: application; group: database
- Upstream: https://www.sqlite.org/download.html
- Usage reference: https://www.sqlite.org/cli.html
- Discovery names: sqlite3.exe
- Verification: In-memory SELECT 1 fixture.
- Acquisition: Confirm exact source, publisher, compatibility and scoped approval; no automatic installation.

## 48. MySQL client — `mysql`
- Type: application; group: database
- Upstream: https://dev.mysql.com/downloads/mysql/
- Usage reference: https://dev.mysql.com/doc/refman/8.4/en/windows-installation.html
- Discovery names: mysql.exe
- Verification: Check variant and connect only to disposable DB.
- Acquisition: Confirm exact source, publisher, compatibility and scoped approval; no automatic installation.

## 49. MySQL Shell — `mysqlsh`
- Type: application; group: database
- Upstream: https://dev.mysql.com/downloads/shell/
- Usage reference: https://dev.mysql.com/doc/mysql-shell/en/
- Discovery names: mysqlsh.exe
- Verification: Verify version and synthetic SELECT 1.
- Acquisition: Confirm exact source, publisher, compatibility and scoped approval; no automatic installation.

## 50. MySQL Workbench — `mysql-workbench`
- Type: application; group: database
- Upstream: https://dev.mysql.com/downloads/workbench/
- Usage reference: https://dev.mysql.com/doc/workbench/en/
- Discovery names: MySQLWorkbench.exe
- Verification: Confirm About and disposable DB connection.
- Acquisition: Confirm exact source, publisher, compatibility and scoped approval; no automatic installation.

## 51. SQL Server sqlcmd — `sqlcmd`
- Type: application; group: database
- Upstream: https://learn.microsoft.com/en-us/sql/tools/sqlcmd/sqlcmd-download-install
- Usage reference: https://learn.microsoft.com/en-us/sql/tools/sqlcmd/sqlcmd-download-install
- Discovery names: sqlcmd.exe
- Verification: Identify variant, verify on disposable instance.
- Acquisition: Confirm exact source, publisher, compatibility and scoped approval; no automatic installation.

## 52. SQL Server Management Studio — `ssms`
- Type: application; group: database
- Upstream: https://learn.microsoft.com/en-us/ssms/install/install
- Usage reference: https://learn.microsoft.com/en-us/ssms/
- Discovery names: Ssms.exe
- Verification: Confirm About and disposable DB connection.
- Acquisition: Confirm exact source, publisher, compatibility and scoped approval; no automatic installation.

## 53. PostgreSQL psql — `psql`
- Type: application; group: database
- Upstream: https://www.postgresql.org/download/windows/
- Usage reference: https://www.postgresql.org/docs/current/app-psql.html
- Discovery names: psql.exe
- Verification: Connect only to disposable test DB.
- Acquisition: Confirm exact source, publisher, compatibility and scoped approval; no automatic installation.

## 54. Windows SDK Winsock — `winsock`
- Type: reference; group: network
- Upstream: https://learn.microsoft.com/en-us/windows/win32/winsock/getting-started-with-winsock
- Usage reference: https://learn.microsoft.com/en-us/windows/win32/winsock/getting-started-with-winsock
- Discovery names: Not a standalone executable
- Verification: Reference; verify SDK headers and compile owned loopback fixture.
- Acquisition: Documentation only; verify dependent SDK/runtime separately.

## 55. ASP.NET Core Identity — `aspnet-identity`
- Type: reference; group: authentication
- Upstream: https://learn.microsoft.com/en-us/aspnet/core/security/authentication/identity
- Usage reference: https://learn.microsoft.com/en-us/aspnet/core/security/authentication/identity
- Discovery names: Not a standalone executable
- Verification: Reference; verify matching framework and synthetic account tests.
- Acquisition: Documentation only; verify dependent SDK/runtime separately.

## 56. .NET SslStream — `sslstream`
- Type: reference; group: crypto
- Upstream: https://learn.microsoft.com/en-us/dotnet/api/system.net.security.sslstream
- Usage reference: https://learn.microsoft.com/en-us/dotnet/api/system.net.security.sslstream
- Discovery names: Not a standalone executable
- Verification: Reference; verify local trusted TLS fixture.
- Acquisition: Documentation only; verify dependent SDK/runtime separately.

## 57. Windows Cryptography API: Next Generation — `windows-cng`
- Type: reference; group: crypto
- Upstream: https://learn.microsoft.com/en-us/windows/win32/seccng/cng-portal
- Usage reference: https://learn.microsoft.com/en-us/windows/win32/seccng/cng-portal
- Discovery names: Not a standalone executable
- Verification: Reference; verify SDK and known-answer synthetic fixture.
- Acquisition: Documentation only; verify dependent SDK/runtime separately.

## 58. OWASP Authentication Cheat Sheet — `owasp-auth`
- Type: reference; group: authentication
- Upstream: https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html
- Usage reference: https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html
- Discovery names: Not a standalone executable
- Verification: Reference; review controls against owned service.
- Acquisition: Documentation only; verify dependent SDK/runtime separately.

## 59. x64dbg — `x64dbg`
- Type: application; group: engineering-v7
- Upstream: https://x64dbg.com/
- Usage reference: https://help.x64dbg.com/en/latest/
- Discovery names: x64dbg.exe, x32dbg.exe
- Verification: Check signed or release-hashed distribution, debug a locally built arithmetic fixture in an isolated guest.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 60. IDA Free — `ida-free`
- Type: application; group: engineering-v7
- Upstream: https://hex-rays.com/ida-free
- Usage reference: https://docs.hex-rays.com/
- Discovery names: ida.exe, ida64.exe, idafree.exe
- Verification: Confirm edition, licensing and offline restrictions; inspect owned arithmetic fixture without uploading.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 61. Binary Ninja — `binary-ninja`
- Type: application; group: engineering-v7
- Upstream: https://binary.ninja/
- Usage reference: https://docs.binary.ninja/
- Discovery names: binaryninja.exe
- Verification: Confirm edition and local-only processing; open locally built fixture and compare symbols.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 62. Rizin — `rizin`
- Type: application; group: engineering-v7
- Upstream: https://rizin.re/
- Usage reference: https://book.rizin.re/
- Discovery names: rizin.exe, rz-bin.exe
- Verification: Check version and identify architecture of known PE fixture.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 63. Cutter — `cutter`
- Type: application; group: engineering-v7
- Upstream: https://cutter.re/
- Usage reference: https://cutter.re/docs/
- Discovery names: cutter.exe
- Verification: Open known PE fixture read-only; verify architecture and sections.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 64. Diaphora — `diaphora`
- Type: source; group: engineering-v7
- Upstream: https://github.com/joxeankoret/diaphora
- Usage reference: https://github.com/joxeankoret/diaphora
- Discovery names: Not a standalone executable
- Verification: Check IDA edition and native tooling compatibility; compare two locally built versions.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 65. dnSpyEx — `dnspyex`
- Type: application; group: engineering-v7
- Upstream: https://github.com/dnSpyEx/dnSpy
- Usage reference: https://github.com/dnSpyEx/dnSpy/wiki
- Discovery names: dnSpy.exe
- Verification: Inspect a locally built managed assembly; debug only the owned fixture.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 66. PEStudio — `pestudio`
- Type: application; group: engineering-v7
- Upstream: https://www.winitor.com/
- Usage reference: https://www.winitor.com/
- Discovery names: pestudio.exe
- Verification: Open known PE fixture offline; compare imports and sections; verify edition.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 67. YARA — `yara`
- Type: application; group: engineering-v7
- Upstream: https://github.com/VirusTotal/yara
- Usage reference: https://yara.readthedocs.io/
- Discovery names: yara.exe, yarac.exe, yara64.exe
- Verification: Match a benign synthetic marker against one positive and one negative fixture.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 68. API Monitor — `api-monitor`
- Type: application; group: engineering-v7
- Upstream: https://www.rohitab.com/apimonitor
- Usage reference: https://www.rohitab.com/apimonitor
- Discovery names: apimonitor-x86.exe, apimonitor-x64.exe
- Verification: Confirm current Windows compatibility; observe only owned synthetic test process.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 69. Process Explorer — `process-explorer`
- Type: application; group: engineering-v7
- Upstream: https://learn.microsoft.com/sysinternals/downloads/process-explorer
- Usage reference: https://learn.microsoft.com/sysinternals/downloads/process-explorer
- Discovery names: procexp.exe, procexp64.exe, procexp64a.exe
- Verification: Observe own fixture process and expected modules.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 70. WinObj — `winobj`
- Type: application; group: engineering-v7
- Upstream: https://learn.microsoft.com/sysinternals/downloads/winobj
- Usage reference: https://learn.microsoft.com/sysinternals/downloads/winobj
- Discovery names: Winobj.exe, Winobj64.exe, Winobj64a.exe
- Verification: Read-only inspect object namespace in disposable lab.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 71. RenderDoc — `renderdoc`
- Type: application; group: engineering-v7
- Upstream: https://renderdoc.org/
- Usage reference: https://renderdoc.org/docs/
- Discovery names: qrenderdoc.exe, renderdoccmd.exe
- Verification: Capture a self-built triangle demo with no external service.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 72. PIX on Windows — `pix`
- Type: application; group: engineering-v7
- Upstream: https://devblogs.microsoft.com/pix/
- Usage reference: https://learn.microsoft.com/windows/pix
- Discovery names: WinPixGpuCapturer.exe, PIX.exe
- Verification: Confirm compatible GPU/API; capture owned Direct3D demo.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 73. Windows Performance Analyzer — `wpa`
- Type: application; group: engineering-v7
- Upstream: https://learn.microsoft.com/windows-hardware/test/wpt/
- Usage reference: https://learn.microsoft.com/windows-hardware/test/wpt/windows-performance-analyzer
- Discovery names: wpa.exe
- Verification: Open an ETL trace generated by a local benchmark; validate time units.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 74. Windows Performance Recorder — `wpr`
- Type: application; group: engineering-v7
- Upstream: https://learn.microsoft.com/windows-hardware/test/wpt/
- Usage reference: https://learn.microsoft.com/windows-hardware/test/wpt/windows-performance-recorder
- Discovery names: wpr.exe, wprui.exe
- Verification: Record bounded local test workload; stop capture and review data sensitivity.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 75. AddressSanitizer — `asan`
- Type: reference; group: engineering-v7
- Upstream: https://learn.microsoft.com/cpp/sanitizers/asan
- Usage reference: https://learn.microsoft.com/cpp/sanitizers/asan
- Discovery names: Not a standalone executable
- Verification: Compile a clean owned fixture with sanitizer enabled; check compiler and architecture support.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 76. vcpkg — `vcpkg`
- Type: application; group: engineering-v7
- Upstream: https://github.com/microsoft/vcpkg
- Usage reference: https://learn.microsoft.com/vcpkg/
- Discovery names: vcpkg.exe
- Verification: Check baseline and manifest; restore a reviewed dependency in disposable project.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 77. NuGet — `nuget`
- Type: application; group: engineering-v7
- Upstream: https://www.nuget.org/
- Usage reference: https://learn.microsoft.com/nuget/
- Discovery names: nuget.exe
- Verification: Check nuget.exe or dotnet nuget; restore reviewed locked dependency in fixture.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 78. GoogleTest — `googletest`
- Type: source; group: engineering-v7
- Upstream: https://github.com/google/googletest
- Usage reference: https://google.github.io/googletest/
- Discovery names: Not a standalone executable
- Verification: Build and run positive/negative assertions in owned test project.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 79. xUnit.net — `xunit`
- Type: source; group: engineering-v7
- Upstream: https://github.com/xunit/xunit
- Usage reference: https://xunit.net/docs/getting-started/v3/getting-started
- Discovery names: Not a standalone executable
- Verification: Restore reviewed package, run dotnet test on a minimal test project.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 80. LLVM / Clang — `llvm-clang`
- Type: application; group: engineering-v7
- Upstream: https://llvm.org/
- Usage reference: https://clang.llvm.org/docs/
- Discovery names: clang.exe, clang-cl.exe, llvm-readobj.exe
- Verification: Verify toolchain version and target; compile a small owned source fixture.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 81. Doxygen — `doxygen`
- Type: application; group: engineering-v7
- Upstream: https://www.doxygen.nl/
- Usage reference: https://www.doxygen.nl/manual/
- Discovery names: doxygen.exe
- Verification: Generate local documentation from tiny owned project.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 82. Microsoft PE/COFF format — `pe-spec`
- Type: reference; group: engineering-v7
- Upstream: https://learn.microsoft.com/windows/win32/debug/pe-format
- Usage reference: https://learn.microsoft.com/windows/win32/debug/pe-format
- Discovery names: Not a standalone executable
- Verification: Cross-check headers and field sizes against known PE fixture.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 83. .NET diagnostics — `dotnet-diagnostics`
- Type: reference; group: engineering-v7
- Upstream: https://learn.microsoft.com/dotnet/core/diagnostics/
- Usage reference: https://learn.microsoft.com/dotnet/core/diagnostics/
- Discovery names: Not a standalone executable
- Verification: Collect counters only from owned local test process; avoid secrets and dumps by default.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 84. JSON Schema — `json-schema`
- Type: reference; group: engineering-v7
- Upstream: https://json-schema.org/
- Usage reference: https://json-schema.org/learn/getting-started-step-by-step
- Discovery names: Not a standalone executable
- Verification: Validate sample record against schema; reject missing evidence fields.
- Acquisition: Review license, publisher, platform, release integrity and dependencies; use approved acquisition only. Reference/source entries are not standalone installed apps.

## 85. Mermaid — `mermaid`
- Type: reference; group: engineering-governance
- Upstream: https://mermaid.js.org/
- Usage reference: https://mermaid.js.org/intro/
- Discovery names: Not a standalone executable
- Verification: Confirm upstream provenance, local version and license; test on a synthetic fixture or read reference; no readiness inferred.
- Acquisition: Use official documentation and approved install path; review exact version, publisher and hashes.

## 86. PlantUML — `plantuml`
- Type: reference; group: engineering-governance
- Upstream: https://plantuml.com/
- Usage reference: https://plantuml.com/starting
- Discovery names: Not a standalone executable
- Verification: Confirm upstream provenance, local version and license; test on a synthetic fixture or read reference; no readiness inferred.
- Acquisition: Use official documentation and approved install path; review exact version, publisher and hashes.

## 87. Graphviz — `graphviz`
- Type: application; group: engineering-governance
- Upstream: https://graphviz.org/
- Usage reference: https://graphviz.org/documentation/
- Discovery names: dot.exe
- Verification: Confirm upstream provenance, local version and license; test on a synthetic fixture or read reference; no readiness inferred.
- Acquisition: Use official documentation and approved install path; review exact version, publisher and hashes.

## 88. DocFX — `docfx`
- Type: application; group: engineering-governance
- Upstream: https://dotnet.github.io/docfx/
- Usage reference: https://dotnet.github.io/docfx/
- Discovery names: docfx.exe
- Verification: Confirm upstream provenance, local version and license; test on a synthetic fixture or read reference; no readiness inferred.
- Acquisition: Use official documentation and approved install path; review exact version, publisher and hashes.

## 89. MkDocs — `mkdocs`
- Type: application; group: engineering-governance
- Upstream: https://www.mkdocs.org/
- Usage reference: https://www.mkdocs.org/getting-started/
- Discovery names: mkdocs.exe
- Verification: Confirm upstream provenance, local version and license; test on a synthetic fixture or read reference; no readiness inferred.
- Acquisition: Use official documentation and approved install path; review exact version, publisher and hashes.

## 90. markdownlint-cli — `markdownlint`
- Type: application; group: engineering-governance
- Upstream: https://github.com/igorshubovych/markdownlint-cli
- Usage reference: https://github.com/igorshubovych/markdownlint-cli
- Discovery names: markdownlint.cmd
- Verification: Confirm upstream provenance, local version and license; test on a synthetic fixture or read reference; no readiness inferred.
- Acquisition: Use official documentation and approved install path; review exact version, publisher and hashes.

## 91. GitHub Actions — `github-actions`
- Type: reference; group: engineering-governance
- Upstream: https://docs.github.com/actions
- Usage reference: https://docs.github.com/actions
- Discovery names: Not a standalone executable
- Verification: Confirm upstream provenance, local version and license; test on a synthetic fixture or read reference; no readiness inferred.
- Acquisition: Use official documentation and approved install path; review exact version, publisher and hashes.

## 92. Cppcheck — `cppcheck`
- Type: application; group: engineering-governance
- Upstream: https://cppcheck.sourceforge.io/
- Usage reference: https://cppcheck.sourceforge.io/manual.html
- Discovery names: cppcheck.exe
- Verification: Confirm upstream provenance, local version and license; test on a synthetic fixture or read reference; no readiness inferred.
- Acquisition: Use official documentation and approved install path; review exact version, publisher and hashes.

## 93. Clang-Tidy — `clang-tidy`
- Type: application; group: engineering-governance
- Upstream: https://clang.llvm.org/extra/clang-tidy/
- Usage reference: https://clang.llvm.org/extra/clang-tidy/
- Discovery names: clang-tidy.exe
- Verification: Confirm upstream provenance, local version and license; test on a synthetic fixture or read reference; no readiness inferred.
- Acquisition: Use official documentation and approved install path; review exact version, publisher and hashes.

## 94. SonarQube — `sonarqube`
- Type: reference; group: engineering-governance
- Upstream: https://www.sonarsource.com/products/sonarqube/
- Usage reference: https://docs.sonarsource.com/
- Discovery names: Not a standalone executable
- Verification: Confirm upstream provenance, local version and license; test on a synthetic fixture or read reference; no readiness inferred.
- Acquisition: Use official documentation and approved install path; review exact version, publisher and hashes.

## 95. Coverlet — `coverlet`
- Type: reference; group: engineering-governance
- Upstream: https://github.com/coverlet-coverage/coverlet
- Usage reference: https://github.com/coverlet-coverage/coverlet
- Discovery names: Not a standalone executable
- Verification: Confirm upstream provenance, local version and license; test on a synthetic fixture or read reference; no readiness inferred.
- Acquisition: Use official documentation and approved install path; review exact version, publisher and hashes.

## 96. BenchmarkDotNet — `benchmarkdotnet`
- Type: reference; group: engineering-governance
- Upstream: https://benchmarkdotnet.org/
- Usage reference: https://benchmarkdotnet.org/articles/overview.html
- Discovery names: Not a standalone executable
- Verification: Confirm upstream provenance, local version and license; test on a synthetic fixture or read reference; no readiness inferred.
- Acquisition: Use official documentation and approved install path; review exact version, publisher and hashes.

## 97. Google Benchmark — `google-benchmark`
- Type: source; group: engineering-governance
- Upstream: https://github.com/google/benchmark
- Usage reference: https://github.com/google/benchmark
- Discovery names: Not a standalone executable
- Verification: Confirm upstream provenance, local version and license; test on a synthetic fixture or read reference; no readiness inferred.
- Acquisition: Use official documentation and approved install path; review exact version, publisher and hashes.

## 98. Gitleaks — `gitleaks`
- Type: application; group: engineering-governance
- Upstream: https://github.com/gitleaks/gitleaks
- Usage reference: https://github.com/gitleaks/gitleaks
- Discovery names: gitleaks.exe
- Verification: Confirm upstream provenance, local version and license; test on a synthetic fixture or read reference; no readiness inferred.
- Acquisition: Use official documentation and approved install path; review exact version, publisher and hashes.

## 99. Syft — `syft`
- Type: application; group: engineering-governance
- Upstream: https://github.com/anchore/syft
- Usage reference: https://github.com/anchore/syft
- Discovery names: syft.exe
- Verification: Confirm upstream provenance, local version and license; test on a synthetic fixture or read reference; no readiness inferred.
- Acquisition: Use official documentation and approved install path; review exact version, publisher and hashes.

## 100. Grype — `grype`
- Type: application; group: engineering-governance
- Upstream: https://github.com/anchore/grype
- Usage reference: https://github.com/anchore/grype
- Discovery names: grype.exe
- Verification: Confirm upstream provenance, local version and license; test on a synthetic fixture or read reference; no readiness inferred.
- Acquisition: Use official documentation and approved install path; review exact version, publisher and hashes.

## 101. Rustup — `rustup`
- Type: application; group: rust
- Upstream: https://rust-lang.org/install
- Usage reference: https://rust-lang.github.io/rustup/
- Discovery names: rustup.exe
- Verification: Review publisher and run trusted fixture; optional tools only when required.
- Acquisition: Review source, version, license and dependencies; install only after scoped approval.

## 102. Rust compiler — `rustc`
- Type: application; group: rust
- Upstream: https://rust-lang.org/
- Usage reference: https://doc.rust-lang.org/rustc/
- Discovery names: rustc.exe
- Verification: Review publisher and run trusted fixture; optional tools only when required.
- Acquisition: Review source, version, license and dependencies; install only after scoped approval.

## 103. Cargo — `cargo`
- Type: application; group: rust
- Upstream: https://doc.rust-lang.org/cargo/
- Usage reference: https://doc.rust-lang.org/cargo/
- Discovery names: cargo.exe
- Verification: Review publisher and run trusted fixture; optional tools only when required.
- Acquisition: Review source, version, license and dependencies; install only after scoped approval.

## 104. Rustfmt — `rustfmt`
- Type: application; group: rust
- Upstream: https://github.com/rust-lang/rustfmt
- Usage reference: https://github.com/rust-lang/rustfmt
- Discovery names: rustfmt.exe
- Verification: Review publisher and run trusted fixture; optional tools only when required.
- Acquisition: Review source, version, license and dependencies; install only after scoped approval.

## 105. Clippy — `clippy`
- Type: application; group: rust
- Upstream: https://github.com/rust-lang/rust-clippy
- Usage reference: https://doc.rust-lang.org/clippy/
- Discovery names: cargo-clippy.exe
- Verification: Review publisher and run trusted fixture; optional tools only when required.
- Acquisition: Review source, version, license and dependencies; install only after scoped approval.

## 106. Cargo audit — `cargo-audit`
- Type: application; group: rust
- Upstream: https://github.com/RustSec/rustsec
- Usage reference: https://github.com/RustSec/rustsec/tree/main/cargo-audit
- Discovery names: cargo-audit.exe
- Verification: Review publisher and run trusted fixture; optional tools only when required.
- Acquisition: Review source, version, license and dependencies; install only after scoped approval.

## 107. Cargo deny — `cargo-deny`
- Type: application; group: rust
- Upstream: https://github.com/EmbarkStudios/cargo-deny
- Usage reference: https://embarkstudios.github.io/cargo-deny/
- Discovery names: cargo-deny.exe
- Verification: Review publisher and run trusted fixture; optional tools only when required.
- Acquisition: Review source, version, license and dependencies; install only after scoped approval.

## 108. Criterion benchmark reference — `criterion`
- Type: reference; group: rust
- Upstream: https://github.com/bheisler/criterion.rs
- Usage reference: https://bheisler.github.io/criterion.rs/book/
- Discovery names: Not a standalone executable
- Verification: Review publisher and run trusted fixture; optional tools only when required.
- Acquisition: Review source, version, license and dependencies; install only after scoped approval.

## NorthStar native engine — `nsre`
- Source: bundled Cargo workspace; install via Install.cmd.
- Verification: Cargo tests, release build, absolute path and doctor.
