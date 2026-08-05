$ErrorActionPreference = "Stop"

# ============================================================================
#  Nux Programming Language — Windows Installer (PowerShell)
#  Security-hardened with integrity checks and Rust-inspired TUI
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

# --- Banner ---
Write-Header "nux-installer" "interactive menu ..."

# --- TLS 1.2+ enforcement (prevents downgrade attacks) ---
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12 -bor [Net.SecurityProtocolType]::Tls13

# --- Menu ---
$IsInstalled = Test-Path "$InstallDir\nux.exe"
Write-Host "  Select an option:" -ForegroundColor White
Write-Host ""
Write-Host "    1)  Install Now        " -NoNewline -ForegroundColor Green
Write-Host "(Recommended)" -ForegroundColor Cyan
Write-Host "    2)  Custom Install      Choose location and features" -ForegroundColor Green
if ($IsInstalled) {
    Write-Host "    3)  Repair              Reinstall all files" -ForegroundColor Green
    Write-Host "    4)  Update              Fetch latest version" -ForegroundColor Green
    Write-Host "    5)  Uninstall           Remove Nux completely" -ForegroundColor Red
}
Write-Host "    0)  Exit" -ForegroundColor Green
Write-Host ""
$Choice = Read-Host "  Enter choice [1]"
if ([string]::IsNullOrEmpty($Choice)) { $Choice = "1" }

function Verify-Checksum {
    param([string]$FilePath, [string]$ExpectedHash)
    if ([string]::IsNullOrEmpty($ExpectedHash)) { return $true }
    $ActualHash = (Get-FileHash -Path $FilePath -Algorithm SHA256).Hash
    if ($ActualHash -ne $ExpectedHash) {
        Write-NuxError "SHA-256 checksum mismatch!"
        Write-NuxError "Expected: $ExpectedHash"
        Write-NuxError "Got:      $ActualHash"
        Write-NuxError "The download may be corrupted or tampered with."
        return $false
    }
    Write-Nux "Verified" "SHA-256 checksum OK" "Green"
    return $true
}

function Do-Install {
    param([string]$TargetDir = $InstallDir, [bool]$AddToPath = $true, [bool]$FileAssoc = $true, [bool]$StdLib = $true)

    Write-Nux "Compiling" "installation plan..." "Magenta"

    # --- Create directory (validate path is not a symlink hijack) ---
    if (Test-Path $TargetDir) {
        $item = Get-Item $TargetDir
        if ($item.Attributes -band [IO.FileAttributes]::ReparsePoint) {
            Write-NuxError "Install directory is a symlink. Aborting for security."
            exit 1
        }
    }
    if (!(Test-Path $TargetDir)) {
        New-Item -ItemType Directory -Force -Path $TargetDir | Out-Null
    }

    # --- Download with TLS + checksum ---
    Write-Nux "Downloading" "nux-windows.zip..." "Cyan"
    $TempZip = [System.IO.Path]::Combine([System.IO.Path]::GetTempPath(), "nux-windows-$([guid]::NewGuid()).zip")
    try {
        Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempZip -UseBasicParsing
    } catch {
        Write-NuxWarn "Download unavailable: $_"
        Write-NuxWarn "If you have a local build, copy it manually to $TargetDir"
        return
    }

    # --- Verify checksum ---
    Write-Nux "Verifying" "download integrity..." "Cyan"
    $ExpectedHash = $null
    try {
        $ExpectedHash = (Invoke-WebRequest -Uri $ChecksumUrl -UseBasicParsing).Content.Trim().Split(" ")[0]
    } catch {
        Write-NuxWarn "Checksum file unavailable. Skipping integrity check."
    }
    if ($ExpectedHash) {
        if (!(Verify-Checksum -FilePath $TempZip -ExpectedHash $ExpectedHash)) {
            Remove-Item $TempZip -Force -ErrorAction SilentlyContinue
            Write-NuxError "Installation aborted due to integrity failure."
            exit 1
        }
    }

    # --- Extract ---
    Write-Nux "Extracting" "files to $TargetDir..." "Cyan"
    Expand-Archive -Path $TempZip -DestinationPath $TargetDir -Force
    Remove-Item $TempZip -Force -ErrorAction SilentlyContinue

    # --- Verify extracted binary exists ---
    if (!(Test-Path "$TargetDir\nux.exe")) {
        Write-NuxError "nux.exe not found after extraction. Archive may be malformed."
        exit 1
    }

    # --- PATH (user scope only, no elevation needed) ---
    if ($AddToPath) {
        Write-Nux "Linking" "PATH environment..." "Cyan"
        $UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
        if ($UserPath -notlike "*$TargetDir*") {
            $Sep = if ($UserPath.EndsWith(";")) { "" } else { ";" }
            [Environment]::SetEnvironmentVariable("PATH", "$UserPath$Sep$TargetDir", "User")
        }
        [Environment]::SetEnvironmentVariable("NUX_HOME", $TargetDir, "User")
        [Environment]::SetEnvironmentVariable("NUX_LIB_PATH", "$TargetDir\lib", "User")
    }

    # --- File Associations (HKCU only, no admin) ---
    if ($FileAssoc) {
        Write-Nux "Linking" ".nux file associations..." "Cyan"
        New-Item -Path "HKCU:\Software\Classes\.nux" -Force | Out-Null
        New-ItemProperty -Path "HKCU:\Software\Classes\.nux" -Name "(default)" -Value "Nux.File" -Force | Out-Null
        New-Item -Path "HKCU:\Software\Classes\Nux.File\shell\open\command" -Force | Out-Null
        New-ItemProperty -Path "HKCU:\Software\Classes\Nux.File\shell\open\command" -Name "(default)" -Value "`"$TargetDir\nux.exe`" run `"%1`"" -Force | Out-Null
    }

    # --- Success Banner ---
    Write-NuxFinish "installed" "successfully to $TargetDir"
    Write-Host "         You can now use the " -NoNewline -ForegroundColor DarkGray
    Write-Host "nux" -NoNewline -ForegroundColor Cyan
    Write-Host " command from any terminal." -ForegroundColor DarkGray
    Write-Host "         Try: " -NoNewline -ForegroundColor DarkGray
    Write-Host "nux run hello.nux" -ForegroundColor Cyan
    Write-Host ""
}

