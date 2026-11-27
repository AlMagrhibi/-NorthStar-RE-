[CmdletBinding()]
param([Parameter(Mandatory=$true)][string]$Engine,[Parameter(Mandatory=$true)][string]$Reports,[switch]$HostedProbe)
$ErrorActionPreference='Stop'
Set-StrictMode -Version Latest
if ($env:OS -ne 'Windows_NT') { throw 'Windows required' }
if (![Environment]::UserInteractive) {
 if (!$HostedProbe) { throw 'Interactive Windows desktop required' }
 New-Item -ItemType Directory -Path $Reports -Force|Out-Null
 @{schema=1;status='BLOCKED';reason='Hosted runner has no interactive desktop';keyboard_effect_verified=$false;uia_verified=$false;capture_verified=$false}|ConvertTo-Json|Set-Content -LiteralPath (Join-Path $Reports 'desktop-control.json') -Encoding UTF8
 return
}
$Engine=(Resolve-Path -LiteralPath $Engine).Path
$Reports=(Resolve-Path -LiteralPath $Reports).Path
$root=Join-Path ([IO.Path]::GetTempPath()) ('nsre-desktop-'+[Guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $root|Out-Null
$fixture=$null
$bridge=$null
$passed=$false
$wgcVerified=$false;$bridgeEffect=$false;$keyboardVerified=$false;$clickVerified=$false
$rightVerified=$false;$middleVerified=$false;$doubleVerified=$false;$wheelVerified=$false;$dragVerified=$false
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
$back.BackColor=[Drawing.Color]::FromArgb(192,64,32)
$back.Show()
$min.Show()
$min.WindowState=[Windows.Forms.FormWindowState]::Minimized
$front.TopMost=$true
$button=[Windows.Forms.Button]::new()
$button.Text='Fixture START'
$button.Name='nsre-start'
$button.AccessibleName='NSRE Start Button'
$button.Location=[Drawing.Point]::new(100,100)
$button.Size=[Drawing.Size]::new(120,60)
$script:clickCount=0
$button.Add_Click({$script:clickCount++;$front.Text='NSRE Desktop Fixture Clicked '+$script:clickCount})
$front.Controls.Add($button)
$bridgeButton=[Windows.Forms.Button]::new()
$bridgeButton.Name='nsre-mcp-start'
$bridgeButton.Text='Fixture MCP'
$bridgeButton.Location=[Drawing.Point]::new(100,180)
$bridgeButton.Size=[Drawing.Size]::new(140,40)
$bridgeButton.Add_Click({$front.Text='NSRE MCP Invoked'})
$front.Controls.Add($bridgeButton)
$text=[Windows.Forms.TextBox]::new()
$text.Name='nsre-text'
$text.Location=[Drawing.Point]::new(20,20)
$text.Size=[Drawing.Size]::new(250,30)
$text.Add_TextChanged({$front.Text='NSRE Typed '+$text.Text})
$front.Controls.Add($text)
$secret=[Windows.Forms.TextBox]::new()
$secret.Name='nsre-password'
$secret.UseSystemPasswordChar=$true
$secret.Text='SYNTHETIC_PASSWORD_DO_NOT_READ'
$secret.Location=[Drawing.Point]::new(20,60)
$front.Controls.Add($secret)
$mouse=[Windows.Forms.ListBox]::new()
$mouse.Name='nsre-mouse-surface'
$mouse.Location=[Drawing.Point]::new(300,20)
$mouse.Size=[Drawing.Size]::new(160,220)
foreach ($n in 1..100) { [void]$mouse.Items.Add('Fixture '+$n) }
$script:dragOrigin=$null
$mouse.Add_MouseDown({param($sender,$e)
 if ($e.Button -eq [Windows.Forms.MouseButtons]::Right) { $front.Text='NSRE Right Clicked' }
 if ($e.Button -eq [Windows.Forms.MouseButtons]::Middle) { $front.Text='NSRE Middle Clicked' }
 if ($e.Button -eq [Windows.Forms.MouseButtons]::Left) { $script:dragOrigin=$e.Location }
})
$mouse.Add_MouseDoubleClick({$front.Text='NSRE Double Clicked'})
$mouse.Add_MouseWheel({$front.Text='NSRE Wheel Observed'})
$mouse.Add_MouseUp({param($sender,$e)
 if ($null -ne $script:dragOrigin -and $e.Button -eq [Windows.Forms.MouseButtons]::Left -and ([Math]::Abs($e.X-$script:dragOrigin.X)+[Math]::Abs($e.Y-$script:dragOrigin.Y)) -gt 40) { $front.Text='NSRE Dragged' }
 $script:dragOrigin=$null
})
$front.Controls.Add($mouse)
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
 # UIA tree and invocation target a specific fixture control, even while covered.
 $controls=Invoke-Desktop -Arguments @('controls',$front.hwnd,$owner)
 if (!$controls.complete -or $controls.values_read) { throw 'Incomplete UIA fixture tree' }
 $start=@($controls.controls|Where-Object automation_id -eq 'nsre-start')
 $secretControl=@($controls.controls|Where-Object automation_id -eq 'nsre-password')
 if ($start.Count -ne 1 -or $secretControl.Count -ne 1 -or !$secretControl[0].is_password -or $null -ne $secretControl[0].name) { throw 'UIA selector/password exclusion failed' }
 if (($controls|ConvertTo-Json -Depth 8) -match 'SYNTHETIC_PASSWORD_DO_NOT_READ') { throw 'Password value exposed' }
 $invoked=Invoke-Desktop -Arguments @('invoke',$front.hwnd,$owner,'nsre-start','--apply')
 $invokeEffect=$false
 for ($i=0;$i -lt 30;$i++) {
  $state=Invoke-Desktop -Arguments @('inspect',$front.hwnd,$owner)
  if ($state.title -eq 'NSRE Desktop Fixture Clicked 1') { $invokeEffect=$true;break }
  Start-Sleep -Milliseconds 100
 }
 if (!$invoked.dispatched -or !$invokeEffect) { throw 'UIA application effect not observed' }
 # Capture a covered window and verify an actual expected client-area colour.
 $capturePath=Join-Path $root 'covered.png'
 $capture=Invoke-Desktop -Arguments @('capture',$back.hwnd,$owner,$capturePath)
 if ($capture.sha256 -ne (Get-FileHash -LiteralPath $capturePath -Algorithm SHA256).Hash.ToLowerInvariant()) { throw 'Capture digest mismatch' }
 Add-Type -AssemblyName System.Drawing
 $image=[Drawing.Bitmap]::new($capturePath)
 try {
  $backNow=Invoke-Desktop -Arguments @('inspect',$back.hwnd,$owner)
  $pixel=$image.GetPixel([int]($backNow.client_x-$backNow.x+20),[int]($backNow.client_y-$backNow.y+200))
  if ($pixel.R -ne 192 -or $pixel.G -ne 64 -or $pixel.B -ne 32) { throw 'Covered-window image content mismatch' }
 } finally { $image.Dispose() }
 # Exercise the actual stdio agent bridge against only this disposable application.
 $bridgeProject=Join-Path $root 'bridge-project'
 New-Item -ItemType Directory -Path (Join-Path $bridgeProject '.northstar/reports') -Force|Out-Null
 New-Item -ItemType Directory -Path (Join-Path $bridgeProject '.northstar/logs') -Force|Out-Null
 $bridgeInfo=[Diagnostics.ProcessStartInfo]::new()
 $bridgeInfo.FileName=$Engine;$bridgeInfo.UseShellExecute=$false
 $bridgeInfo.RedirectStandardInput=$true;$bridgeInfo.RedirectStandardOutput=$true;$bridgeInfo.RedirectStandardError=$true
 $bridgeInfo.StandardInputEncoding=[Text.UTF8Encoding]::new($false)
 $bridgeInfo.StandardOutputEncoding=[Text.UTF8Encoding]::new($false)
 foreach($argument in @('mcp','--project',$bridgeProject,'--desktop-control')){[void]$bridgeInfo.ArgumentList.Add($argument)}
 $bridge=[Diagnostics.Process]::Start($bridgeInfo)
 $bridgeErrors=$bridge.StandardError.ReadToEndAsync()
 function Invoke-Mcp([hashtable]$Request){
  $bridge.StandardInput.WriteLine(($Request|ConvertTo-Json -Depth 12 -Compress));$bridge.StandardInput.Flush()
  $line=$bridge.StandardOutput.ReadLineAsync()
  if(!$line.Wait(25000)){throw 'Desktop MCP response deadline exceeded'}
  if($null -eq $line.Result){throw 'Desktop MCP terminated without response'}
  return ($line.Result|ConvertFrom-Json)
 }
 $initial=Invoke-Mcp @{jsonrpc='2.0';id=1;method='initialize';params=@{protocolVersion='2025-11-25'}}
 if($initial.result.protocolVersion -ne '2025-11-25'){throw 'Desktop MCP initialization failed'}
 $bridge.StandardInput.WriteLine('{"jsonrpc":"2.0","method":"notifications/initialized"}');$bridge.StandardInput.Flush()
 $toolList=Invoke-Mcp @{jsonrpc='2.0';id=2;method='tools/list'}
 if(@($toolList.result.tools|Where-Object name -like 'nsre_desktop_*').Count -ne 2){throw 'Desktop MCP tools not advertised'}
 $bridgeInvoke=Invoke-Mcp @{jsonrpc='2.0';id=3;method='tools/call';params=@{name='nsre_desktop_act';arguments=@{action='invoke';hwnd=$front.hwnd;pid=$fixture.Id;apply=$true;automation_id='nsre-mcp-start'}}}
 if($bridgeInvoke.result.isError){throw 'Desktop MCP invoke returned an error'}
 $bridgeEffect=$false
 for($i=0;$i -lt 20;$i++){$state=Invoke-Desktop -Arguments @('inspect',$front.hwnd,$owner);if($state.title -eq 'NSRE MCP Invoked'){$bridgeEffect=$true;break};Start-Sleep -Milliseconds 50}
 if(!$bridgeEffect){throw 'Desktop MCP application effect missing'}
 $bridgeCapture=Invoke-Mcp @{jsonrpc='2.0';id=4;method='tools/call';params=@{name='nsre_desktop_observe';arguments=@{action='capture';hwnd=$back.hwnd;pid=$fixture.Id}}}
 if($bridgeCapture.result.isError){throw 'Desktop MCP capture returned an error'}
 $bridgeCaptureData=$bridgeCapture.result.content[0].text|ConvertFrom-Json
 if(!$bridgeCaptureData.command_succeeded -or $bridgeCaptureData.result.sha256 -ne (Get-FileHash -LiteralPath $bridgeCaptureData.result.output -Algorithm SHA256).Hash.ToLowerInvariant()){throw 'Desktop MCP capture integrity mismatch'}
 if(@(Get-ChildItem -LiteralPath (Join-Path $bridgeProject '.northstar/reports') -Filter '*.png').Count -ne 1){throw 'Desktop MCP artifact scope mismatch'}
 $imageBlock=@($bridgeCapture.result.content|Where-Object type -eq 'image')
 if($imageBlock.Count -ne 1 -or $imageBlock[0].mimeType -ne 'image/png'){throw 'Desktop MCP image block missing'}
 $imageBytes=[Convert]::FromBase64String($imageBlock[0].data)
 $imageDigest=[Convert]::ToHexString([Security.Cryptography.SHA256]::HashData($imageBytes)).ToLowerInvariant()
 if($imageDigest -ne $bridgeCaptureData.result.sha256){throw 'Desktop MCP image bytes do not match captured artifact'}
 $audit=@(Get-Content -LiteralPath (Join-Path $bridgeProject '.northstar/logs/desktop-actions.jsonl')|ForEach-Object{$_|ConvertFrom-Json})
 if($audit.Count -ne 4 -or @($audit|Where-Object text_logged -ne $false).Count -ne 0){throw 'Desktop MCP audit evidence mismatch'}
 $bridge.StandardInput.Close()
 if(!$bridge.WaitForExit(5000) -or $bridge.ExitCode -ne 0){throw 'Desktop MCP shutdown failed'}
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
   if ($state.title -eq 'NSRE Desktop Fixture Clicked 2') { $clickVerified=$true; break }
   Start-Sleep -Milliseconds 100
  }
  if (!$clickVerified) { throw 'Fixture button effect not observed' }
 }
 $keyboardVerified=$false
 if ($focus.verified) {
  # The synthetic text field is placed at client point 30,30; no arbitrary user control.
  Invoke-Desktop -Arguments @('click',$front.hwnd,$owner,'30','30','--apply')|Out-Null
  Invoke-Desktop -Arguments @('key',$front.hwnd,$owner,'Ctrl+A','--apply')|Out-Null
  $typed=Invoke-Desktop -Arguments @('type',$front.hwnd,$owner,'مرحبا NSRE','--apply')
  for ($i=0;$i -lt 30;$i++) {
   $state=Invoke-Desktop -Arguments @('inspect',$front.hwnd,$owner)
   if ($state.title -eq 'NSRE Typed مرحبا NSRE') { $keyboardVerified=$true;break }
   Start-Sleep -Milliseconds 100
  }
  if (!$typed.dispatched -or !$keyboardVerified) { throw 'Unicode keyboard effect not observed' }
 }
 $rightVerified=$false;$middleVerified=$false;$doubleVerified=$false;$wheelVerified=$false;$dragVerified=$false
 if ($focus.verified) {
  $mouseCases=@(
   @{arguments=@('right-click',$front.hwnd,$owner,'330','90','--apply');title='NSRE Right Clicked';kind='right'},
   @{arguments=@('middle-click',$front.hwnd,$owner,'330','110','--apply');title='NSRE Middle Clicked';kind='middle'},
   @{arguments=@('double-click',$front.hwnd,$owner,'330','140','--apply');title='NSRE Double Clicked';kind='double'}
  )
  foreach ($case in $mouseCases) {
   $sent=Invoke-Desktop -Arguments $case.arguments
   $effect=$false
   for ($i=0;$i -lt 20;$i++) { $state=Invoke-Desktop -Arguments @('inspect',$front.hwnd,$owner);if($state.title -eq $case.title){$effect=$true;break};Start-Sleep -Milliseconds 50 }
   if (!$sent.dispatched -or !$effect) { throw ('Mouse effect missing: '+$case.kind) }
   switch($case.kind){'right'{$rightVerified=$true};'middle'{$middleVerified=$true};'double'{$doubleVerified=$true}}
  }
  Invoke-Desktop -Arguments @('click',$front.hwnd,$owner,'330','60','--apply')|Out-Null
  $wheel=Invoke-Desktop -Arguments @('scroll',$front.hwnd,$owner,'330','160','-120','--apply')
  for ($i=0;$i -lt 20;$i++) { $state=Invoke-Desktop -Arguments @('inspect',$front.hwnd,$owner);if($state.title -eq 'NSRE Wheel Observed'){$wheelVerified=$true;break};Start-Sleep -Milliseconds 50 }
  if (!$wheel.dispatched -or !$wheelVerified) { throw 'Wheel application effect missing' }
  $drag=Invoke-Desktop -Arguments @('drag',$front.hwnd,$owner,'330','100','420','170','--apply')
  for ($i=0;$i -lt 20;$i++) { $state=Invoke-Desktop -Arguments @('inspect',$front.hwnd,$owner);if($state.title -eq 'NSRE Dragged'){$dragVerified=$true;break};Start-Sleep -Milliseconds 50 }
  if (!$drag.dispatched -or !$drag.button_release_inserted -or !$dragVerified) { throw 'Drag application effect/release missing' }
 }
 $captureCapabilities=Invoke-Desktop -Arguments @('capabilities')
 $wgcVerified=$false
 if ($captureCapabilities.wgc_supported) {
  $wgcPath=Join-Path $root 'wgc-covered.png'
  $wgc=Invoke-Desktop -Arguments @('capture-wgc',$back.hwnd,$owner,$wgcPath)
  if ($wgc.method -ne 'Windows Graphics Capture' -or $wgc.sha256 -ne (Get-FileHash -LiteralPath $wgcPath -Algorithm SHA256).Hash.ToLowerInvariant()) { throw 'WGC backend/provenance mismatch' }
  $wgcImage=[Drawing.Bitmap]::new($wgcPath)
  try {
   $sample=$wgcImage.GetPixel(30,230)
   if ($sample.R -ne 192 -or $sample.G -ne 64 -or $sample.B -ne 32) { throw 'WGC covered-window content mismatch' }
   $wgcVerified=$true
  } finally { $wgcImage.Dispose() }
 }
 $passed=$true
 @{schema=1;status='PASS';scope='interactive Windows fixture only; not FreePi or protected content acceptance';os=[Environment]::OSVersion.VersionString;covered_window_found=$true;move_verified=$true;resize_verified=$true;restore_verified=$true;unselected_window_unchanged=$true;invalid_target_rejected=$true;foreground_verified=[bool]$focus.verified;foreground_exit=$focusExit;click_effect_verified=$clickVerified;keyboard_effect_verified=$keyboardVerified;right_click_effect_verified=$rightVerified;middle_click_effect_verified=$middleVerified;double_click_effect_verified=$doubleVerified;wheel_effect_verified=$wheelVerified;drag_effect_verified=$dragVerified;uia_invoke_effect_verified=$invokeEffect;password_values_excluded=$true;mcp_invoke_effect_verified=$bridgeEffect;mcp_capture_integrity_verified=$true;mcp_scoped_audit_verified=$true;covered_capture_content_verified=$true;wgc_supported=[bool]$captureCapabilities.wgc_supported;wgc_capture_content_verified=$wgcVerified;os_is_windows11=([Environment]::OSVersion.Version.Build -ge 22000)}|ConvertTo-Json|Set-Content -LiteralPath (Join-Path $Reports 'desktop-control.json') -Encoding UTF8
} finally {
 if ($null -ne $bridge) {
  if (!$bridge.HasExited) { $bridge.Kill();$bridge.WaitForExit() }
  $bridge.Dispose()
 }
 if ($null -ne $fixture) {
  if (!$fixture.HasExited) { $fixture.Kill(); $fixture.WaitForExit() }
  $fixture.Dispose()
 }
 if (!$passed) {
  @{schema=1;status='FAIL';scope='partial hosted fixture observations; not full acceptance';keyboard_effect_verified=$keyboardVerified;click_effect_verified=$clickVerified;mcp_invoke_effect_verified=$bridgeEffect;right_click_effect_verified=$rightVerified;middle_click_effect_verified=$middleVerified;double_click_effect_verified=$doubleVerified;wheel_effect_verified=$wheelVerified;drag_effect_verified=$dragVerified;wgc_capture_content_verified=$wgcVerified}|ConvertTo-Json|Set-Content -LiteralPath (Join-Path $Reports 'desktop-control.partial.json') -Encoding UTF8
  Write-Warning 'Desktop fixture did not pass; no overall acceptance claim'
 }
 Remove-Item -LiteralPath $root -Recurse -Force
}
