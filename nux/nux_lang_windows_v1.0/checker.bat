@echo off
setlocal

echo [Checking Nux Installation Health...]

set "INSTALL_DIR=%ProgramFiles%\Nux"
set "BIN_DIR=%ProgramFiles%\Nux\bin"
set "ERRORS=0"

if exist "%INSTALL_DIR%\bin\nux.exe" (
    echo OK: nux.exe found.
) else (
    echo ALARM: nux.exe MISSING!
    set /a ERRORS=ERRORS+1
)

where nux >nul 2>nul
if %errorLevel% == 0 (
    echo OK: 'nux' command is in PATH.
    nux --version
) else (
    echo ALARM: 'nux' command NOT found in PATH.
    set /a ERRORS=ERRORS+1
)

if %ERRORS% == 0 (
    echo System Health: EXCELLENT
) else (
    echo System Health: BROKEN (%ERRORS% issues found)
    echo Recommended action: Run Repair.
)

pause
