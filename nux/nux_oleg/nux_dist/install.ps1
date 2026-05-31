# install.ps1 - Nux Language Installer
# Installs Nux to %LOCALAPPDATA%\Nux without requiring Administrator privileges

$ErrorActionPreference = "Stop"

Write-Host "====================================" -ForegroundColor Cyan
Write-Host " Nux Programming Language Installer" -ForegroundColor Cyan
Write-Host "====================================" -ForegroundColor Cyan

# Set working directory to the location of the script
Push-Location $PSScriptRoot

# 1. Build the project
Write-Host "`n[1/6] Building Nux engine (Release)..." -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "Failed to build Nux!" -ForegroundColor Red
    exit 1
}

# 2. Remove older versions
Write-Host "`n[2/6] Cleaning up old versions..." -ForegroundColor Yellow
$installDir = Join-Path $env:LOCALAPPDATA "Nux"
if (Test-Path $installDir) {
    Remove-Item "$installDir\*" -Recurse -Force -ErrorAction SilentlyContinue
}

# 3. Setup Directories
Write-Host "`n[3/6] Setting up installation directories..." -ForegroundColor Yellow
$libDir = Join-Path $installDir "lib"

if (-not (Test-Path $installDir)) { New-Item -ItemType Directory -Path $installDir -Force | Out-Null }
if (-not (Test-Path $libDir)) { New-Item -ItemType Directory -Path $libDir -Force | Out-Null }

# Copy files
Write-Host "      Copying nux.exe..."
Copy-Item ".\target\release\nux.exe" -Destination $installDir -Force
Write-Host "      Copying standard libraries..."
Copy-Item -Path ".\lib\*" -Destination $libDir -Recurse -Force -ErrorAction SilentlyContinue
Write-Host "      Copying icons..."
if (Test-Path ".\nux_file_icon.ico") { Copy-Item ".\nux_file_icon.ico" -Destination $installDir -Force }
if (Test-Path ".\nuxc_file_icon.ico") { Copy-Item ".\nuxc_file_icon.ico" -Destination $installDir -Force }
if (Test-Path ".\logo.png") { Copy-Item ".\logo.png" -Destination $installDir -Force }

# 4. Update User PATH
Write-Host "`n[4/6] Updating PATH environment variable..." -ForegroundColor Yellow
$userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($userPath -notlike "*$installDir*") {
    $newPath = "$userPath;$installDir"
    [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
    Write-Host "      Added $installDir to User PATH."
} else {
    Write-Host "      PATH already contains Nux directory."
}

# 5. File Associations & Icons
Write-Host "`n[5/6] Setting up File Associations..." -ForegroundColor Yellow
$nuxIcon = Join-Path $installDir "nux_file_icon.ico"
$nuxcIcon = Join-Path $installDir "nuxc_file_icon.ico"

# We associate .nux with a ProgID "Nux.SourceFile"
New-Item -Path "HKCU:\Software\Classes\.nux" -Force | Out-Null
Set-ItemProperty -Path "HKCU:\Software\Classes\.nux" -Name "(default)" -Value "Nux.SourceFile"

# We associate .nuxc with a ProgID "Nux.CompiledFile"
New-Item -Path "HKCU:\Software\Classes\.nuxc" -Force | Out-Null
Set-ItemProperty -Path "HKCU:\Software\Classes\.nuxc" -Name "(default)" -Value "Nux.CompiledFile"

# Create the ProgID class for .nux
New-Item -Path "HKCU:\Software\Classes\Nux.SourceFile" -Force | Out-Null
Set-ItemProperty -Path "HKCU:\Software\Classes\Nux.SourceFile" -Name "(default)" -Value "Nux Source File"
New-Item -Path "HKCU:\Software\Classes\Nux.SourceFile\DefaultIcon" -Force | Out-Null
Set-ItemProperty -Path "HKCU:\Software\Classes\Nux.SourceFile\DefaultIcon" -Name "(default)" -Value "$nuxIcon"
New-Item -Path "HKCU:\Software\Classes\Nux.SourceFile\shell\open\command" -Force | Out-Null
Set-ItemProperty -Path "HKCU:\Software\Classes\Nux.SourceFile\shell\open\command" -Name "(default)" -Value "`"$installDir\nux.exe`" run `"%1`""

# Create the ProgID class for .nuxc
New-Item -Path "HKCU:\Software\Classes\Nux.CompiledFile" -Force | Out-Null
Set-ItemProperty -Path "HKCU:\Software\Classes\Nux.CompiledFile" -Name "(default)" -Value "Nux Compiled File"
New-Item -Path "HKCU:\Software\Classes\Nux.CompiledFile\DefaultIcon" -Force | Out-Null
Set-ItemProperty -Path "HKCU:\Software\Classes\Nux.CompiledFile\DefaultIcon" -Name "(default)" -Value "$nuxcIcon"
New-Item -Path "HKCU:\Software\Classes\Nux.CompiledFile\shell\open\command" -Force | Out-Null
Set-ItemProperty -Path "HKCU:\Software\Classes\Nux.CompiledFile\shell\open\command" -Name "(default)" -Value "`"$installDir\nux.exe`" run `"%1`""

# 6. VSCode Extension
Write-Host "`n[6/6] Setting up VSCode Extension..." -ForegroundColor Yellow
$vscodeExtDir = Join-Path $env:USERPROFILE ".vscode\extensions\nux-language"
$srcExtDir = "..\vscode_extension"
if (Test-Path $srcExtDir) {
    if (Test-Path $vscodeExtDir) {
        Remove-Item "$vscodeExtDir\*" -Recurse -Force -ErrorAction SilentlyContinue
    } else {
        New-Item -ItemType Directory -Path $vscodeExtDir -Force | Out-Null
    }
    Copy-Item -Path "$srcExtDir\*" -Destination $vscodeExtDir -Recurse -Force
    Write-Host "      Installed VSCode extension successfully."
} else {
    Write-Host "      VSCode extension source not found. Skipping."
}

# Notify Windows Explorer of the file association changes
$code = @"
using System;
using System.Runtime.InteropServices;
public class Win32 {
    [DllImport("shell32.dll")]
    public static extern void SHChangeNotify(int wEventId, int uFlags, IntPtr dwItem1, IntPtr dwItem2);
}
"@
Add-Type -TypeDefinition $code
[Win32]::SHChangeNotify(0x08000000, 0x0000, [IntPtr]::Zero, [IntPtr]::Zero)

Write-Host "`n====================================" -ForegroundColor Green
Write-Host " Installation Successful!" -ForegroundColor Green
Write-Host " You can now use the 'nux' command." -ForegroundColor Green
Write-Host " (Note: You may need to restart your terminal for PATH changes to take effect)" -ForegroundColor Green
Write-Host "====================================" -ForegroundColor Green

Pop-Location
