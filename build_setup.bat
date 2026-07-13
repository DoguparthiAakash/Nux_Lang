@echo off
echo === Nux Full Installer Builder ===
echo.

set CSC="C:\Windows\Microsoft.NET\Framework64\v4.0.30319\csc.exe"
set NUX_SRC="E:\nux\Nux_Lang\nux\nux_oleg\nux_portable"
set NUX_BIN="E:\nux\Nux_Lang\nux\nux_oleg\nux_portable\target\release\nux.exe"
set ZIP_SCRIPT="E:\nux\Nux_Lang\make_zip.ps1"
set SOURCE_ZIP="E:\nux\Nux_Lang\Nux_Source.zip"
set INSTALLER_CS="E:\nux\Nux_Lang\installer.cs"
set UNINSTALLER_CS="E:\nux\Nux_Lang\uninstaller.cs"
set OUTPUT_EXE="E:\nux\Nux_Lang\NuxSetup_Full.exe"
set UNINSTALL_EXE="E:\nux\Nux_Lang\NuxUninstall.exe"

:: Step 1 - Check nux.exe binary exists
if not exist %NUX_BIN% (
    echo [Step 1/4] Building Nux compiler from source...
    pushd %NUX_SRC%
    cargo build --release --bin nux
    if %ERRORLEVEL% neq 0 (
        echo ERROR: cargo build failed!
        popd
        exit /b 1
    )
    popd
) else (
    echo [Step 1/4] Found pre-built nux.exe - skipping cargo build.
)

:: Step 2 - Compile standalone NuxUninstall.exe first
echo [Step 2/4] Compiling NuxUninstall.exe...
%CSC% /nologo /target:winexe /out:%UNINSTALL_EXE% %UNINSTALLER_CS%
if %ERRORLEVEL% neq 0 (
    echo ERROR: Failed to compile NuxUninstall.exe!
    exit /b 1
)
echo     Done. NuxUninstall.exe ready.

:: Step 3 - Create the distributable zip
echo [Step 3/4] Packaging Nux runtime into Nux_Source.zip...
powershell -ExecutionPolicy Bypass -File %ZIP_SCRIPT%
if %ERRORLEVEL% neq 0 (
    echo ERROR: Failed to create Nux_Source.zip!
    exit /b 1
)

:: Step 4 - Compile the GUI installer embedding BOTH the zip AND the uninstaller
echo [Step 4/4] Compiling NuxSetup_Full.exe (with embedded uninstaller)...
%CSC% /nologo /target:winexe /out:%OUTPUT_EXE% ^
  /res:%SOURCE_ZIP%,Nux_Source.zip ^
  /res:%UNINSTALL_EXE%,NuxUninstall.exe ^
  /reference:System.IO.Compression.dll ^
  /reference:System.IO.Compression.FileSystem.dll ^
  %INSTALLER_CS%

if %ERRORLEVEL% equ 0 (
    echo.
    echo === SUCCESS ===
    echo.
    echo NuxSetup_Full.exe  ^(Shareable full installer^):
    dir /b "E:\nux\Nux_Lang\NuxSetup_Full.exe"
    echo NuxUninstall.exe   ^(Standalone uninstaller^):
    dir /b "E:\nux\Nux_Lang\NuxUninstall.exe"
    echo.
    echo Both files are ready in E:\nux\Nux_Lang\
) else (
    echo.
    echo ERROR: Compilation failed. See output above for details.
    exit /b 1
)
