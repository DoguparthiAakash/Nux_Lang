@echo off
setlocal EnableDelayedExpansion
title Uninstall Nux

:: Colors
set "CYAN=[36m"
set "GREEN=[32m"
set "RED=[31m"
set "YELLOW=[33m"
set "WHITE=[37m"
set "NC=[0m"

cls
echo.
echo %RED%    ╔═══════════════════════════════════════════════════════════════════╗%NC%
echo %RED%    ║                  %WHITE%UNINSTALL NUX PROGRAMMING LANGUAGE%RED%                 ║%NC%
echo %RED%    ╚═══════════════════════════════════════════════════════════════════╝%NC%
echo.

set "INSTALL_DIR=%ProgramFiles%\Nux"
set "USER_DIR=%USERPROFILE%\.nux"

:: Check Admin
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo     %RED%Error: Run as Administrator%NC%
    pause
    exit /b 1
)

echo     %YELLOW%This will completely remove Nux from your system.%NC%
set /p "confirm=    Are you sure you want to continue? [y/N] "
if /i not "%confirm%"=="Y" (
    echo     %GREEN%Aborted.%NC%
    exit /b 0
)
echo.

echo     %CYAN%Removing files...%NC%
if exist "%INSTALL_DIR%" (
    rmdir /s /q "%INSTALL_DIR%"
    echo     %GREEN%✓%NC% Removed installation directory
)

if exist "%USER_DIR%" (
    rmdir /s /q "%USER_DIR%"
    echo     %GREEN%✓%NC% Removed user data
)

echo.
echo     %GREEN%Successfully uninstalled Nux. We're sad to see you go!%NC%
echo.
pause
