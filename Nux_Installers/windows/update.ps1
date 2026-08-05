$ErrorActionPreference = "Stop"

# ============================================================================
#  Nux — Update Manager (Windows)
#  Security-hardened with TLS 1.2+, checksum verification
# ============================================================================

$Version = "1.0.0"
$InstallDir = "$env:LOCALAPPDATA\Nux"
$DownloadUrl = "https://github.com/DoguparthiAakash/Nux_Installers/releases/latest/download/nux-windows.zip"
$ChecksumUrl = "https://github.com/DoguparthiAakash/Nux_Installers/releases/latest/download/nux-windows.sha256"

# --- TUI Colors (Rust/Nux-inspired) ---
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
function Write-NuxWarn { param([string]$Msg)
    Write-Host "├─ " -NoNewline -ForegroundColor DarkGray
    Write-Host "⚠ " -NoNewline -ForegroundColor Yellow
    Write-Host "warning  " -NoNewline -ForegroundColor Yellow
    Write-Host "$Msg" -ForegroundColor Gray
}
function Write-NuxFinish { param([string]$Tag, [string]$Msg)
    Write-Host "╰─ " -NoNewline -ForegroundColor DarkGray
    Write-Host "▶ " -NoNewline -ForegroundColor Cyan
    Write-Host "$Tag  " -NoNewline -ForegroundColor Cyan
    Write-Host "$Msg" -ForegroundColor White
}

[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12 -bor [Net.SecurityProtocolType]::Tls13

if (!(Test-Path "$InstallDir\nux.exe")) {
    Write-NuxError "Nux is not installed at $InstallDir. Run install.ps1 first."
    exit 1
}

Write-Header "nux-update" "fetching latest version ..."

# Download
$TempZip = [System.IO.Path]::Combine([System.IO.Path]::GetTempPath(), "nux-update-$([guid]::NewGuid()).zip")
try {
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempZip -UseBasicParsing
} catch {
    Write-NuxError "Failed to download update: $_"
    exit 1
}

# Verify checksum
Write-Nux "Verifying" "download integrity..." "Cyan"
try {
    $ExpectedHash = (Invoke-WebRequest -Uri $ChecksumUrl -UseBasicParsing).Content.Trim().Split(" ")[0]
    $ActualHash = (Get-FileHash -Path $TempZip -Algorithm SHA256).Hash
    if ($ActualHash -ne $ExpectedHash) {
        Write-NuxError "Checksum mismatch! Update aborted for security."
        Remove-Item $TempZip -Force -ErrorAction SilentlyContinue
        exit 1
    }
    Write-Nux "Verified" "SHA-256 OK" "Green"
} catch {
    Write-NuxWarn "Checksum unavailable, proceeding..."
}

# Extract
Write-Nux "Extracting" "update files..." "Cyan"
Expand-Archive -Path $TempZip -DestinationPath $InstallDir -Force
Remove-Item $TempZip -Force -ErrorAction SilentlyContinue

Write-NuxFinish "updated" "successfully"
