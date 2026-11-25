[CmdletBinding()]
param([Parameter(Mandatory=$true)][string]$Engine,[Parameter(Mandatory=$true)][string]$Output,[int[]]$Counts=@(1000,10000,100000,1000000),[int]$Jobs=4)
$ErrorActionPreference='Stop'
Set-StrictMode -Version Latest
$Engine=[IO.Path]::GetFullPath($Engine)
if (Test-Path -LiteralPath $Output) { throw 'Use a new output directory' }
New-Item -ItemType Directory -Path $Output|Out-Null
$Output=[IO.Path]::GetFullPath($Output)
$version=& $Engine --version; if ($LASTEXITCODE -ne 0) { throw 'Engine unavailable' }
$results=@()
foreach ($count in $Counts) {
 if ($count -lt 1 -or $count -gt 1000000) { throw 'Invalid corpus capacity' }
 $corpus=Join-Path $Output "corpus-$count"; New-Item -ItemType Directory -Path $corpus|Out-Null
 $payload=[Text.Encoding]::UTF8.GetBytes(('NorthStarRE synthetic fixture. No user or game data. '.PadRight(64,'0')))
 $expected=([BitConverter]::ToString([Security.Cryptography.SHA256]::Create().ComputeHash($payload))).Replace('-','').ToLowerInvariant()
 for ($i=0;$i -lt $count;$i++) {
  $dir=Join-Path $corpus ([string][int][Math]::Floor($i/1000))
  if (($i%1000) -eq 0) { New-Item -ItemType Directory -Path $dir|Out-Null }
  [IO.File]::WriteAllBytes((Join-Path $dir "$i.dat"),$payload)
 }
 $db=Join-Path $Output "index-$count.sqlite"
 foreach ($mode in @('strict','reuse-metadata')) {
  $stdout=Join-Path $Output "$count-$mode.json"; $stderr=Join-Path $Output "$count-$mode.stderr"
  $watch=[Diagnostics.Stopwatch]::StartNew()
  $process=Start-Process -FilePath $Engine -ArgumentList @('index','--root',('"'+$corpus+'"'),'--db',('"'+$db+'"'),'--jobs',$Jobs,'--mode',$mode) -PassThru -RedirectStandardOutput $stdout -RedirectStandardError $stderr
  $peak=0L
  while (!$process.HasExited) { try {$process.Refresh();$peak=[Math]::Max($peak,$process.WorkingSet64)} catch {}; Start-Sleep -Milliseconds 20 }
  $process.WaitForExit(); $watch.Stop(); $process.Refresh()
  if ($process.ExitCode -ne 0) { throw "Index failed: $count $mode" }
  $result=Get-Content -LiteralPath $stdout -Raw|ConvertFrom-Json
  if ($result.state -ne 'complete' -or ($result.fresh+$result.reused) -ne $count -or $result.bytes -ne ($count*$payload.Length)) { throw 'Index output accuracy failure' }
  $export=Join-Path $Output "$count-$mode.jsonl"
  & $Engine export --db $db --run $result.run_id | Set-Content -LiteralPath $export -Encoding UTF8
  if ($LASTEXITCODE -ne 0) { throw 'Export failed' }
  $reader=[IO.File]::OpenText($export); $actual=0
  try {while ($null -ne ($line=$reader.ReadLine())) { $row=$line|ConvertFrom-Json; if ($row.kind -eq 'file') {if ($row.sha256 -ne $expected -or $row.size -ne $payload.Length) {throw 'Export hash mismatch'};$actual++} }} finally {$reader.Dispose()}
  if ($actual -ne $count) { throw 'Export count mismatch' }
  $cpu=$null; try {$cpu=$process.TotalProcessorTime.TotalSeconds} catch {}
  $results+=@{count=$count;mode=$mode;wall_seconds=$watch.Elapsed.TotalSeconds;files_per_second=$count/$watch.Elapsed.TotalSeconds;bytes_per_second=($count*$payload.Length)/$watch.Elapsed.TotalSeconds;cpu_seconds=$cpu;sampled_peak_rss_bytes=$peak;database_bytes=(Get-Item -LiteralPath $db).Length;cache_hit_ratio=$result.reused/$count;accuracy_verified=$true;recovery_test='not fault-injected; prior tmpfs exhaustion recorded separately';cache_condition='not controlled';jobs=$Jobs;payload_bytes=$payload.Length}
  @{schema_version=1;engine_version=$version;host=[Runtime.InteropServices.RuntimeInformation]::OSDescription;arch=[Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString();cpu_count=[Environment]::ProcessorCount;utc=[DateTime]::UtcNow.ToString('o');results=$results}|ConvertTo-Json -Depth 10|Set-Content -LiteralPath (Join-Path $Output 'results.json') -Encoding UTF8
 }
 Remove-Item -LiteralPath $corpus -Recurse -Force
}
@{schema_version=1;engine_version=$version;host=[Runtime.InteropServices.RuntimeInformation]::OSDescription;arch=[Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString();cpu_count=[Environment]::ProcessorCount;utc=[DateTime]::UtcNow.ToString('o');results=$results}|ConvertTo-Json -Depth 10|Set-Content -LiteralPath (Join-Path $Output 'results.json') -Encoding UTF8
