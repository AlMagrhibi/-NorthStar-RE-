# A real isolated Windows lifecycle test; uses the actual CI-built engine, no mock EXE.
param([string]$Root=(Split-Path -Parent $PSScriptRoot))
$ErrorActionPreference='Stop'
if ($env:OS -ne 'Windows_NT') { throw 'Windows required for lifecycle verification' }
$oldLocal=$env:LOCALAPPDATA; $oldUser=$env:USERPROFILE
$temp=Join-Path ([IO.Path]::GetTempPath()) ('nsre-lifecycle-'+[Guid]::NewGuid().ToString('N'))
function Invoke-Manager([string[]]$Arguments,[bool]$Success=$true) {
 & powershell.exe -NoProfile -File (Join-Path $Root 'install/Manage.ps1') @Arguments
 if (($LASTEXITCODE -eq 0) -ne $Success) { throw "Unexpected installer exit: $LASTEXITCODE" }
}
try {
 New-Item -ItemType Directory -Path $temp|Out-Null
 $env:LOCALAPPDATA=Join-Path $temp 'local'; $env:USERPROFILE=Join-Path $temp 'user'
 $skills=Join-Path $temp 'agent/skills'; $target=Join-Path $skills 'northstar-re'
 New-Item -ItemType Directory -Path $target -Force|Out-Null
 Set-Content -LiteralPath (Join-Path $target 'sentinel.txt') -Value 'preserve'
 Invoke-Manager @('-Action','Install','-SkillRoot',$skills,'-NonInteractive') $false
 if (!(Test-Path -LiteralPath (Join-Path $target 'sentinel.txt'))) { throw 'Unowned skill was modified' }
 Remove-Item -LiteralPath $target -Recurse
 Invoke-Manager @('-Action','Install','-SkillRoot',$skills,'-NonInteractive')
 $reg=Join-Path $env:LOCALAPPDATA 'NorthStarRE/registration.json'
 $first=Get-Content -LiteralPath $reg -Raw
 Invoke-Manager @('-Action','Install','-SkillRoot',$skills,'-NonInteractive')
 if ((Get-Content -LiteralPath $reg -Raw) -ne $first) { throw 'Not idempotent' }
 Invoke-Manager @('-Action','Doctor')
 if (@(Get-ChildItem -LiteralPath $target -Filter SKILL.md -Recurse).Count -ne 1) { throw 'Specialists leaked into discovery' }
 Invoke-Manager @('-Action','Backup')
 $backup=(Get-ChildItem -LiteralPath (Join-Path $env:LOCALAPPDATA 'NorthStarRE/backups') -Directory|Select-Object -First 1).FullName
 # Change only the installed package digest to exercise an upgrade of the same candidate.
 $r=Get-Content -LiteralPath $reg -Raw|ConvertFrom-Json; $r.package_sha256='upgrade-fixture'; $r|ConvertTo-Json|Set-Content -LiteralPath $reg
 Invoke-Manager @('-Action','Update','-SkillRoot',$skills,'-NonInteractive')
 Invoke-Manager @('-Action','Rollback','-BackupPath',$backup,'-NonInteractive')
 Invoke-Manager @('-Action','Doctor')
 $project=Join-Path $temp 'project/.northstar'; New-Item -ItemType Directory -Path $project -Force|Out-Null
 Set-Content -LiteralPath (Join-Path $project 'sentinel') -Value 'project-data'
 Invoke-Manager @('-Action','Uninstall','-NonInteractive')
 if ((Test-Path -LiteralPath $target) -or (Test-Path -LiteralPath $reg)) { throw 'Registration remains' }
 if (!(Test-Path -LiteralPath (Join-Path $project 'sentinel'))) { throw 'Project data touched' }
 'PASS: clean installation, ownership refusal, idempotency, doctor, backup, update, rollback and uninstall'
} finally { $env:LOCALAPPDATA=$oldLocal; $env:USERPROFILE=$oldUser; if (Test-Path -LiteralPath $temp) { Remove-Item -LiteralPath $temp -Recurse -Force } }
