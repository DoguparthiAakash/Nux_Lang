@echo off
setlocal EnableDelayedExpansion
title Nux Health Checker

:: Colors
set "CYAN=[36m"
set "GREEN=[32m"
set "RED=[31m"
set "YELLOW=[33m"
set "WHITE=[37m"
set "NC=[0m"
set "CHECK=✓"
set "CROSS=✗"

cls
echo.
echo %CYAN%    ╔═══════════════════════════════════════════════════════════════════╗%NC%
echo %CYAN%    ║                                                                   ║%NC%
echo %CYAN%    ║    ████     ██████████████      ███╗    ██╗██╗    ██╗██╗    ██╗   ║%NC%
echo %CYAN%    ║    ████     ██████████████      ████╗   ██║██║    ██║╚██╗  ██╔╝   ║%NC%
echo %CYAN%    ║    ████     ████                ██╔██╗  ██║██║    ██║ ╚██╗██╔╝    ║%NC%
echo %CYAN%    ║    ████     ████                ██║╚██╗ ██║██║    ██║  ╚███╔╝     ║%NC%
echo %CYAN%    ║    ██████████████████████       ██║ ╚██╗██║██║    ██║   ███║      ║%NC%
echo %CYAN%    ║    ██████████████████████       ██║  ╚████║██║    ██║  ██╔██╗     ║%NC%
echo %CYAN%    ║             ████     ████       ██║   ╚███║██║    ██║ ██╔╝╚██╗    ║%NC%
echo %CYAN%    ║             ████     ████       ██║    ╚██║██║    ██║██╔╝  ╚██╗   ║%NC%
echo %CYAN%    ║    █████████████     ████       ██║     ╚█║╚██████╔╝██║      ██║  ║%NC%
echo %CYAN%    ║    █████████████     ████       ╚═╝      ╚╝ ╚═════╝ ╚═╝      ╚═╝  ║%NC%
echo %CYAN%    ║                                                                   ║%NC%
echo %CYAN%    ║               %WHITE%Programming Language%CYAN% v1.0.0                          ║%NC%
echo %CYAN%    ║                                                                   ║%NC%
echo %CYAN%    ╚═══════════════════════════════════════════════════════════════════╝%NC%
echo.
echo     %CYAN%Starting System Health Check...%NC%
echo.

set "INSTALL_DIR=%ProgramFiles%\Nux"
set "BIN_DIR=%ProgramFiles%\Nux\bin"
set "ERRORS=0"

echo     %WHITE%Checking Core Binaries:%NC%
if exist "%INSTALL_DIR%\bin\nux.exe" (
    echo     %GREEN%%CHECK%%NC% Found: nux.exe
) else (
    echo     %RED%%CROSS% MISSING:%NC% nux.exe
    set /a ERRORS=ERRORS+1
)

echo.
echo     %WHITE%Checking Environment:%NC%
where nux >nul 2>nul
if %errorLevel% == 0 (
    echo     %GREEN%%CHECK%%NC% 'nux' command is in PATH
    for /f "tokens=*" %%i in ('nux --version 2^>nul') do set VER=%%i
    echo     %CYAN%ℹ%NC%  Detected Version: !VER!
) else (
    echo     %RED%%CROSS% 'nux' command NOT found in PATH%NC%
    set /a ERRORS=ERRORS+1
)

echo.
if %ERRORS% == 0 (
    echo     %GREEN%═══════════════════════════════════════════════════════════════════%NC%
    echo     %GREEN%   %CHECK% System Health: EXCELLENT%NC%
    echo     %GREEN%═══════════════════════════════════════════════════════════════════%NC%
) else (
    echo     %RED%═══════════════════════════════════════════════════════════════════%NC%
    echo     %RED%   %CROSS% System Health: BROKEN (%ERRORS% issues found)%NC%
    echo        Recommended action: Run %YELLOW%setup.bat%NC% and select %YELLOW%Repair%NC%
    echo     %RED%═══════════════════════════════════════════════════════════════════%NC%
)
echo.
pause
