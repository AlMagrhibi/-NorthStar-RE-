# Isolated runtime observation of authorized fixtures — reference and verification guide

Read exact installed-version docs before use. No automatic downloads or execution of untrusted code.

## x64dbg (`x64dbg`)
- Official: https://x64dbg.com/
- Usage: https://help.x64dbg.com/en/latest/
- Local check: Check signed or release-hashed distribution, debug a locally built arithmetic fixture in an isolated guest.

## dnSpyEx (`dnspyex`)
- Official: https://github.com/dnSpyEx/dnSpy
- Usage: https://github.com/dnSpyEx/dnSpy/wiki
- Local check: Inspect a locally built managed assembly; debug only the owned fixture.

## WinDbg (`windbg`)
- Official: https://learn.microsoft.com/windows-hardware/drivers/debugger/
- Usage: https://learn.microsoft.com/windows-hardware/drivers/debugger/
- Local check: Trusted dump or owned fixture; no unknown execution

## Process Monitor (`procmon`)
- Official: https://learn.microsoft.com/sysinternals/downloads/procmon
- Usage: https://learn.microsoft.com/sysinternals/downloads/procmon
- Local check: Optional trusted diagnostic; approval for driver/elevation

## Process Explorer (`process-explorer`)
- Official: https://learn.microsoft.com/sysinternals/downloads/process-explorer
- Usage: https://learn.microsoft.com/sysinternals/downloads/process-explorer
- Local check: Observe own fixture process and expected modules.

## WinObj (`winobj`)
- Official: https://learn.microsoft.com/sysinternals/downloads/winobj
- Usage: https://learn.microsoft.com/sysinternals/downloads/winobj
- Local check: Read-only inspect object namespace in disposable lab.

## API Monitor (`api-monitor`)
- Official: https://www.rohitab.com/apimonitor
- Usage: https://www.rohitab.com/apimonitor
- Local check: Confirm current Windows compatibility; observe only owned synthetic test process.

## Windows Sandbox (`sandbox`)
- Official: https://learn.microsoft.com/windows/security/application-security/application-isolation/windows-sandbox/
- Usage: https://learn.microsoft.com/windows/security/application-security/application-isolation/windows-sandbox/
- Local check: Feature presence is not isolation proof; verify configuration

## Fixture and evidence
Known test program returns expected result; no unauthorized process attachment or network.
Record expected versus observed results, exact tool version, exit status, source and output hashes.
