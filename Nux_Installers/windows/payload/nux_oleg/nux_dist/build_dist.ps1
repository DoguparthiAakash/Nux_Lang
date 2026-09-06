$ErrorActionPreference = "Stop"

Write-Host "Building Nux release version..."
cargo build --release

$DistDir = "nux_release"
if (Test-Path $DistDir) {
    Remove-Item -Recurse -Force $DistDir
}
New-Item -ItemType Directory -Path $DistDir

Write-Host "Copying executable..."
Copy-Item "target\release\nux.exe" -Destination $DistDir

Write-Host "Copying standard libraries..."
Copy-Item "lib" -Destination $DistDir -Recurse

Write-Host "Creating user install script..."
$InstallScript = @"
`$ErrorActionPreference = "Stop"
`$InstallDir = "`$env:LOCALAPPDATA\Nux"

Write-Host "Installing Nux to `$InstallDir..."
if (-not (Test-Path `$InstallDir)) {
    New-Item -ItemType Directory -Path `$InstallDir
}

Copy-Item ".\nux.exe" -Destination `$InstallDir -Force
Copy-Item ".\lib" -Destination `$InstallDir -Recurse -Force

Write-Host "Setting environment variables..."
[System.Environment]::SetEnvironmentVariable("NUX_LIB_PATH", "`$InstallDir\lib", "User")

`$UserPath = [System.Environment]::GetEnvironmentVariable("PATH", "User")
if (`$UserPath -notlike "*`$InstallDir*") {
    [System.Environment]::SetEnvironmentVariable("PATH", "`$UserPath;`$InstallDir", "User")
}

Write-Host "Registering .nux file extension..."
try {
    New-Item -Path "HKCU:\Software\Classes\.nux" -Force -ErrorAction SilentlyContinue | Out-Null
    Set-ItemProperty -Path "HKCU:\Software\Classes\.nux" -Name "(Default)" -Value "Nux.Script" -ErrorAction SilentlyContinue

    New-Item -Path "HKCU:\Software\Classes\Nux.Script\shell\open\command" -Force -ErrorAction SilentlyContinue | Out-Null
    Set-ItemProperty -Path "HKCU:\Software\Classes\Nux.Script\shell\open\command" -Name "(Default)" -Value "`"`$InstallDir\nux.exe`" run `"`%1`"" -ErrorAction SilentlyContinue
} catch {
    Write-Host "Note: Could not register file extension. You may need to run this script as Administrator." -ForegroundColor Yellow
}

Write-Host "Installation complete! You may need to restart your terminal to use the 'nux' command." -ForegroundColor Green
"@

Set-Content -Path "$DistDir\install.ps1" -Value $InstallScript

Write-Host "Creating zip archive..."
if (Test-Path "nux_release.zip") {
    Remove-Item "nux_release.zip"
}
Compress-Archive -Path "$DistDir\*" -DestinationPath "nux_release.zip"

Write-Host "Release package created at nux_release.zip" -ForegroundColor Green
