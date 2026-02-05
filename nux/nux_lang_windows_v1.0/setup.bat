@echo off
setlocal EnableDelayedExpansion
title Nux Setup Manager

:: ╔══════════════════════════════════════════════════════════════╗
:: ║                        COLORS & STYLES                        ║
:: ╚══════════════════════════════════════════════════════════════╝
for /f "tokens=3" %%v in ('reg query "HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion" /v CurrentBuildNumber 2^>nul') do set BUILD=%%v
if defined BUILD (
    if %BUILD% GEQ 10586 (
        set "ESC= "
        set "CYAN=[36m"
        set "GREEN=[32m"
        set "RED=[31m"
        set "YELLOW=[33m"
        set "WHITE=[37m"
        set "NC=[0m"
    ) else (
        set "ESC="
        set "CYAN="
        set "GREEN="
        set "RED="
        set "YELLOW="
        set "WHITE="
        set "NC="
    )
)

:menu
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

:: Check if installed
where nux >nul 2>nul
if %errorLevel% neq 0 (
    echo %YELLOW%Nux is not installed on this system.%NC%
    echo.
    set /p "choice=Do you want to install Nux now? [Y/n] "
    if "!choice!"=="" set choice=Y
    if /i "!choice!"=="Y" (
        call "%~dp0install.bat"
        goto :eof
    ) else (
        echo Installation aborted.
        goto :eof
    )
) else (
    echo %GREEN%✓ Nux is currently installed.%NC%
    echo.
    echo What would you like to do?
    echo.
    echo    [1] %YELLOW%Repair%NC% (Check for missing files & restore)
    echo    [2] %RED%Uninstall%NC% (Remove Nux completely)
    echo    [3] %CYAN%Check Health%NC% (Verify installation status)
    echo    [4] %WHITE%Version Manager%NC% (Switch versions)
    echo    [5] Exit
    echo.
    set /p "opt=Select validation option [1-5]: "
    
    if "!opt!"=="1" call "%~dp0repair.bat"
    if "!opt!"=="2" call "%~dp0uninstall.bat"
    if "!opt!"=="3" call "%~dp0checker.bat"
    if "!opt!"=="4" call "%~dp0version.bat"
    if "!opt!"=="5" goto :eof
)

pause
