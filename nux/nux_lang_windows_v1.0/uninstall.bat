@echo off
setlocal

:: Colors
set "RED=[31m"
set "GREEN=[32m"
set "NC=[0m"
set "ESC="

echo [Uninstalling Nux Programming Language...]

set "INSTALL_DIR=%ProgramFiles%\Nux"
set "USER_DIR=%USERPROFILE%\.nux"

:: Check Admin
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo Error: Run as Administrator
    pause
    exit /b 1
)

if exist "%INSTALL_DIR%" (
    rmdir /s /q "%INSTALL_DIR%"
)

if exist "%USER_DIR%" (
    rmdir /s /q "%USER_DIR%"
)

:: Remove from PATH (requires user intervention or external tool like setx carefully)
echo Warning: Please verify your PATH environment variable is clean.
echo Nux has been removed from file system.

echo Done.
pause
