$ErrorActionPreference = "Stop"

$WIX_BIN = "C:\Users\dogup\AppData\Local\tauri\WixTools314"
$CANDLE = "$WIX_BIN\candle.exe"
$LIGHT = "$WIX_BIN\light.exe"
$HEAT = "$WIX_BIN\heat.exe"

$SRC_DIR = "E:\nux\Nux_Lang"
$DIST_DIR = "$SRC_DIR\nux\nux_oleg\nux_dist\target\release"

# 1. Copy nux.exe
Write-Host "Copying nux.exe..."
Copy-Item "$DIST_DIR\nux.exe" -Destination ".\nux.exe" -Force

# 2. Harvest stdlib using heat
Write-Host "Harvesting std directory..."
& $HEAT dir "$SRC_DIR\lib\std" -cg StdComponents -dr STDDIR -gg -scom -sreg -sfrag -srd -var var.StdSource -out std.wxs

# 3. Compile WXS files
Write-Host "Compiling WXS..."
& $CANDLE -arch x64 nux.wxs std.wxs -dStdSource="$SRC_DIR\lib\std" -ext WixUtilExtension

# 4. Link into MSI
Write-Host "Linking MSI..."
& $LIGHT -sval -out nux.msi nux.wixobj std.wixobj -ext WixUtilExtension

Write-Host "MSI Built Successfully: nux.msi"

# 5. Compile C++ UI with MSVC
Write-Host "Compiling C++ Setup UI..."
$VCVARS = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
$COMPILE_CMD = @"
call `"$VCVARS`"
rc.exe resources.rc
cl.exe /O2 /EHsc /MD main.cpp resources.res user32.lib gdi32.lib shell32.lib comctl32.lib /FeNuxSetup.exe
"@

Set-Content -Path "compile_ui.bat" -Value $COMPILE_CMD
& cmd /c "compile_ui.bat"

Write-Host "Installer built successfully: NuxSetup.exe"
