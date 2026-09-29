$ErrorActionPreference = "Stop"

$ScriptDir  = "E:\nux\Nux_Lang\Nux_Installers\windows"
$RepoDir    = "E:\nux\Nux_Lang\nux\nux_oleg\nux_dist"
$PayloadDir = Join-Path $ScriptDir "payload"
$BinDest    = Join-Path $PayloadDir "bin"
$VsxSrc     = "E:\nux\Nux_Lang\nux\nux_oleg\vscode_extension\nux-lang-0.0.2.vsix"
$LibSrc     = "E:\nux\Nux_Lang\lib"
$ISCC       = "C:\Program Files (x86)\Inno Setup 6\ISCC.exe"
$IssFile    = Join-Path $ScriptDir "nux_setup.iss"

# Extract version from Cargo.toml
$CargoToml = Join-Path $RepoDir "Cargo.toml"
$Version = "0.1.0"
if (Test-Path $CargoToml) {
    $Lines = Get-Content $CargoToml
    foreach ($Line in $Lines) {
        if ($Line -match '^version\s*=\s*"([^"]+)"') {
            $Version = $Matches[1]
            break
        }
    }
}
Write-Host "Detected Nux version: $Version" -ForegroundColor Cyan

# ── Step 1: Build release binary ──────────────────────────────────────────────
Write-Host "`n>>> Building nux release binary..." -ForegroundColor Cyan
Push-Location $RepoDir
cargo build --bin nux --release
Pop-Location

$NuxExe = Join-Path $RepoDir "target\release\nux.exe"
if (!(Test-Path $NuxExe)) { throw "Build failed: nux.exe not found at $NuxExe" }
Write-Host ">>> nux.exe built: $NuxExe" -ForegroundColor Green

# ── Step 2: Assemble payload ───────────────────────────────────────────────────
Write-Host "`n>>> Assembling payload..." -ForegroundColor Cyan
if (!(Test-Path $PayloadDir)) { New-Item -ItemType Directory -Force $PayloadDir | Out-Null }
if (!(Test-Path $BinDest))    { New-Item -ItemType Directory -Force $BinDest    | Out-Null }

# Copy the nux binary
Copy-Item -Path $NuxExe -Destination (Join-Path $BinDest "nux.exe") -Force
Write-Host "  Copied nux.exe -> payload\bin\nux.exe"

# Copy standard library
if (Test-Path $LibSrc) {
    $LibDest = Join-Path $PayloadDir "lib"
    if (Test-Path $LibDest) { Remove-Item -Recurse -Force $LibDest }
    Copy-Item -Path $LibSrc -Destination $LibDest -Recurse -Force
    Write-Host "  Copied lib -> payload\lib"
}

# Copy VS Code extension
if (Test-Path $VsxSrc) {
    Copy-Item -Path $VsxSrc -Destination (Join-Path $PayloadDir "nux-lang.vsix") -Force
    Write-Host "  Copied .vsix -> payload\nux-lang.vsix"
}

# ── Step 3: Zip Payload ────────────────────────────────────────────────────────
Write-Host "`n>>> Zipping payload..." -ForegroundColor Cyan
$PayloadZip = Join-Path $ScriptDir "payload.zip"
if (Test-Path $PayloadZip) { Remove-Item -Force $PayloadZip }
Compress-Archive -Path "$PayloadDir\*" -DestinationPath $PayloadZip -CompressionLevel Optimal

# ── Step 4: Compile Custom Rust Installer ───────────────────────────────────────
Write-Host "`n>>> Compiling Nux Custom Installer via Cargo..." -ForegroundColor Magenta
$InstallerDir = Join-Path $ScriptDir "..\nux_installer"
Push-Location $InstallerDir
cargo build --release
if ($LASTEXITCODE -ne 0) { throw "Cargo build failed for nux_installer with exit code $LASTEXITCODE" }
Pop-Location

$InstallerExe = Join-Path $InstallerDir "target\release\nux_installer.exe"
$OutputExe = Join-Path $ScriptDir "Output\Nux_Setup_v$Version.exe"

if (!(Test-Path (Join-Path $ScriptDir "Output"))) {
    New-Item -ItemType Directory -Force (Join-Path $ScriptDir "Output") | Out-Null
}

Write-Host "`n>>> Assembling Final Installer..." -ForegroundColor Cyan
cmd.exe /c "copy /b `"$InstallerExe`" + `"$PayloadZip`" `"$OutputExe`""

if (Test-Path $OutputExe) {
    $Size = [math]::Round((Get-Item $OutputExe).Length / 1MB, 2)
    Write-Host "`n>>> SUCCESS! Installer ($Size MB) -> $OutputExe" -ForegroundColor Green
} else {
    Write-Host "`n>>> Installer build failed." -ForegroundColor Red
}
