param([Parameter(Mandatory=$true)][string]$Reports,[string]$Root=(Split-Path -Parent $PSScriptRoot))
$ErrorActionPreference='Stop'
if ($env:OS -ne 'Windows_NT') { throw 'Windows-only native validation policy' }
New-Item -ItemType Directory -Path $Reports -Force|Out-Null
$manifest=Join-Path $Root 'skills/northstar-re/engine/nsre/Cargo.toml'
$checks=@(@('fmt','--all','--','--check'),@('check','--workspace','--locked'),@('test','--workspace','--locked'),@('clippy','--workspace','--all-targets','--locked','--','-D','warnings'),@('build','--release','--locked'))
$lines=@('# تقرير البناء والاختبارات',"المضيف: $([Runtime.InteropServices.RuntimeInformation]::OSDescription)","Commit: $env:GITHUB_SHA",'')
Push-Location (Split-Path -Parent $manifest)
try {
 foreach ($argsList in $checks) {
  $label=$argsList[0]; & cargo @argsList 2>&1|Tee-Object -FilePath (Join-Path $Reports "$label.log")|Out-Host
  if ($LASTEXITCODE -ne 0) { throw "Cargo $label failed" }; $lines+="- PASS: cargo $($argsList -join ' ')"
 }
 $bin='target/release/nsre.exe'
 & "./$bin" --version|Set-Content -LiteralPath (Join-Path $Reports 'engine-version.txt')
 if ($LASTEXITCODE -ne 0) { throw 'Version smoke failed' }
 & "./$bin" doctor|Set-Content -LiteralPath (Join-Path $Reports 'doctor.json')
 if ($LASTEXITCODE -ne 0) { throw 'Doctor failed' }
 if ($env:OS -eq 'Windows_NT') {
  $vswhere=Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio/Installer/vswhere.exe'
  $dumpbin=& $vswhere -latest -products '*' -find 'VC/Tools/MSVC/**/bin/Hostx64/x64/dumpbin.exe' | Select-Object -Last 1
  if (!$dumpbin) { throw 'dumpbin required to verify Windows distribution dependencies' }
  $dependencies=& $dumpbin /DEPENDENTS $bin
  if ($LASTEXITCODE -ne 0 -or ($dependencies -match '(?i)(VCRUNTIME|MSVCP)[0-9_]*\.dll')) { throw 'Unexpected external VC runtime dependency' }
  $dependencies|Set-Content -LiteralPath (Join-Path $Reports 'windows-dependencies.txt')
 }
 & (Join-Path $root 'scripts/Test-Orchestrator-Windows.ps1') -Engine (Join-Path (Get-Location).Path $bin) -Reports $Reports
 $lines+='- PASS: Windows orchestrator process smoke'; $lines+='- PASS: --version and doctor'; $lines+='- UNVERIFIED: actual Windows 11 desktop and coding-agent acceptance'
 $lines|Set-Content -LiteralPath (Join-Path $Reports 'build-report.md') -Encoding UTF8
 Copy-Item -LiteralPath (Join-Path $Reports 'build-report.md') -Destination (Join-Path $Reports 'test-report.md')
} finally {Pop-Location}
