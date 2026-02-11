@echo off
setlocal EnableDelayedExpansion
title Nux Repair Tool

:: Colors
set "CYAN=[36m"
set "GREEN=[32m"
set "RED=[31m"
set "YELLOW=[33m"
set "WHITE=[37m"
set "NC=[0m"
set "WRENCH=🔧"
set "ARROW=➜"

cls
echo.
echo %CYAN%    ╔═══════════════════════════════════════════════════════════════════╗%NC%
echo %CYAN%    ║               %WHITE%Nux Installation Repair Tool%CYAN%                            ║%NC%
echo %CYAN%    ╚═══════════════════════════════════════════════════════════════════╝%NC%
echo.

set "INSTALL_DIR=%ProgramFiles%\Nux"
set "REPO_URL=https://github.com/Nux-Lang/Nux_Windows.git"
set "TEMP_DIR=%TEMP%\nux_repair_%RANDOM%"

:: Check Admin
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo     %RED%Error: Run as Administrator%NC%
    pause
    exit /b 1
)

echo     %YELLOW%%WRENCH% Beginning Repair Process...%NC%
echo.

echo     %CYAN%%ARROW%%NC% Downloading fresh files from GitHub...
mkdir "%TEMP_DIR%"
git clone --no-checkout --depth 1 --filter=blob:none "%REPO_URL%" "%TEMP_DIR%" >nul 2>&1
cd /d "%TEMP_DIR%"
git sparse-checkout init --cone >nul 2>&1
git sparse-checkout set nux_pack_windows_v1.0 >nul 2>&1
git checkout >nul 2>&1

if %errorlevel% neq 0 (
    echo     %RED%✗%NC% Download failed
    cd /d "%~dp0"
    rmdir /s /q "%TEMP_DIR%"
    exit /b 1
) else (
    echo     %GREEN%✓%NC% Download complete
    cd /d "%~dp0"
)

echo     %CYAN%%ARROW%%NC% Restoring core files...
:: Patch to prevent recursion loop
echo set NUX_INSTALLER_RUNNING=1 > "%TEMP_DIR%\patch.bat"
type "%TEMP_DIR%\patch.bat" "%TEMP_DIR%\nux_pack_windows_v1.0\setup.bat" > "%TEMP_DIR%\nux_pack_windows_v1.0\setup_patched.bat"
del "%TEMP_DIR%\patch.bat"

call "%TEMP_DIR%\nux_pack_windows_v1.0\setup_patched.bat"

rmdir /s /q "%TEMP_DIR%"

echo.
echo     %GREEN%✓ Repair complete. Your installation has been restored.%NC%
echo.
pause