function Do-CustomInstall {
    $dir = Read-Host "  Install directory [$InstallDir]"
    if ([string]::IsNullOrEmpty($dir)) { $dir = $InstallDir }

    # Sanitize path — reject suspicious characters
    if ($dir -match '[<>|"&;`$]') {
        Write-NuxError "Invalid characters in path. Aborting."
        exit 1
    }

    $addPath = Read-Host "  Add to PATH? [Y/n]"
    $addPath = if ($addPath -eq "n" -or $addPath -eq "N") { $false } else { $true }

    $fileAssoc = Read-Host "  File associations? [Y/n]"
    $fileAssoc = if ($fileAssoc -eq "n" -or $fileAssoc -eq "N") { $false } else { $true }

    Do-Install -TargetDir $dir -AddToPath $addPath -FileAssoc $fileAssoc
}

function Do-Update {
    Write-Nux "Updating" "Nux to latest version..." "Cyan"
    if (!(Test-Path "$InstallDir\nux.exe")) {
        Write-NuxError "Nux is not installed. Run install first."
        return
    }
    Do-Install -TargetDir $InstallDir
}

function Do-Uninstall {
    Write-Host ""
    Write-Host "  WARNING: This will completely remove Nux." -ForegroundColor Red
    $confirm = Read-Host "  Are you sure? [y/N]"
    if ($confirm -ne "y" -and $confirm -ne "Y") {
        Write-Host "  Uninstall cancelled." -ForegroundColor DarkGray
        return
    }

    Write-Nux "Removing" "Nux files..." "Red"
    if (Test-Path $InstallDir) {
        Remove-Item -Path $InstallDir -Recurse -Force
    }

    # Clean PATH
    $UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    if ($UserPath -like "*$InstallDir*") {
        $NewPath = ($UserPath -split ';' | Where-Object { $_ -ne $InstallDir -and $_ -ne "" }) -join ';'
        [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
    }
    [Environment]::SetEnvironmentVariable("NUX_HOME", $null, "User")
    [Environment]::SetEnvironmentVariable("NUX_LIB_PATH", $null, "User")

    # Clean registry
    Remove-Item -Path "HKCU:\Software\Classes\.nux" -Recurse -Force -ErrorAction SilentlyContinue
    Remove-Item -Path "HKCU:\Software\Classes\Nux.File" -Recurse -Force -ErrorAction SilentlyContinue

    Write-Host ""
    Write-Host "       ✔ " -NoNewline -ForegroundColor Green
    Write-Host "Nux has been completely removed." -ForegroundColor White
    Write-Host ""
}

# --- Dispatch ---
switch ($Choice) {
    "1" { Do-Install }
    "2" { Do-CustomInstall }
    "3" { if ($IsInstalled) { Do-Install } else { Write-NuxError "Not installed." } }
    "4" { if ($IsInstalled) { Do-Update } else { Write-NuxError "Not installed." } }
    "5" { if ($IsInstalled) { Do-Uninstall } else { Write-NuxError "Not installed." } }
    "0" { Write-Host "  Exiting." -ForegroundColor DarkGray; exit 0 }
    default { Write-NuxError "Invalid choice."; exit 1 }
}
