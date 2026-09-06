@echo off
setlocal

:: Bonfort Windows Wrapper
:: Since bonfort is primarily a bash script, this relies on Git Bash, WSL, or MSYS2 being available.

where bash >nul 2>&1
if errorlevel 1 (
    echo [Bonfort Error] Bash is not installed or not in PATH!
    echo Please install Git for Windows ^(Git Bash^) or WSL to run bonfort on Windows natively.
    exit /b 1
)

set "SCRIPT_DIR=%~dp0"
bash "%SCRIPT_DIR%bonfort.sh" %*
exit /b %ERRORLEVEL%
