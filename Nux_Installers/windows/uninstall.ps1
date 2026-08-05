$ErrorActionPreference = "Continue"

# ============================================================================
#  Nux — Uninstaller (Windows)
#  Clean removal with Rust-inspired TUI
# ============================================================================

$InstallDir = "$env:LOCALAPPDATA\Nux"

function Write-Header {
    param([string]$Title, [string]$Subtitle)
    Write-Host ""
    Write-Host "╭─ " -NoNewline -ForegroundColor DarkGray
    Write-Host "◆ " -NoNewline -ForegroundColor Cyan
    Write-Host "$Title " -NoNewline -ForegroundColor White
    Write-Host "───────────────────────────────" -ForegroundColor DarkGray
    Write-Host "│  " -NoNewline -ForegroundColor DarkGray
    Write-Host "Nux  " -NoNewline -ForegroundColor White
    Write-Host "·  " -NoNewline -ForegroundColor DarkGray
    Write-Host "$Subtitle" -ForegroundColor DarkGray
    Write-Host "╰────────────────────────────────────────" -ForegroundColor DarkGray
}

function Write-Nux { param([string]$Tag, [string]$Msg, [string]$Color = "Red")
    Write-Host "├─ " -NoNewline -ForegroundColor DarkGray
    Write-Host "✦ " -NoNewline -ForegroundColor Green
    Write-Host "$Tag  " -NoNewline -ForegroundColor $Color
    Write-Host "$Msg" -ForegroundColor Gray
}

function Write-NuxError { param([string]$Msg)
    Write-Host "╰─ " -NoNewline -ForegroundColor DarkGray
    Write-Host "✕ " -NoNewline -ForegroundColor Red
    Write-Host "error  " -NoNewline -ForegroundColor Red
    Write-Host "$Msg" -ForegroundColor White
}
function Write-NuxFinish { param([string]$Tag, [string]$Msg)
    Write-Host "╰─ " -NoNewline -ForegroundColor DarkGray
    Write-Host "▶ " -NoNewline -ForegroundColor Cyan
    Write-Host "$Tag  " -NoNewline -ForegroundColor Cyan
    Write-Host "$Msg" -ForegroundColor White
}

if (!(Test-Path $InstallDir)) {
    Write-NuxError "Nux is not installed at $InstallDir."
    exit 1
}

Write-Header "nux-uninstall" "removing installation ..."

Write-Host "  WARNING: This will completely remove Nux from your system." -ForegroundColor Red
$confirm = Read-Host "  Are you sure? [y/N]"
if ($confirm -ne "y" -and $confirm -ne "Y") {
    Write-Host "  Uninstall cancelled." -ForegroundColor DarkGray
    exit 0
}

# Remove files
Write-Nux "Removing" "files..." "Red"
if (Test-Path $InstallDir) {
    Remove-Item -Path $InstallDir -Recurse -Force
}

# Clean PATH
Write-Nux "Cleaning" "environment..." "Red"
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -like "*$InstallDir*") {
    $NewPath = ($UserPath -split ';' | Where-Object { $_ -ne $InstallDir -and $_ -ne "" }) -join ';'
    [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
}
[Environment]::SetEnvironmentVariable("NUX_HOME", $null, "User")
[Environment]::SetEnvironmentVariable("NUX_LIB_PATH", $null, "User")

# Clean registry
Write-Nux "Cleaning" "registry..." "Red"
Remove-Item -Path "HKCU:\Software\Classes\.nux" -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item -Path "HKCU:\Software\Classes\Nux.File" -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item -Path "HKCU:\Software\Classes\.nuxc" -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item -Path "HKCU:\Software\Classes\Nux.CompiledFile" -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item -Path "HKCU:\Software\Nux" -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\Nux" -Recurse -Force -ErrorAction SilentlyContinue

Write-NuxFinish "uninstalled" "completely"
