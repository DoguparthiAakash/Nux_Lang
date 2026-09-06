param (
    [string]$TargetDir = ""
)

$ErrorActionPreference = "Stop"

# ============================================================================
#  Nux Programming Language — Portable Windows Installer
# ============================================================================

$Version = "1.1.0"
$DownloadUrl = "https://github.com/DoguparthiAakash/Nux_Installers/releases/latest/download/nux-windows.zip"
$ChecksumUrl = "https://github.com/DoguparthiAakash/Nux_Installers/releases/latest/download/nux-windows.sha256"

function Write-Header {
    param([string]$Title, [string]$Subtitle)
    Write-Host ""
    Write-Host "╭─ " -NoNewline -ForegroundColor DarkGray
    Write-Host "◆ " -NoNewline -ForegroundColor Cyan
    Write-Host "$Title " -NoNewline -ForegroundColor White
    Write-Host "─────────────────────────────────" -ForegroundColor DarkGray
    Write-Host "│  " -NoNewline -ForegroundColor DarkGray
    Write-Host "Nux v$Version  " -NoNewline -ForegroundColor White
    Write-Host "·  " -NoNewline -ForegroundColor DarkGray
    Write-Host "$Subtitle" -ForegroundColor DarkGray
    Write-Host "╰────────────────────────────────────────" -ForegroundColor DarkGray
}

function Write-Nux { param([string]$Tag, [string]$Msg, [string]$Color = "Green")
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

function Verify-Checksum {
    param([string]$FilePath, [string]$ExpectedHash)
    if ([string]::IsNullOrEmpty($ExpectedHash)) { return $true }
    $ActualHash = (Get-FileHash -Path $FilePath -Algorithm SHA256).Hash
    if ($ActualHash -ne $ExpectedHash) {
        Write-NuxError "SHA-256 checksum mismatch!"
        return $false
    }
    Write-Nux "Verified" "SHA-256 checksum OK" "Green"
    return $true
}

[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12 -bor [Net.SecurityProtocolType]::Tls13

Write-Header "nux-portable-installer" "extracts locally, no PATH changes"

if ([string]::IsNullOrEmpty($TargetDir)) {
    $TargetDir = Join-Path $PWD "nux-v$Version"
    $Choice = Read-Host "  Install portable Nux to $TargetDir? [Y/n]"
    if ($Choice -and $Choice.ToLower() -eq 'n') {
        Write-Host "  Installation aborted."
        exit 0
    }
}

if (!(Test-Path $TargetDir)) { New-Item -ItemType Directory -Force -Path $TargetDir | Out-Null }

if (Test-Path "payload.zip") {
    Write-Nux "Extracting" "local payload.zip..." "Cyan"
    Get-ChildItem $TargetDir -Recurse | Remove-Item -Force -Recurse
    Expand-Archive -Path "payload.zip" -DestinationPath $TargetDir -Force
} else {
    $TempZip = [System.IO.Path]::Combine([System.IO.Path]::GetTempPath(), "nux-$([guid]::NewGuid()).zip")
    Write-Nux "Downloading" "Nux from Github..." "Cyan"
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempZip -UseBasicParsing
    
    try {
        $TempHash = [System.IO.Path]::Combine([System.IO.Path]::GetTempPath(), "nux-$([guid]::NewGuid()).sha256")
        Invoke-WebRequest -Uri $ChecksumUrl -OutFile $TempHash -UseBasicParsing
        $ExpectedHash = ((Get-Content $TempHash) -split '\s+')[0]
        Remove-Item $TempHash -Force
        if (-not (Verify-Checksum $TempZip $ExpectedHash)) {
            Remove-Item $TempZip -Force
            Write-NuxError "Installation aborted due to checksum mismatch."
            exit 1
        }
    } catch {
        Write-Nux "Warning" "Could not download checksum file. Skipping verification." "Yellow"
    }
    
    Write-Nux "Extracting" "files to $TargetDir..." "Cyan"
    Get-ChildItem $TargetDir -Recurse | Remove-Item -Force -Recurse
    Expand-Archive -Path $TempZip -DestinationPath $TargetDir -Force
    Remove-Item $TempZip -Force
}

# Create a convenient runner batch file
$BatPath = Join-Path $TargetDir "nux-cli.bat"
Set-Content -Path $BatPath -Value "@echo off`nset NUX_HOME=%~dp0`nset PATH=%~dp0;%PATH%`ncmd.exe /K `"echo Nux Portable Environment activated! Type 'nux' to start.`""

Write-NuxFinish "installed" "Portable environment ready. Run nux-cli.bat to start using Nux."
