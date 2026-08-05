$ErrorActionPreference = "Continue"

$InstallDir = "$env:LOCALAPPDATA\Nux"

Write-Host "Uninstalling Nux..."

if (Test-Path $InstallDir) {
    Remove-Item -Path $InstallDir -Recurse -Force
    Write-Host "Removed directory $InstallDir"
}

$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -like "*$InstallDir*") {
    $NewPath = ($UserPath -split ';' | Where-Object { $_ -ne $InstallDir }) -join ';'
    [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
    Write-Host "Removed Nux from PATH."
}

# Remove associations
Remove-Item -Path "HKCU:\Software\Classes\.nux" -Recurse -Force
Remove-Item -Path "HKCU:\Software\Classes\Nux.File" -Recurse -Force

Write-Host "Uninstallation complete."
