$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
if (Test-Path "$ScriptDir\install.ps1") {
    & "$ScriptDir\install.ps1" -Update
} else {
    Write-Host "Installer not found at $ScriptDir\install.ps1" -ForegroundColor Red
    exit 1
}
