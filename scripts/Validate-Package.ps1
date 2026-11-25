[CmdletBinding()]
param([string]$Root=(Split-Path -Parent $PSScriptRoot),[switch]$CheckHashes)
$ErrorActionPreference='Stop'
Set-StrictMode -Version Latest
$Root=[IO.Path]::GetFullPath($Root)
$all=@(Get-ChildItem -LiteralPath $Root -File -Recurse -Force|Where-Object {$_.FullName -notmatch '[/\\](target|\.git|dist)[/\\]'})
foreach ($f in $all) {
 if ($f.Extension -match '^\.py[co]?$') { throw 'Python artifact forbidden' }
 if ($f.Attributes -band [IO.FileAttributes]::ReparsePoint) { throw 'Reparse file forbidden' }
 $text=[IO.File]::ReadAllText($f.FullName)
 if ($text -match '(?m)(gh[pousr]_[A-Za-z0-9]{30,}|github_pat_[A-Za-z0-9_]{30,}|AKIA[0-9A-Z]{16}|-----BEGIN (?:RSA |OPENSSH |EC )?PRIVATE KEY-----)') { throw "Potential credential in $($f.Name); do not publish" }
 if ($f.Extension -in @('.ps1','.cmd','.yml','.yaml','.toml') -and $text -match '(?im)^\s*(?:-\s*run:\s*)?(?:python[23]?|py\s+-3|pip[23]?)\s') { throw 'Interpreter invocation forbidden' }
}
$master=Join-Path $Root 'skills/northstar-re'
$skills=@(Get-ChildItem -LiteralPath (Join-Path $master 'specialists') -Directory)
if ($skills.Count -ne 52) { throw "Expected 52 specialists; found $($skills.Count)" }
foreach ($d in @((Get-Item -LiteralPath $master))+$skills) {
 $p=Join-Path $d.FullName 'SKILL.md'; $text=[IO.File]::ReadAllText($p)
 if ($text -notmatch '(?s)^---\r?\nname: ([a-z0-9-]+)\r?\ndescription: .+?\r?\n---') { throw "Invalid manifest: $p" }
 if ($Matches[1] -ne $d.Name) { throw "Skill name mismatch: $p" }
}
$registry=Get-Content -LiteralPath (Join-Path $Root 'schemas/specialists.json') -Raw|ConvertFrom-Json
$names=@($skills.Name|Sort-Object)
if (@(Compare-Object $names @($registry.specialists|ForEach-Object {$_.id}|Sort-Object)).Count) { throw 'Specialist registry drift' }
$tools=@(Get-Content -LiteralPath (Join-Path $master 'specialists/environment-discovery-setup/references/tool-catalog.json') -Raw|ConvertFrom-Json)
if (@($tools|Group-Object id|Where-Object Count -gt 1).Count) { throw 'Duplicate tool identifier' }
foreach ($t in $tools) { if (!$t.id -or !$t.name -or $t.official_url -notmatch '^https://') { throw 'Invalid tool entry' } }
# Validate tracked Markdown file links, excluding external URLs and local anchors.
$links=0
foreach ($f in $all|Where-Object Extension -eq '.md') {
 $text=[IO.File]::ReadAllText($f.FullName)
 foreach ($m in [regex]::Matches($text,'!?\[[^\]\r\n]*\]\(([^)\s]+)(?:\s+"[^"]*")?\)')) {
  $dest=$m.Groups[1].Value
  if ($dest -match '^(?:[a-zA-Z][a-zA-Z0-9+.-]*:|#|/)') { continue }
  $dest=($dest -split '#',2)[0]; if (!$dest) { continue }
  $resolved=[IO.Path]::GetFullPath((Join-Path $f.DirectoryName $dest))
  if (!(Test-Path -LiteralPath $resolved)) { throw "Broken link in $($f.Name): $dest" }; $links++
 }
 foreach ($m in [regex]::Matches($text,'`(specialists/[a-z0-9-]+/SKILL\.md|references/[a-zA-Z0-9/_.-]+\.md)`')) {
  if (!(Test-Path -LiteralPath (Join-Path $master $m.Groups[1].Value)) -and !(Test-Path -LiteralPath (Join-Path $f.DirectoryName $m.Groups[1].Value))) { throw "Broken routing reference in $($f.FullName): $($m.Groups[1].Value)" }
 }
}
foreach ($f in $all|Where-Object Extension -eq '.ps1') {
 $parseErrors=$null; $tokens=$null
 [void][Management.Automation.Language.Parser]::ParseFile($f.FullName,[ref]$tokens,[ref]$parseErrors)
 if ($parseErrors.Count) { throw "PowerShell parse failure: $($f.Name): $($parseErrors[0].Message)" }
}
if ($CheckHashes) {
 $seen=@{}; $manifest=Join-Path $Root 'docs/SHA256SUMS.txt'
 foreach ($line in Get-Content -LiteralPath $manifest) {
  if ($line -notmatch '^([a-f0-9]{64})  (.+)$') { throw 'Malformed checksum record' }
  $hash=$Matches[1]; $rel=$Matches[2]
  if ($rel -match '(^/|\\|:|(^|/)\.\.(/|$))' -or $seen.ContainsKey($rel)) { throw 'Unsafe or duplicate checksum path' }
  $seen[$rel]=$true; $path=Join-Path $Root $rel
  if ((Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash.ToLowerInvariant() -ne $hash) { throw "Checksum mismatch: $rel" }
 }
 foreach ($f in $all) {
  $rel=$f.FullName.Substring($Root.Length+1).Replace('\','/')
  if ($rel -ne 'docs/SHA256SUMS.txt' -and !$seen.ContainsKey($rel)) { throw "Unlisted file: $rel" }
 }
}
@{status='PASS';files=$all.Count;specialists=$skills.Count;tools=$tools.Count;local_links=$links;hashes_checked=[bool]$CheckHashes;secret_check='limited-pattern scan; not certification';python_runtime=$false}|ConvertTo-Json
