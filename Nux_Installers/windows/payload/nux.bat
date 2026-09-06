@echo off
setlocal enabledelayedexpansion

:: nux — Nux language runner for Windows
:: Automatically selects the correct runtime:
::   1. ovm.nxb  (compiled OVM — fastest, preferred)
::   2. boot.py  (Python bootstrap — first-time only)

set "SCRIPT_DIR=%~dp0"
set "OVM_NXB=%SCRIPT_DIR%lib\ovm\ovm.nxb"
set "BOOTSTRAP=%SCRIPT_DIR%nux_oleg\bootstrap\boot.py"

if "%~1"=="" (
    echo Nux v2.0 - OVM Runtime
    echo Usage: nux ^<file.nux^|file.nxb^>
    echo        nux compile ^<input.nux^> [-o output.nxb]
    echo        nux --status
    exit /b 1
)

:: -- Compile subcommand --
if /I "%~1"=="compile" goto compile
if /I "%~1"=="build" goto compile

:: -- Run subcommand --
set "FILE=%~1"
if not exist "%FILE%" (
    echo nux: file not found: %FILE%
    exit /b 1
)

if exist "%OVM_NXB%" (
    "%OVM_NXB%" "%FILE%"
    exit /b %ERRORLEVEL%
)

if exist "%BOOTSTRAP%" (
    python --version >nul 2>&1
    if errorlevel 1 (
        echo nux: bootstrap requires Python 3 ^(install or compile ovm.nxb first^)
        exit /b 1
    )
    python "%BOOTSTRAP%" "%FILE%"
    exit /b %ERRORLEVEL%
)

echo nux: ERROR - no runtime found.
echo      Expected: %OVM_NXB% OR %BOOTSTRAP%
echo      Run: bonfort build lib\ovm\ovm.nux to compile the OVM
exit /b 1

:compile
set "INPUT=%~2"
if "%~3"=="-o" (
    set "OUTPUT=%~4"
) else (
    set "OUTPUT=%INPUT:.nux=.nxb%"
)

if not exist "%INPUT%" (
    echo nux: file not found: %INPUT%
    exit /b 1
)

if exist "%OVM_NXB%" (
    "%OVM_NXB%" compile "%INPUT%" -o "%OUTPUT%"
    exit /b %ERRORLEVEL%
)

where zstd >nul 2>&1
if not errorlevel 1 (
    zstd -19 -q "%INPUT%" -o "%OUTPUT%"
    echo nux: compiled %INPUT% -^> %OUTPUT% ^(zstd^)
    exit /b 0
)

echo nux: nuxc not found, and no zstd compression tool available.
echo nux: install zstd or compile OVM native binary.
exit /b 1
