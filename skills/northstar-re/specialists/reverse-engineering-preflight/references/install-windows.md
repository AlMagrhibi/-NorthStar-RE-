# Windows 11 setup (approval required)
Use official sources and confirm package IDs with `winget search` and publisher before installing. Prefer pinned verified versions. No automatic installation.
- .NET: https://dotnet.microsoft.com/download ; `winget search Microsoft.DotNet.SDK`; select SDK required by tool/project, verify `dotnet --list-sdks`.
- ILSpy: https://github.com/icsharpcode/ILSpy ; `dotnet tool install --global ilspycmd --version <verified-version>`; verify `ilspycmd --disable-updatecheck -v`. Uninstall via `dotnet tool uninstall --global ilspycmd`.
- Ghidra: https://github.com/NationalSecurityAgency/ghidra/releases ; verify published SHA-256, use `Expand-Archive` into new directory, set `$env:GHIDRA_HOME` to verified root. Match full JDK to release properties. https://adoptium.net/ ; `java -version`, `javac -version`.
- Visual Studio: https://visualstudio.microsoft.com/downloads/ ; select required workloads via Visual Studio Installer; remove individual workloads there if necessary.
- Sandbox: https://learn.microsoft.com/windows/security/application-security/application-isolation/windows-sandbox/ ; availability depends on edition, virtualization and optional features. Enable only with approval, use disconnected disposable guest for unknown samples; never assume guest isolation from launcher presence.
- Risks: downloads, disk use, PATH changes, possible restart. Record exact changes and use vendor uninstallers to revert. Do not remove other tools or existing installations.
