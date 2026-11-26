[CmdletBinding()]
param([Parameter(Mandatory=$true)][string]$Engine,[Parameter(Mandatory=$true)][string]$Reports)
$ErrorActionPreference='Stop'
Set-StrictMode -Version Latest
if ($env:OS -ne 'Windows_NT') { throw 'Windows host required' }
$Engine=(Resolve-Path -LiteralPath $Engine).Path
$Reports=(Resolve-Path -LiteralPath $Reports).Path
$root=Join-Path ([IO.Path]::GetTempPath()) ('nsre-queue-test-'+[Guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $root|Out-Null
$workers=@()
function Invoke-Engine([string[]]$Arguments) {
 $result=& $Engine @Arguments
 if ($LASTEXITCODE -ne 0) { throw "Engine command failed: $($Arguments[0])" }
 return $result
}
try {
 $registry=Invoke-Engine -Arguments @('engines')|ConvertFrom-Json
 if ($registry.schema -ne 1 -or @($registry.engines).Count -ne 12) { throw 'Engine registry mismatch' }
 # Check the desktop CLI rejects invalid targets without interacting with runner windows.
 & $Engine desktop inspect 0 1 2>$null|Out-Null
 if ($LASTEXITCODE -ne 1) { throw 'Desktop invalid handle accepted' }
 $source=Join-Path $root 'source'
 New-Item -ItemType Directory -Path $source|Out-Null
 [IO.File]::WriteAllText((Join-Path $source 'a.txt'),'abc',[Text.UTF8Encoding]::new($false))
 $manifest=Join-Path $root 'collect-input.json'
 $output=Join-Path $root 'collect-output.json'
 [IO.File]::WriteAllText($manifest,'{"schema":1,"files":[{"path":"a.txt","expected_sha256":"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"}]}',[Text.UTF8Encoding]::new($false))
 Invoke-Engine -Arguments @('collect','--root',$source,'--input',$manifest,'--out',$output,'--jobs','2','--reviewed-redacted')|Out-Null
 $collected=Get-Content -LiteralPath $output -Raw|ConvertFrom-Json
 if (!$collected.passed -or !$collected.results[0].hash_recomputed -or $collected.results[0].bytes -ne 3) { throw 'Local evidence collection failed' }
 # Parse the actual freshly built MSVC image through the read-only collector.
 Copy-Item -LiteralPath $Engine -Destination (Join-Path $source 'nsre-fixture.exe')
 $peManifest=Join-Path $root 'pe-input.json'
 $peOutput=Join-Path $root 'pe-output.json'
 @{schema=1;files=@(@{path='nsre-fixture.exe';require_pe_layout=$true;expected_sha256=(Get-FileHash -LiteralPath $Engine -Algorithm SHA256).Hash.ToLowerInvariant()})}|ConvertTo-Json -Depth 5|Set-Content -LiteralPath $peManifest -Encoding utf8NoBOM
 Invoke-Engine -Arguments @('collect','--root',$source,'--input',$peManifest,'--out',$peOutput,'--jobs','1','--reviewed-redacted')|Out-Null
 $pe=Get-Content -LiteralPath $peOutput -Raw|ConvertFrom-Json
 if (!$pe.passed -or !$pe.results[0].pe_layout.validated -or $pe.results[0].pe_layout.architecture -ne 'x64' -or $pe.results[0].pe_layout.full_format_validated) { throw 'Built MSVC PE layout fixture failed' }
 $allInput=Join-Path $root 'all-engines-input.json'
 $allOutput=Join-Path $root 'all-engines-output.json'
 [IO.File]::WriteAllText($allInput,'{"schema":1,"tasks":[{"id":"core","engine":"native-core","input":{"operation":"content-digest","utf8":"abc"}},{"id":"binary","engine":"binary-intelligence","input":{"operation":"binary-header","hex":"4d5a"}},{"id":"semantic","engine":"semantic-intelligence","depends_on":["binary"],"input":{"operation":"semantic-reachability","nodes":["a","b"],"edges":[{"from":"a","to":"b"}],"root":"a"}},{"id":"algorithm","engine":"algorithm-intelligence","input":{"operation":"algorithm-catalog"}},{"id":"data","engine":"data-engineering","input":{"operation":"json-roundtrip","data":{"fixture":1}}},{"id":"runtime","engine":"runtime-diagnostics","input":{"operation":"ui-report","windows":[{"id":"fixture","responsive":true}]}},{"id":"compatibility","engine":"compatibility","input":{"operation":"protocol-check","version":"1","expected_version":"1","fields":{"id":"integer"},"data":{"id":1}}},{"id":"security","engine":"security","input":{"operation":"security-regression-report","gates":[{"id":"fixture","status":"passed","evidence_sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}]}},{"id":"performance","engine":"performance","input":{"operation":"resource-report","resource_samples":[1,2],"limit":2,"unit":"MiB"}},{"id":"knowledge","engine":"knowledge","input":{"operation":"evidence-check","records":[{"id":"fixture","sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","confidence":"observed","producer":"synthetic","references":[]}]}},{"id":"recovery","engine":"recovery","input":{"operation":"checkpoint-compare","expected":[{"id":"a","sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}],"actual":[{"id":"a","sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}]}},{"id":"qa","engine":"quality-assurance","depends_on":["core","binary","semantic","algorithm","data","runtime","compatibility","security","performance","knowledge","recovery"],"input":{"operation":"regression-check","baseline":1,"actual":1}}]}',[Text.UTF8Encoding]::new($false))
 Invoke-Engine -Arguments @('batch','--input',$allInput,'--out',$allOutput,'--jobs','4','--reviewed-redacted')|Out-Null
 $all=Get-Content -LiteralPath $allOutput -Raw|ConvertFrom-Json
 if (!$all.passed -or @($all.results).Count -ne 12 -or @($all.results.engine|Sort-Object -Unique).Count -ne 12 -or $all.dependency_layers -ne 3) { throw 'Twelve-engine dispatch fixture failed' }
 if (@($all.results|Where-Object {$_.state -ne 'completed' -or !$_.result.passed}).Count -ne 0) { throw 'Engine operation failed' }
 $db=Join-Path $root 'jobs.sqlite'
 $input=Join-Path $root 'input.json'
 [IO.File]::WriteAllText($input,'{"schema":1,"tasks":[{"id":"data","input":{"operation":"json-roundtrip","data":{"x":1}}},{"id":"qa","depends_on":["data"],"input":{"operation":"regression-check","baseline":1,"actual":1}}]}',[Text.UTF8Encoding]::new($false))
 Invoke-Engine -Arguments @('queue','init',$db)|Out-Null
 foreach ($id in @('first','second','cancelled')) {
  Invoke-Engine -Arguments @('queue','submit',$db,$id,$input,'50','2','--reviewed-redacted')|Out-Null
 }
 Invoke-Engine -Arguments @('queue','cancel',$db,'cancelled')|Out-Null
 $queued=Invoke-Engine -Arguments @('queue','status',$db,'first')|ConvertFrom-Json
 if ($queued.state -ne 'queued') { throw 'Submit must not execute inline' }
 foreach ($i in 1..2) {
  $workers+=Start-Process -FilePath $Engine -ArgumentList @('queue','run-one',('"'+$db+'"')) -PassThru -NoNewWindow -RedirectStandardOutput (Join-Path $root "worker-$i.json") -RedirectStandardError (Join-Path $root "worker-$i.err")
 }
 # Independent caller-side work before waiting for workers; not proof of a live agent integration.
 $callerHash=(Get-FileHash -LiteralPath $input -Algorithm SHA256).Hash.ToLowerInvariant()
 foreach ($worker in $workers) {
  if (!$worker.WaitForExit(30000)) { throw 'Queue worker timed out' }
  if ($worker.ExitCode -ne 0) { throw 'Queue worker failed' }
 }
 $claimed=@(1..2|ForEach-Object {(Get-Content -LiteralPath (Join-Path $root "worker-$_.json") -Raw|ConvertFrom-Json).id})
 if (@($claimed|Sort-Object -Unique).Count -ne 2) { throw 'Concurrent duplicate claim' }
 foreach ($id in @('first','second')) {
  $status=Invoke-Engine -Arguments @('queue','status',$db,$id)|ConvertFrom-Json
  if ($status.state -ne 'completed' -or !$status.result.passed -or @($status.result.results).Count -ne 2) { throw "Invalid result: $id" }
 }
 Invoke-Engine -Arguments @('queue','submit',$db,'drain-test',$input,'50','2','--reviewed-redacted')|Out-Null
 $drained=Invoke-Engine -Arguments @('queue','drain',$db,'2')|ConvertFrom-Json
 if (!$drained.passed -or !$drained.idle_observed -or @($drained.jobs).Count -ne 1 -or $drained.jobs[0].id -ne 'drain-test') { throw 'Queue drain failed' }
 $waited=Invoke-Engine -Arguments @('queue','wait',$db,'first','0')|ConvertFrom-Json
 if (!$waited.terminal -or $waited.timed_out -or $waited.input_sha256 -ne $callerHash -or $waited.result_sha256 -notmatch '^[0-9a-f]{64}$') { throw 'Wait/provenance fixture failed' }
 $cancel=Invoke-Engine -Arguments @('queue','status',$db,'cancelled')|ConvertFrom-Json
 if ($cancel.state -ne 'cancelled') { throw 'Cancelled task executed' }
 # Cross-job ordering and independently hashed local collection through the queue.
 $scheduled=Join-Path $root 'scheduled.json'
 $collector=Join-Path $root 'collector-job.json'
 @{schema=1;queue=@{schema=1;depends_on=@('dag-parent');exclusive_keys=@('fixture-index')};tasks=@(@{id='check';input=@{operation='json-roundtrip';data=1}})}|ConvertTo-Json -Depth 8|Set-Content -LiteralPath $scheduled -Encoding utf8NoBOM
 @{schema=2;kind='collect';root=$source;manifest=@{schema=1;files=@(@{path='a.txt';expected_sha256='ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad'})};queue=@{schema=1;depends_on=@('dag-child')}}|ConvertTo-Json -Depth 8|Set-Content -LiteralPath $collector -Encoding utf8NoBOM
 Invoke-Engine -Arguments @('queue','submit',$db,'dag-parent',$input,'0','1','--reviewed-redacted')|Out-Null
 Invoke-Engine -Arguments @('queue','submit',$db,'dag-child',$scheduled,'100','1','--reviewed-redacted')|Out-Null
 Invoke-Engine -Arguments @('queue','submit',$db,'collect-job',$collector,'100','1','--reviewed-redacted')|Out-Null
 $ordered=Invoke-Engine -Arguments @('queue','drain',$db,'4')|ConvertFrom-Json
 if (!$ordered.passed -or @($ordered.jobs).Count -ne 3 -or ($ordered.jobs.id -join ',') -ne 'dag-parent,dag-child,collect-job') { throw 'Cross-job dependency ordering failed' }
 $observed=Invoke-Engine -Arguments @('queue','status',$db,'collect-job')|ConvertFrom-Json
 if (!$observed.result.results[0].hash_recomputed -or $observed.result.results[0].sha256 -ne 'ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad') { throw 'Queued collector evidence failed' }
 [IO.File]::WriteAllText((Join-Path $source 'a.txt'),'changed',[Text.UTF8Encoding]::new($false))
 Invoke-Engine -Arguments @('queue','submit',$db,'stale-collect',$collector,'100','1','--reviewed-redacted')|Out-Null
 & $Engine queue run-one $db|Out-Null
 if ($LASTEXITCODE -ne 2) { throw 'Stale collector did not fail closed' }
 $stale=Invoke-Engine -Arguments @('queue','status',$db,'stale-collect')|ConvertFrom-Json
 if ($stale.state -ne 'failed' -or $stale.result.results[0].passed) { throw 'Stale source accepted' }
 @{schema=1;status='PASS';scope='Windows hosted runner; not Windows 11 desktop or FreePi acceptance';registry_engines=12;all_engine_fixture_passed=$true;local_collection_passed=$true;concurrent_claims=$claimed;caller_input_sha256=$callerHash;cancelled_job_preserved=$true;dependency_fixture_passed=$true;queue_drain_passed=$true;queue_wait_provenance_passed=$true;cross_job_dag_passed=$true;queued_collector_passed=$true;stale_source_rejected=$true;pe_layout_passed=$true;commit=$env:GITHUB_SHA}|ConvertTo-Json -Depth 5|Set-Content -LiteralPath (Join-Path $Reports 'orchestrator-windows.json') -Encoding UTF8
} finally {
 foreach ($worker in $workers) {
  if (!$worker.HasExited) { $worker.Kill(); $worker.WaitForExit() }
  $worker.Dispose()
 }
 Remove-Item -LiteralPath $root -Recurse -Force
}
