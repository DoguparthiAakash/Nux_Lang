$ErrorActionPreference = "Stop"

# URLs
$DownloadUrl = "https://github.com/DoguparthiAakash/Nux_Installers/releases/latest/download/nux-windows.zip"
$InnoUrl = "https://files.jrsoftware.org/is/6/innosetup-6.3.3-portable.zip"

$ScriptDir = $PSScriptRoot
$PayloadDir = Join-Path $ScriptDir "payload"

Write-Host ">>> Downloading Nux payload from Github..." -ForegroundColor Cyan
if (Test-Path $PayloadDir) { Remove-Item -Recurse -Force $PayloadDir }
New-Item -ItemType Directory -Force $PayloadDir | Out-Null

try {
    $TempZip = Join-Path $ScriptDir "nux_payload.zip"
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempZip -UseBasicParsing
    Expand-Archive -Path $TempZip -DestinationPath $PayloadDir -Force
    Remove-Item $TempZip -Force
} catch {
    Write-Host ">>> Could not download payload (Release missing?). Mocking payload from local repo..." -ForegroundColor Yellow
    Copy-Item -Path "e:\nux\Nux_Lang\nux\*" -Destination $PayloadDir -Recurse -Force
    Get-ChildItem -Path $PayloadDir -Filter "target*" -Recurse -Directory -ErrorAction SilentlyContinue | Remove-Item -Recurse -Force
}

Write-Host ">>> Checking for Inno Setup compiler..." -ForegroundColor Cyan
$ISCC = "${env:ProgramFiles(x86)}\Inno Setup 6\ISCC.exe"
if (!(Test-Path $ISCC)) {
    Write-Host ">>> Inno Setup not found. Installing via Winget..." -ForegroundColor Yellow
    winget install -e --id JRSoftware.InnoSetup --accept-package-agreements --accept-source-agreements
}

Write-Host ">>> Compiling Nux_Setup.exe..." -ForegroundColor Magenta
$IssFile = Join-Path $ScriptDir "nux_setup.iss"

& $ISCC $IssFile

Write-Host ">>> Done! Nux_Setup.exe is generated in Output/ folder." -ForegroundColor Green
