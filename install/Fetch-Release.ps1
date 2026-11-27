# Explicit online acquisition. Does not install or execute downloaded files.
[CmdletBinding()]
param([Parameter(Mandatory=$true)][string]$Destination,[string]$Tag,[switch]$AllowPrerelease)
$ErrorActionPreference='Stop'
Set-StrictMode -Version Latest
if (!(Get-Command gh -ErrorAction SilentlyContinue)) { throw 'Authenticated GitHub CLI required for private release download. Never paste tokens.' }
$repo='AlMagrhibi/-NorthStar-RE-'
if (!$Tag) {
 $json=& gh api "repos/$repo/releases"; if ($LASTEXITCODE -ne 0) { throw 'Release discovery failed' }
 $parsed=$json|ConvertFrom-Json
 $releases=@($parsed|Where-Object {!$_.draft -and ($AllowPrerelease -or !$_.prerelease) -and $_.tag_name -match '^v1[12]\.'})
 if (!$releases.Count) { throw 'No compatible release. Local installation remains usable offline.' }; $Tag=$releases[0].tag_name
}
if ($Tag -notmatch '^v1[12]\.[0-9]+\.[0-9]+(?:-dev\.[0-9]+|-rc\.[0-9]+)?$') { throw 'Unsupported tag' }
if ($Tag -match '-dev\.') { throw 'Development builds are not distributed through releases' }
if ($Tag -match '-rc\.' -and !$AllowPrerelease) { throw 'Explicit prerelease opt-in required' }
if (Test-Path -LiteralPath $Destination) { throw 'Use a new destination directory' }
New-Item -ItemType Directory -Path $Destination|Out-Null
& gh release download $Tag --repo $repo --dir $Destination --pattern 'NorthStar-RE-Framework-*-Windows11-x64.zip' --pattern 'SHA256SUMS.txt' --pattern 'release-manifest.json'
if ($LASTEXITCODE -ne 0) { throw 'Download failed; no installation performed' }
$manifest=Get-Content -LiteralPath (Join-Path $Destination 'release-manifest.json') -Raw -Encoding UTF8|ConvertFrom-Json
if ($manifest.schema_version -ne 1 -or $manifest.tag -ne $Tag -or $manifest.framework_major -notin @(11,12) -or $manifest.engine_version -ne '0.3.0' -or $manifest.platform -ne 'windows-x64' -or !$manifest.required_checks_passed -or $manifest.publication_authorized -ne $true) { throw 'Incompatible release manifest' }
$name='NorthStar-RE-Framework-' + $(if ($manifest.framework_major -eq 12) {'V12'} else {'V11'}) + '-Windows11-x64.zip'
 $name=$name.Trim()
$entry=@($manifest.assets|Where-Object {$_.name -eq $name})
if ($entry.Count -ne 1 -or $entry[0].sha256 -notmatch '^[a-f0-9]{64}$') { throw 'Missing or invalid artifact declaration' }
$hash=(Get-FileHash -LiteralPath (Join-Path $Destination $name) -Algorithm SHA256).Hash.ToLowerInvariant()
$sum=@(Get-Content -LiteralPath (Join-Path $Destination 'SHA256SUMS.txt')|Where-Object {$_ -ceq ($hash+'  '+$name)})
if ($hash -ne $entry[0].sha256 -or $sum.Count -ne 1) { throw 'Release checksum mismatch' }
Write-Host "Verified: $(Join-Path $Destination $name). Review the authenticated release, extract to a new directory, then run Update.cmd. No silent update performed."
