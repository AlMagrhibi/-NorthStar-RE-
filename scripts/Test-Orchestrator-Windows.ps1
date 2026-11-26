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
 $db=Join-Path $root 'jobs.sqlite'
 $input=Join-Path $root 'input.json'
 [IO.File]::WriteAllText($input,'{"schema":1,"tasks":[{"id":"data","input":{"operation":"json-roundtrip","data":{"x":1}}},{"id":"qa","input":{"operation":"regression-check","baseline":1,"actual":1}}]}',[Text.UTF8Encoding]::new($false))
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
 $cancel=Invoke-Engine -Arguments @('queue','status',$db,'cancelled')|ConvertFrom-Json
 if ($cancel.state -ne 'cancelled') { throw 'Cancelled task executed' }
 @{schema=1;status='PASS';scope='Windows hosted runner; not Windows 11 desktop or FreePi acceptance';registry_engines=12;concurrent_claims=$claimed;caller_input_sha256=$callerHash;cancelled_job_preserved=$true;commit=$env:GITHUB_SHA}|ConvertTo-Json -Depth 5|Set-Content -LiteralPath (Join-Path $Reports 'orchestrator-windows.json') -Encoding UTF8
} finally {
 foreach ($worker in $workers) {
  if (!$worker.HasExited) { $worker.Kill(); $worker.WaitForExit() }
  $worker.Dispose()
 }
 Remove-Item -LiteralPath $root -Recurse -Force
}
