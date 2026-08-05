$ErrorActionPreference = "Stop"

$InstallDir = "$env:LOCALAPPDATA\Nux"
$DownloadUrl = "https://github.com/DoguparthiAakash/Nux_Installers/releases/latest/download/nux-windows.zip"

Write-Host "Updating Nux..."
if (!(Test-Path $InstallDir)) {
    Write-Host "Nux is not installed at $InstallDir. Please run install.ps1 instead."
    exit 1
}

$TempZip = "$env:TEMP\nux-windows-update.zip"
try {
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempZip
    Expand-Archive -Path $TempZip -DestinationPath $InstallDir -Force
    Write-Host "Update successful!"
} catch {
    Write-Host "Failed to download or extract update from $DownloadUrl." -ForegroundColor Red
}
