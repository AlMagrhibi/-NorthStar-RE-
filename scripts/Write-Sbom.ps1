param([Parameter(Mandatory=$true)][string]$Output,[string]$Root=(Split-Path -Parent $PSScriptRoot))
$ErrorActionPreference='Stop'
$manifest=Join-Path $Root 'skills/northstar-re/engine/nsre/Cargo.toml'
$json=& cargo metadata --manifest-path $manifest --locked --format-version 1
if ($LASTEXITCODE -ne 0) { throw 'Cargo metadata failed' }
$metadata=$json|ConvertFrom-Json
$components=@($metadata.packages|Where-Object {$_.name -ne 'nsre'}|Sort-Object name,version|ForEach-Object {
 if (!$_.license) { throw "Missing dependency license: $($_.name)" }
 @{type='library';name=$_.name;version=$_.version;'bom-ref'="pkg:cargo/$($_.name)@$($_.version)";purl="pkg:cargo/$($_.name)@$($_.version)";licenses=@(@{expression=($_.license -replace '/', ' OR ')});properties=@(@{name='northstar:upstream-declared-license';value=$_.license})}
})
$dependencies=@($metadata.resolve.nodes|ForEach-Object {
 $id=$_.id; $package=$metadata.packages|Where-Object {$_.id -eq $id}|Select-Object -First 1
 $refs=@($_.dependencies|ForEach-Object { $depId=$_; $dep=$metadata.packages|Where-Object {$_.id -eq $depId}|Select-Object -First 1; "pkg:cargo/$($dep.name)@$($dep.version)" })
 @{ref="pkg:cargo/$($package.name)@$($package.version)";dependsOn=$refs}
})
@{bomFormat='CycloneDX';specVersion='1.5';version=1;serialNumber=('urn:uuid:'+ [Guid]::NewGuid());metadata=@{timestamp=[DateTime]::UtcNow.ToString('o');component=@{type='application';name='nsre';version='0.3.0';'bom-ref'='pkg:cargo/nsre@0.3.0'}};components=$components;dependencies=$dependencies}|ConvertTo-Json -Depth 20|Set-Content -LiteralPath $Output -Encoding UTF8
