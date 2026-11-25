# Compatibility entry point. Complete evidence is now required; use Package-Release.ps1.
param([Parameter(Mandatory=$true)][string]$Output,[Parameter(Mandatory=$true)][string]$Reports)
& (Join-Path $PSScriptRoot 'Package-Release.ps1') -Output $Output -Reports $Reports
