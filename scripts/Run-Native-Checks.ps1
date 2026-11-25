param([Parameter(Mandatory=$true)][string]$Reports,[string]$Root=(Split-Path -Parent $PSScriptRoot))
$ErrorActionPreference='Stop'
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
 $bin=if ($env:OS -eq 'Windows_NT') {'target/release/nsre.exe'} else {'target/release/nsre'}
 & "./$bin" --version|Set-Content -LiteralPath (Join-Path $Reports 'engine-version.txt')
 if ($LASTEXITCODE -ne 0) { throw 'Version smoke failed' }
 & "./$bin" doctor|Set-Content -LiteralPath (Join-Path $Reports 'doctor.json')
 if ($LASTEXITCODE -ne 0) { throw 'Doctor failed' }
 $lines+='- PASS: --version and doctor'; $lines+='- UNVERIFIED: actual Windows 11 desktop and coding-agent acceptance'
 $lines|Set-Content -LiteralPath (Join-Path $Reports 'build-report.md') -Encoding UTF8
 Copy-Item -LiteralPath (Join-Path $Reports 'build-report.md') -Destination (Join-Path $Reports 'test-report.md')
} finally {Pop-Location}
