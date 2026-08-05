$ErrorActionPreference = "Stop"

$InstallDir = "$env:LOCALAPPDATA\Nux"
$DownloadUrl = "https://github.com/DoguparthiAakash/Nux_Installers/releases/latest/download/nux-windows.zip" # Placeholder URL

Write-Host "===================================="
Write-Host " Nux Programming Language Installer "
Write-Host "===================================="

if (!(Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

# Example of downloading and extracting
Write-Host "[1/3] Downloading latest Nux binaries..."
$TempZip = "$env:TEMP\nux-windows.zip"
try {
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempZip
    Write-Host "[2/3] Extracting files..."
    Expand-Archive -Path $TempZip -DestinationPath $InstallDir -Force
} catch {
    Write-Host "Error downloading/extracting from $DownloadUrl. If running locally, please place nux.exe and lib/ in $InstallDir manually." -ForegroundColor Yellow
}

Write-Host "[3/3] Updating PATH environment variable..."
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -notlike "*$InstallDir*") {
    $NewPath = if ($UserPath.EndsWith(";")) { "$UserPath$InstallDir" } else { "$UserPath;$InstallDir" }
    [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
    Write-Host "Added $InstallDir to User PATH."
} else {
    Write-Host "PATH already contains Nux directory."
}

# Setup basic file association (requires admin for system-wide, so doing HKCU)
New-Item -Path "HKCU:\Software\Classes\.nux" -Force | Out-Null
New-ItemProperty -Path "HKCU:\Software\Classes\.nux" -Name "(default)" -Value "Nux.File" -Force | Out-Null
New-Item -Path "HKCU:\Software\Classes\Nux.File\shell\open\command" -Force | Out-Null
New-ItemProperty -Path "HKCU:\Software\Classes\Nux.File\shell\open\command" -Name "(default)" -Value "`"$InstallDir\nux.exe`" run `"%1`"" -Force | Out-Null

Write-Host "Installation Complete! Please restart your terminal."
