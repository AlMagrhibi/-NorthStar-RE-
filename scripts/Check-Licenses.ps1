param([string]$Root=(Split-Path -Parent $PSScriptRoot))
$ErrorActionPreference='Stop'
$json=& cargo metadata --manifest-path (Join-Path $Root 'skills/northstar-re/engine/nsre/Cargo.toml') --locked --format-version 1
if ($LASTEXITCODE -ne 0) { throw 'Cargo metadata failed' }
$packages=($json|ConvertFrom-Json).packages
# A conservative complete-expression allowlist; unexpected additions require review.
$allowed=@('MIT','Apache-2.0','MIT OR Apache-2.0','Apache-2.0 OR MIT','MIT/Apache-2.0','Apache-2.0/MIT','BSD-3-Clause','BSD-2-Clause','ISC','Zlib','CC0-1.0','Unlicense','MIT OR Unlicense','Unlicense OR MIT','Apache-2.0 OR MIT OR Zlib','MIT OR Apache-2.0 OR Zlib','(MIT OR Apache-2.0) AND BSD-3-Clause','MIT OR Apache-2.0 OR LGPL-2.1-or-later','Unlicense/MIT','(MIT OR Apache-2.0) AND Unicode-3.0','BSD-2-Clause OR Apache-2.0 OR MIT')
foreach ($p in $packages) { if ($p.name -eq 'nsre' -and !$p.source) { continue }; if (!$p.license -or $p.license -notin $allowed) { throw "License review required: $($p.name) $($p.version) [$($p.license)]" } }
@{status='PASS';packages=@($packages).Count;scope='declared SPDX expressions only; notices/legal review separate'}|ConvertTo-Json
