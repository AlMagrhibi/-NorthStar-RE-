@echo off
setlocal
powershell.exe -NoLogo -NoProfile -File "%~dp0install\Manage.ps1" -Action Doctor %*
exit /b %errorlevel%
