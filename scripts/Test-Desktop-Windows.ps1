[CmdletBinding()]
param([Parameter(Mandatory=$true)][string]$Engine,[Parameter(Mandatory=$true)][string]$Reports)
$ErrorActionPreference='Stop'
Set-StrictMode -Version Latest
if ($env:OS -ne 'Windows_NT' -or ![Environment]::UserInteractive) { throw 'Interactive Windows desktop required' }
$Engine=(Resolve-Path -LiteralPath $Engine).Path
$Reports=(Resolve-Path -LiteralPath $Reports).Path
$root=Join-Path ([IO.Path]::GetTempPath()) ('nsre-desktop-'+[Guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $root|Out-Null
$fixture=$null
$passed=$false
function Invoke-Desktop([string[]]$Arguments) {
 $raw=& $Engine desktop @Arguments
 if ($LASTEXITCODE -ne 0) { throw "Desktop command failed: $($Arguments[0])" }
 return ($raw|ConvertFrom-Json)
}
try {
 $fixturePath=Join-Path $root 'fixture.ps1'
 @'
$ErrorActionPreference='Stop'
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
function New-FixtureWindow([string]$Title) {
 $form=[Windows.Forms.Form]::new()
 $form.Text=$Title
 $form.StartPosition=[Windows.Forms.FormStartPosition]::Manual
 $form.Location=[Drawing.Point]::new(80,80)
 $form.Size=[Drawing.Size]::new(500,350)
 return $form
}
$back=New-FixtureWindow 'NSRE Desktop Fixture Back'
$front=New-FixtureWindow 'NSRE Desktop Fixture Front'
$min=New-FixtureWindow 'NSRE Desktop Fixture Minimized'
$min.Location=[Drawing.Point]::new(120,120)
$back.Show()
$min.Show()
$min.WindowState=[Windows.Forms.FormWindowState]::Minimized
$front.TopMost=$true
$button=[Windows.Forms.Button]::new()
$button.Text='Fixture START'
$button.Location=[Drawing.Point]::new(100,100)
$button.Size=[Drawing.Size]::new(120,60)
$button.Add_Click({$front.Text='NSRE Desktop Fixture Clicked'})
$front.Controls.Add($button)
$front.Show()
[Windows.Forms.Application]::Run($front)
$back.Dispose()
$min.Dispose()
$front.Dispose()
'@|Set-Content -LiteralPath $fixturePath -Encoding UTF8
 $hostExe=(Get-Process -Id $PID).Path
 $fixture=Start-Process -FilePath $hostExe -ArgumentList @('-NoProfile','-STA','-File',('"'+$fixturePath+'"')) -PassThru -RedirectStandardOutput (Join-Path $root 'fixture.out') -RedirectStandardError (Join-Path $root 'fixture.err')
 $windows=@()
 for ($i=0; $i -lt 50; $i++) {
  if ($fixture.HasExited) { throw 'Desktop fixture exited early' }
  $list=Invoke-Desktop -Arguments @('list')
  $windows=@($list.windows|Where-Object {$_.pid -eq $fixture.Id -and $_.title -like 'NSRE Desktop Fixture *'})
  if ($windows.Count -eq 3) { break }
  Start-Sleep -Milliseconds 100
 }
 if ($windows.Count -ne 3) { throw 'Not all fixture windows discovered' }
 $back=$windows|Where-Object title -eq 'NSRE Desktop Fixture Back'
 $front=$windows|Where-Object title -eq 'NSRE Desktop Fixture Front'
 $min=$windows|Where-Object title -eq 'NSRE Desktop Fixture Minimized'
 if (!$min.minimized -or $back.x -ne $front.x -or $back.y -ne $front.y) { throw 'Fixture state incorrect' }
 $owner=[string]$fixture.Id
 $x=[string]($back.x+40)
 $y=[string]($back.y+40)
 $moved=Invoke-Desktop -Arguments @('move',$back.hwnd,$owner,$x,$y,'--apply')
 $resized=Invoke-Desktop -Arguments @('resize',$back.hwnd,$owner,'640','420','--apply')
 $restored=Invoke-Desktop -Arguments @('restore',$min.hwnd,$owner,'--apply')
 $observed=Invoke-Desktop -Arguments @('inspect',$back.hwnd,$owner)
 $frontAfter=Invoke-Desktop -Arguments @('inspect',$front.hwnd,$owner)
 if (!$moved.verified -or !$resized.verified -or !$restored.verified -or $observed.x -ne [int]$x -or $observed.y -ne [int]$y -or $observed.width -ne 640 -or $observed.height -ne 420) { throw 'Window action not verified' }
 if ($frontAfter.x -ne $front.x -or $frontAfter.y -ne $front.y -or $frontAfter.width -ne $front.width -or $frontAfter.height -ne $front.height) { throw 'Unselected fixture window changed' }
 # Wrong owner and missing explicit apply must fail without mutation.
 & $Engine desktop move $back.hwnd 0 0 0 --apply 2>$null|Out-Null
 if ($LASTEXITCODE -ne 1) { throw 'Invalid PID accepted' }
 & $Engine desktop move $back.hwnd $owner 0 0 2>$null|Out-Null
 if ($LASTEXITCODE -ne 1) { throw 'Unapproved mutation accepted' }
 $focusRaw=& $Engine desktop focus $front.hwnd $owner --apply
 $focusExit=$LASTEXITCODE
 if ($focusExit -notin @(0,2)) { throw 'Unexpected focus error' }
 $focus=$focusRaw|ConvertFrom-Json
 if (($focusExit -eq 0) -ne [bool]$focus.verified) { throw 'Foreground result mismatch' }
 # Click only if normal foreground policy allowed the fixture to be active.
 $clickVerified=$false
 if ($focus.verified) {
  $clicked=Invoke-Desktop -Arguments @('click',$front.hwnd,$owner,'150','125','--apply')
  if (!$clicked.dispatched) { throw 'Fixture click not dispatched' }
  for ($i=0; $i -lt 20; $i++) {
   $state=Invoke-Desktop -Arguments @('inspect',$front.hwnd,$owner)
   if ($state.title -eq 'NSRE Desktop Fixture Clicked') { $clickVerified=$true; break }
   Start-Sleep -Milliseconds 100
  }
  if (!$clickVerified) { throw 'Fixture button effect not observed' }
 }
 $passed=$true
 @{schema=1;status='PASS';scope='interactive Windows fixture only; not FreePi or protected content acceptance';os=[Environment]::OSVersion.VersionString;covered_window_found=$true;move_verified=$true;resize_verified=$true;restore_verified=$true;unselected_window_unchanged=$true;invalid_target_rejected=$true;foreground_verified=[bool]$focus.verified;foreground_exit=$focusExit;click_effect_verified=$clickVerified}|ConvertTo-Json|Set-Content -LiteralPath (Join-Path $Reports 'desktop-control.json') -Encoding UTF8
} finally {
 if ($null -ne $fixture) {
  if (!$fixture.HasExited) { $fixture.Kill(); $fixture.WaitForExit() }
  $fixture.Dispose()
 }
 if (!$passed) { Write-Warning 'Desktop fixture did not pass; no acceptance claim' }
 Remove-Item -LiteralPath $root -Recurse -Force
}
