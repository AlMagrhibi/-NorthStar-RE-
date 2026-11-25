[CmdletBinding()]
param(
 [ValidateSet('Install','Update','Doctor','Build-Engine','Backup','Uninstall','Rollback')][string]$Action='Install',
 [string]$SkillRoot, [string]$BackupPath,
 [ValidateSet('FreePi','Codex','Pi','Claude','Other')][string]$Agent='Other',
 [switch]$AllowDependencyDownload, [switch]$NonInteractive
)
$ErrorActionPreference='Stop'
Set-StrictMode -Version Latest
if ($env:OS -ne 'Windows_NT') { throw 'Windows is required for installation. No host changes made.' }
if ($env:PROCESSOR_ARCHITECTURE -ne 'AMD64' -and $env:PROCESSOR_ARCHITEW6432 -ne 'AMD64') { throw 'Windows x64 host required' }
$Package=Split-Path -Parent $PSScriptRoot
$Master=Join-Path $Package 'skills\northstar-re'
$HomeRoot=Join-Path $env:LOCALAPPDATA 'NorthStarRE'
$Reg=Join-Path $HomeRoot 'registration.json'
$Metadata=Get-Content -LiteralPath (Join-Path $Package 'framework.json') -Raw -Encoding UTF8|ConvertFrom-Json
if ($Metadata.product -ne 'NorthStar RE Framework' -or $Metadata.engine_version -ne '0.3.0' -or $Metadata.version -notmatch '^11\.[0-9]+\.[0-9]+-rc\.[0-9]+$') { throw 'Incompatible framework metadata' }
function Assert-Normal([string]$Path) {
 $p=[IO.Path]::GetFullPath($Path)
 while ($p) {
  if (Test-Path -LiteralPath $p) {
   if ((Get-Item -LiteralPath $p -Force).Attributes -band [IO.FileAttributes]::ReparsePoint) { throw "Redirected path refused: $p" }
  }
  $parent=[IO.Path]::GetDirectoryName($p); if (!$parent -or $parent -eq $p) { break }; $p=$parent
 }
}
function Assert-Tree([string]$Path) {
 Assert-Normal $Path
 if (!(Test-Path -LiteralPath $Path)) { return }
 $todo=New-Object 'System.Collections.Generic.Stack[string]'; $todo.Push($Path)
 while ($todo.Count) {
  foreach ($item in Get-ChildItem -LiteralPath ($todo.Pop()) -Force) {
   if ($item.Attributes -band [IO.FileAttributes]::ReparsePoint) { throw "Redirected item refused: $($item.FullName)" }
   if ($item.PSIsContainer) { $todo.Push($item.FullName) }
  }
 }
}
function Write-Log([string]$Event,[object]$Details) {
 $record=@{time=[DateTime]::UtcNow.ToString('o');action=$Action;event=$Event;details=$Details}
 $record|ConvertTo-Json -Compress -Depth 8|Add-Content -LiteralPath (Join-Path $HomeRoot 'installation.jsonl') -Encoding UTF8
}
function Confirm-Action([string]$Message) {
 if (!$NonInteractive -and (Read-Host "$Message [type YES]") -cne 'YES') { throw 'Cancelled' }
}
function Run-Native([string]$Exe,[string[]]$Arguments) {
 & $Exe @Arguments | Out-Host
 if ($LASTEXITCODE -ne 0) { throw "Native command failed, exit $LASTEXITCODE" }
}
function Read-Registration {
 if (!(Test-Path -LiteralPath $Reg)) { return $null }
 Assert-Normal $Reg
 $r=Get-Content -LiteralPath $Reg -Raw -Encoding UTF8|ConvertFrom-Json
 $release=[IO.Path]::GetFullPath([string]$r.release)
 $allowed=[IO.Path]::GetFullPath((Join-Path $HomeRoot 'releases'))+[IO.Path]::DirectorySeparatorChar
 if (!$release.StartsWith($allowed,[StringComparison]::OrdinalIgnoreCase)) { throw 'Invalid registered release location' }
 if ((Split-Path -Leaf ([string]$r.skill)) -ne 'northstar-re') { throw 'Invalid registered skill location' }
 Assert-Tree ([string]$r.skill); Assert-Tree $release
 return $r
}
function Assert-Owned([string]$Path) {
 Assert-Tree $Path
 $marker=Join-Path $Path '.northstar-framework-owner'
 if (!(Test-Path -LiteralPath $marker) -or (Get-Content -LiteralPath $marker -Raw -Encoding UTF8).Trim() -ne 'NorthStarRE V11') { throw 'Ownership marker missing or invalid; existing skill preserved' }
}
function Test-Engine($Registration) {
 Assert-Owned ([string]$Registration.skill)
 $exe=Join-Path $Registration.release 'engine\bin\nsre.exe'; Assert-Normal $exe
 if ((Get-FileHash -LiteralPath $exe -Algorithm SHA256).Hash -ne $Registration.engine_sha256) { throw 'Installed engine hash mismatch' }
 Run-Native $exe @('--version'); Run-Native $exe @('doctor')
 if (!(Test-Path -LiteralPath (Join-Path $Registration.skill 'SKILL.md'))) { throw 'Master missing' }
 $guides=@(Get-ChildItem -LiteralPath (Join-Path $Registration.skill 'specialists') -Filter GUIDE.md -Recurse)
 if ($guides.Count -ne 52) { throw 'Installed specialist registry mismatch' }
}
function Save-Backup($Registration) {
 Assert-Owned ([string]$Registration.skill)
 $path=Join-Path $HomeRoot ('backups\'+[Guid]::NewGuid().ToString('N'))
 New-Item -ItemType Directory -Path $path -Force|Out-Null
 Copy-Item -LiteralPath $Registration.skill -Destination (Join-Path $path 'skill') -Recurse
 Copy-Item -LiteralPath $Reg -Destination (Join-Path $path 'registration.json')
 Write-Log 'backup' @{path=$path}; return $path
}
function Build-Engine([string]$Work) {
 $src=Join-Path $Work 'engine\nsre'
 $cargo=Get-Command rustup.exe -ErrorAction SilentlyContinue
 if (!$cargo) { throw 'Rust MSVC/Cargo and Visual Studio C++ Build Tools required. Install explicitly; no fallback.' }
 $toolchains=& $cargo.Source toolchain list
 if ($LASTEXITCODE -ne 0 -or !($toolchains -match '^1\.91\.1-x86_64-pc-windows-msvc(?:\s|$)')) { throw 'Pinned Rust 1.91.1 x64 MSVC toolchain missing; install explicitly before retrying' }
 $oldTarget=$env:CARGO_TARGET_DIR
 Push-Location $src
 try {
  $env:CARGO_TARGET_DIR=Join-Path $Work 'target'
  if (!(Test-Path -LiteralPath 'Cargo.lock')) { throw 'Reviewed Cargo.lock required' }
  $opts=@('--locked'); if (!$AllowDependencyDownload) { $opts+='--offline' }
  Run-Native $cargo.Source (@('run','1.91.1-x86_64-pc-windows-msvc','cargo','test','--workspace')+$opts)
  Run-Native $cargo.Source (@('run','1.91.1-x86_64-pc-windows-msvc','cargo','build','--release')+$opts)
  $exe=Join-Path $env:CARGO_TARGET_DIR 'release\nsre.exe'
  Run-Native $exe @('--version'); Run-Native $exe @('doctor')
  New-Item -ItemType Directory -Path (Join-Path $Work 'engine\bin') -Force|Out-Null
  Copy-Item -LiteralPath $exe -Destination (Join-Path $Work 'engine\bin\nsre.exe')
 } finally { Pop-Location; $env:CARGO_TARGET_DIR=$oldTarget }
}
Assert-Normal $HomeRoot
New-Item -ItemType Directory -Path $HomeRoot -Force|Out-Null
Assert-Normal (Join-Path $HomeRoot 'install.lock'); Assert-Normal (Join-Path $HomeRoot 'installation.jsonl')
$lock=[IO.File]::Open((Join-Path $HomeRoot 'install.lock'),[IO.FileMode]::OpenOrCreate,[IO.FileAccess]::ReadWrite,[IO.FileShare]::None)
try {
 Write-Log 'start' @{}
 $old=Read-Registration
 if ($Action -eq 'Doctor') { if (!$old) { throw 'Not installed' }; Test-Engine $old; Write-Log 'verified' @{}; return }
 if ($Action -in @('Backup','Uninstall')) {
  if (!$old) { throw 'Not installed' }; Assert-Owned $old.skill
  if ($Action -eq 'Uninstall') { Confirm-Action 'Unregister NorthStar RE? Central releases and all project data remain.' }
  $backup=Save-Backup $old
  if ($Action -eq 'Uninstall') {
   $unregistered=Join-Path $backup 'unregistered'
   Move-Item -LiteralPath $old.skill -Destination $unregistered
   try { Move-Item -LiteralPath $Reg -Destination (Join-Path $backup 'unregistered.json') }
   catch { Move-Item -LiteralPath $unregistered -Destination $old.skill; throw }
  }
  Write-Host "Backup: $backup"; Write-Log 'complete' @{backup=$backup}; return
 }
 if ($Action -eq 'Rollback') {
  if (!$BackupPath) { throw 'Explicit -BackupPath required' }; Assert-Tree $BackupPath
  $allowed=[IO.Path]::GetFullPath((Join-Path $HomeRoot 'backups'))+[IO.Path]::DirectorySeparatorChar
  if (![IO.Path]::GetFullPath($BackupPath).StartsWith($allowed,[StringComparison]::OrdinalIgnoreCase)) { throw 'Backup must be inside central backups' }
  $saved=Get-Content -LiteralPath (Join-Path $BackupPath 'registration.json') -Raw -Encoding UTF8|ConvertFrom-Json
  Assert-Owned (Join-Path $BackupPath 'skill')
  $dest=[string]$saved.skill
  if ($old -and $old.skill -ne $dest) { throw 'Backup belongs to a different registration' }
  Assert-Tree $dest
  if (Test-Path -LiteralPath $dest) { Assert-Owned $dest }
  Confirm-Action "Restore backup to $dest"
  $stamp=[Guid]::NewGuid().ToString('N'); $stage=Join-Path (Split-Path -Parent $dest) ".northstar-rollback-$stamp"
  $prior=Join-Path $HomeRoot "backups\rollback-prior-$stamp"; $moved=$false; $placed=$false
  Copy-Item -LiteralPath (Join-Path $BackupPath 'skill') -Destination $stage -Recurse
  $savedReg=Join-Path $HomeRoot "registration-$stamp.tmp"
  Copy-Item -LiteralPath (Join-Path $BackupPath 'registration.json') -Destination $savedReg
  # Validate the backup engine before changing registration.
  $engine=Join-Path $saved.release 'engine\bin\nsre.exe'; Assert-Normal $engine
  $releaseRoot=[IO.Path]::GetFullPath((Join-Path $HomeRoot 'releases'))+[IO.Path]::DirectorySeparatorChar
  if (![IO.Path]::GetFullPath([string]$saved.release).StartsWith($releaseRoot,[StringComparison]::OrdinalIgnoreCase)) { throw 'Invalid backup release path' }
  if ((Get-FileHash -LiteralPath $engine -Algorithm SHA256).Hash -ne $saved.engine_sha256) { throw 'Backup engine mismatch' }
  Run-Native $engine @('doctor')
  try {
   if (Test-Path -LiteralPath $dest) { Move-Item -LiteralPath $dest -Destination $prior; $moved=$true }
   Move-Item -LiteralPath $stage -Destination $dest; $placed=$true
   if (Test-Path -LiteralPath $Reg) { [IO.File]::Replace($savedReg,$Reg,(Join-Path $HomeRoot "registration-rollback-$stamp.bak")) } else { Move-Item -LiteralPath $savedReg -Destination $Reg }
  } catch {
   if ($placed) { Move-Item -LiteralPath $dest -Destination (Join-Path $HomeRoot "backups\rollback-failed-$stamp") }
   if ($moved) { Move-Item -LiteralPath $prior -Destination $dest }; throw
  }
  Write-Log 'rollback-complete' @{backup=$BackupPath}; return
 }
 if ($Action -eq 'Update' -and !$old) { throw 'Not installed; use Install' }
 & (Join-Path $Package 'scripts\Validate-Package.ps1') -Root $Package -CheckHashes
 $digest=(Get-FileHash -LiteralPath (Join-Path $Package 'docs\SHA256SUMS.txt') -Algorithm SHA256).Hash
 if ($old -and $Action -ne 'Build-Engine' -and $old.PSObject.Properties.Name -contains 'package_sha256' -and $old.package_sha256 -eq $digest) {
  if ($SkillRoot -and [IO.Path]::GetFullPath((Join-Path $SkillRoot 'northstar-re')) -ne [IO.Path]::GetFullPath([string]$old.skill)) { throw 'Already installed in another path' }
  Test-Engine $old; Write-Host 'Already installed and verified.'; Write-Log 'idempotent' @{}; return
 }
 if (!$SkillRoot -and $old) { $SkillRoot=Split-Path -Parent $old.skill }
 if (!$SkillRoot -and $Action -ne 'Build-Engine') {
  $paths=@{FreePi='.free-pi\agent\skills';Codex='.agents\skills';Pi='.pi\agent\skills';Claude='.claude\skills'}
  if ($Agent -ne 'Other') { $SkillRoot=Join-Path $env:USERPROFILE $paths[$Agent] }
  else {
   $found=@($paths.Values|ForEach-Object {Join-Path $env:USERPROFILE $_}|Where-Object {Test-Path -LiteralPath $_})
   if ($found.Count -eq 1) { $SkillRoot=$found[0] }
   elseif ($NonInteractive) { throw 'Ambiguous agent; supply -Agent or -SkillRoot' }
   else { $SkillRoot=Read-Host 'Absolute skills directory confirmed for your agent' }
  }
 }
 if ($Action -ne 'Build-Engine') {
  if (!$SkillRoot -or ![IO.Path]::IsPathRooted($SkillRoot)) { throw 'Absolute skills directory required' }
  Assert-Normal $SkillRoot; $target=Join-Path $SkillRoot 'northstar-re'; Assert-Tree $target
  if ($old -and [IO.Path]::GetFullPath($old.skill) -ne [IO.Path]::GetFullPath($target)) { throw 'Different registration; uninstall first' }
  if (Test-Path -LiteralPath $target) { Assert-Owned $target }
  Confirm-Action "Install to $target"
 }
 $stamp=[Guid]::NewGuid().ToString('N'); $work=Join-Path $HomeRoot "releases\v11-$stamp"
 New-Item -ItemType Directory -Path $work -Force|Out-Null
 Get-ChildItem -LiteralPath $Master -Force|ForEach-Object {Copy-Item -LiteralPath $_.FullName -Destination $work -Recurse}
 $exe=Join-Path $work 'engine\bin\nsre.exe'
 if ($Action -eq 'Build-Engine' -or !(Test-Path -LiteralPath $exe)) { Build-Engine $work }
 $version=& $exe --version
 if ($LASTEXITCODE -ne 0 -or $version -ne 'nsre 0.3.0') { throw 'Engine version mismatch' }; Run-Native $exe @('doctor')
 if ($Action -eq 'Build-Engine') { Write-Host "Built: $exe"; Write-Log 'built' @{}; return }
 New-Item -ItemType Directory -Path $SkillRoot -Force|Out-Null
 $stage=Join-Path $SkillRoot ".northstar-stage-$stamp"; New-Item -ItemType Directory -Path $stage|Out-Null
 foreach ($name in @('SKILL.md','specialists','references')) { Copy-Item -LiteralPath (Join-Path $work $name) -Destination $stage -Recurse }
 # Keep specialists internal even on hosts that recursively discover SKILL.md.
 foreach ($file in Get-ChildItem -LiteralPath (Join-Path $stage 'specialists') -Filter SKILL.md -Recurse) { Rename-Item -LiteralPath $file.FullName -NewName GUIDE.md }
 foreach ($file in Get-ChildItem -LiteralPath $stage -Filter '*.md' -Recurse) {
  $content=Get-Content -LiteralPath $file.FullName -Raw -Encoding UTF8
  $content=$content -replace '(specialists/[a-z0-9-]+/|\.\./[a-z0-9-]+/)SKILL\.md','$1GUIDE.md'
  [IO.File]::WriteAllText($file.FullName,$content,[Text.UTF8Encoding]::new($false))
 }
 $exePath=$exe.Replace('\','/')
 Add-Content -LiteralPath (Join-Path $stage 'SKILL.md') -Value "`n## Installed native engine`nAbsolute executable: ``$exePath``. Run doctor first; no fallback." -Encoding UTF8
 Set-Content -LiteralPath (Join-Path $stage '.northstar-framework-owner') -Value 'NorthStarRE V11' -Encoding UTF8
 $regTemp=Join-Path $HomeRoot "registration-$stamp.tmp"
 @{skill=$target;release=$work;version=[string]$Metadata.version;engine_sha256=(Get-FileHash -LiteralPath $exe -Algorithm SHA256).Hash;package_sha256=$digest;agent=$Agent}|ConvertTo-Json|Set-Content -LiteralPath $regTemp -Encoding UTF8
 $backup=$null; if ($old) { $backup=Save-Backup $old }
 $prior=Join-Path $SkillRoot ".northstar-prior-$stamp"; $moved=$false; $placed=$false
 try {
  if (Test-Path -LiteralPath $target) { Move-Item -LiteralPath $target -Destination $prior; $moved=$true }
  Move-Item -LiteralPath $stage -Destination $target; $placed=$true
  if (Test-Path -LiteralPath $Reg) { [IO.File]::Replace($regTemp,$Reg,(Join-Path $HomeRoot "registration-$stamp.bak")) }
  else { Move-Item -LiteralPath $regTemp -Destination $Reg }
 } catch {
  if ($placed) { Move-Item -LiteralPath $target -Destination (Join-Path $SkillRoot ".northstar-failed-$stamp") }
  if ($moved) { Move-Item -LiteralPath $prior -Destination $target }; throw
 }
 if ($moved) { Write-Log 'prior-retained' @{path=$prior} }
 Write-Log 'complete' @{skill=$target;backup=$backup}
 Write-Host "Installed: $target`nEngine: $exe`nReload the agent and verify its actual invocation syntax."
} catch { Write-Log 'failed' @{message=$_.Exception.Message}; throw }
finally { $lock.Dispose() }
