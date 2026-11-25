param([string]$Root=(Split-Path -Parent $PSScriptRoot))
$ErrorActionPreference='Stop'
$Root=[IO.Path]::GetFullPath($Root)
$manifest=Join-Path $Root 'docs/SHA256SUMS.txt'
$lines=Get-ChildItem -LiteralPath $Root -Recurse -File -Force|Where-Object {$_.FullName -ne $manifest -and $_.FullName -notmatch '[/\\](target|\.git|dist)[/\\]'}|Sort-Object FullName|ForEach-Object {
 (Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash.ToLowerInvariant()+'  '+$_.FullName.Substring($Root.Length+1).Replace('\','/')
}
[IO.File]::WriteAllLines($manifest,[string[]]$lines,[Text.UTF8Encoding]::new($false))
