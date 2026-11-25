[CmdletBinding()]
param([Parameter(Mandatory=$true)][string]$Output,[string]$Tag='v11.0.0-rc.1',[string]$Reports,[switch]$SourceOnly)
$ErrorActionPreference='Stop'
Set-StrictMode -Version Latest
$root=Split-Path -Parent $PSScriptRoot
if ($Tag -notmatch '^v11\.[0-9]+\.[0-9]+(?:-rc\.[0-9]+)?$') { throw 'Invalid release tag' }
New-Item -ItemType Directory -Path $Output -Force|Out-Null
$Output=[IO.Path]::GetFullPath($Output)
$workspace=Join-Path $root 'skills/northstar-re/engine/nsre'
$exe=Join-Path $workspace 'target/release/nsre.exe'
if (!$SourceOnly) {
 if ($env:OS -ne 'Windows_NT') { throw 'Windows build and execution required for binary distribution' }
 if (!$Reports -or !(Test-Path -LiteralPath (Join-Path $Reports 'build-report.md')) -or !(Test-Path -LiteralPath (Join-Path $Reports 'test-report.md')) -or !(Test-Path -LiteralPath (Join-Path $Reports 'installer.log')) -or !(Test-Path -LiteralPath (Join-Path $Reports 'sbom.json')) -or !(Test-Path -LiteralPath (Join-Path $Reports 'audit.json'))) { throw 'Successful build, installer and security reports required' }
 $version=& $exe --version; if ($LASTEXITCODE -ne 0 -or $version -ne 'nsre 0.3.0') { throw 'Wrong engine' }
 & $exe doctor|Out-Host; if ($LASTEXITCODE -ne 0) { throw 'Doctor failed' }
}
& (Join-Path $root 'scripts/Validate-Package.ps1') -Root $root -CheckHashes|Out-Host
$stage=Join-Path ([IO.Path]::GetTempPath()) ('nsre-release-'+[Guid]::NewGuid().ToString('N'))
$pkg=Join-Path $stage 'NorthStar-RE'
New-Item -ItemType Directory -Path $pkg -Force|Out-Null
try {
 foreach ($item in Get-ChildItem -LiteralPath $root -Force) {
  if ($item.Name -in @('.git','dist')) { continue }
  Copy-Item -LiteralPath $item.FullName -Destination $pkg -Recurse
 }
 foreach ($dir in @(Get-ChildItem -LiteralPath $pkg -Directory -Recurse -Force|Where-Object {$_.Name -in @('target','.northstar')}|Sort-Object {$_.FullName.Length} -Descending)) { Remove-Item -LiteralPath $dir.FullName -Recurse -Force }
 # Both editions include all source, research, locked dependency definitions and installers.
 $binDir=Join-Path $pkg 'skills/northstar-re/engine/bin'
 if (Test-Path -LiteralPath $binDir) { Remove-Item -LiteralPath $binDir -Recurse -Force }
 & (Join-Path $root 'scripts/Write-Checksums.ps1') -Root $pkg
 $sourceName='NorthStar-RE-Framework-v11-Windows11-Source.zip'
 Add-Type -AssemblyName System.IO.Compression.FileSystem
 $dest=Join-Path $Output $sourceName; if (Test-Path -LiteralPath $dest) { throw 'Refuse artifact overwrite' }
 [IO.Compression.ZipFile]::CreateFromDirectory($pkg,$dest,[IO.Compression.CompressionLevel]::Optimal,$false)
 if (!$SourceOnly) {
  New-Item -ItemType Directory -Path $binDir -Force|Out-Null; Copy-Item -LiteralPath $exe -Destination (Join-Path $binDir 'nsre.exe')
  & (Join-Path $root 'scripts/Write-Checksums.ps1') -Root $pkg
  $binary=Join-Path $Output 'NorthStar-RE-Framework-v11-Windows11-x64.zip'
  if (Test-Path -LiteralPath $binary) { throw 'Refuse artifact overwrite' }
  [IO.Compression.ZipFile]::CreateFromDirectory($pkg,$binary,[IO.Compression.CompressionLevel]::Optimal,$false)
  $check=Join-Path $stage 'verify'; [IO.Compression.ZipFile]::ExtractToDirectory($binary,$check)
  & (Join-Path $root 'scripts/Validate-Package.ps1') -Root $check -CheckHashes|Out-Host
  Copy-Item -LiteralPath $exe -Destination (Join-Path $Output 'nsre-windows-x64.exe')
  foreach ($name in @('sbom.json','build-report.md','test-report.md','audit.json','installer.log')) { Copy-Item -LiteralPath (Join-Path $Reports $name) -Destination (Join-Path $Output $name) }
 }
 $assets=@(Get-ChildItem -LiteralPath $Output -File|Sort-Object Name|ForEach-Object {@{name=$_.Name;sha256=(Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash.ToLowerInvariant();bytes=$_.Length}})
 @{schema_version=1;tag=$Tag;framework_major=11;engine_version='0.3.0';platform=if($SourceOnly){'source'}else{'windows-x64'};required_checks_passed=(!$SourceOnly);status='RELEASE_CANDIDATE';commit=$env:GITHUB_SHA;assets=$assets;limitations=@('Actual Windows 11 desktop and agent integration UNVERIFIED','MCP candidate only; no exact resume or distributed scheduler')}|ConvertTo-Json -Depth 10|Set-Content -LiteralPath (Join-Path $Output 'release-manifest.json') -Encoding UTF8
 $sums=Get-ChildItem -LiteralPath $Output -File|Where-Object Name -ne 'SHA256SUMS.txt'|Sort-Object Name|ForEach-Object {(Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash.ToLowerInvariant()+'  '+$_.Name}
 [IO.File]::WriteAllLines((Join-Path $Output 'SHA256SUMS.txt'),[string[]]$sums,[Text.UTF8Encoding]::new($false))
} finally { Remove-Item -LiteralPath $stage -Recurse -Force }
