@echo off
setlocal

echo [Repairing Nux Installation...]

set "INSTALL_DIR=%ProgramFiles%\Nux"
set "REPO_URL=https://github.com/Nux-Lang/Nux_Windows.git"
set "TEMP_DIR=%TEMP%\nux_repair_%RANDOM%"

:: Check Admin
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo Error: Run as Administrator
    pause
    exit /b 1
)

echo Downloading fresh files from GitHub...
mkdir "%TEMP_DIR%"
git clone --depth 1 "%REPO_URL%" "%TEMP_DIR%"

echo Restoring core files...
call "%TEMP_DIR%\nux_pack_windows_v1.0\setup.bat"

rmdir /s /q "%TEMP_DIR%"

echo Repair complete.
pause
