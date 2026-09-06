param (
    [switch]$Update,
    [switch]$Uninstall
)

$ErrorActionPreference = "Stop"

# ============================================================================
#  Nux Programming Language — Windows Installer (Version Manager)
# ============================================================================

$Version = "1.1.0"
$InstallDirBase = "$env:LOCALAPPDATA\Nux"
$InstallDir = "$InstallDirBase\$Version"
$CurrentJunction = "$InstallDirBase\current"
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

function Install-Nux {
    param([string]$TargetDir = $InstallDir, [bool]$AddToPath = $true)
    if (!(Test-Path $TargetDir)) { New-Item -ItemType Directory -Force -Path $TargetDir | Out-Null }
    
    if (Test-Path "payload.zip") {
        Write-Nux "Extracting" "local payload.zip..." "Cyan"
        Get-ChildItem $TargetDir -Recurse | Remove-Item -Force -Recurse
        Expand-Archive -Path "payload.zip" -DestinationPath $TargetDir -Force
    } else {
        $TempZip = [System.IO.Path]::Combine([System.IO.Path]::GetTempPath(), "nux-$([guid]::NewGuid()).zip")
        Write-Nux "Downloading" "nux-windows.zip..." "Cyan"
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

        
        Get-ChildItem $TargetDir -Recurse | Remove-Item -Force -Recurse
        Expand-Archive -Path $TempZip -DestinationPath $TargetDir -Force
        Remove-Item $TempZip -Force
    }
    Remove-Item $TempZip -Force
    
    if (Test-Path $CurrentJunction) { 
        cmd /c rmdir "$CurrentJunction" 2>$null
    }
    New-Item -ItemType Junction -Path $CurrentJunction -Target $TargetDir | Out-Null
    
    if ($AddToPath) {
        $UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
        if ($UserPath -notlike "*$CurrentJunction*") { [Environment]::SetEnvironmentVariable("PATH", "$UserPath;$CurrentJunction", "User") }
    }
    Write-NuxFinish "installed" "v$Version successfully"
}

function Update-Nux {
    Write-Nux "Updating" "Nux to latest version..." "Magenta"
    Install-Nux
}

function Uninstall-Nux {
    if (!(Test-Path $InstallDirBase)) {
        Write-NuxError "Nux is not installed."
        exit 1
    }
    
    $Versions = @(Get-ChildItem -Path $InstallDirBase -Directory | Where-Object { $_.Name -ne 'current' })
    if ($Versions.Count -eq 0) {
        Write-NuxError "No Nux versions found."
        exit 1
    }
    
    Write-Host "  Installed Versions:"
    
    $ActiveTarget = ""
    if (Test-Path $CurrentJunction) {
        # Get target of junction. In PS5.1, Get-Item doesn't directly expose .Target. We can use fsutil or parse dir
        $ActiveTarget = cmd /c dir "$InstallDirBase" | Select-String "current \["
        if ($ActiveTarget -match "\[(.*)\]") {
            $ActiveTarget = $matches[1]
        }
    }

    for ($i = 0; $i -lt $Versions.Count; $i++) {
        $vname = $Versions[$i].Name
        $marker = ""
        if ($ActiveTarget -like "*\$vname") { $marker = " (active)" }
        Write-Host "    $($i + 1))  $vname$marker" -ForegroundColor Yellow
    }
    Write-Host "    0)  Uninstall ALL versions" -ForegroundColor Red
    Write-Host ""
    $choiceStr = Read-Host "  Select versions to uninstall (comma-separated, e.g. 1,3 or 0 for all)"
    
    if ($choiceStr -eq "0") {
        Write-Nux "Removing" "all Nux files..." "Red"
        if (Test-Path $CurrentJunction) { cmd /c rmdir "$CurrentJunction" 2>$null }
        Remove-Item -Path $InstallDirBase -Recurse -Force -ErrorAction SilentlyContinue
        
        $UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
        $NewPath = ($UserPath -split ';' | Where-Object { $_ -ne $CurrentJunction -and $_ -ne "" }) -join ';'
        [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
        
        Write-NuxFinish "uninstalled" "Nux completely."
        return
    }
    
    $choices = $choiceStr -split ',' | ForEach-Object { $_.Trim() }
    foreach ($c in $choices) {
        $idx = [int]$c - 1
        if ($idx -ge 0 -and $idx -lt $Versions.Count) {
            $vToRemove = $Versions[$idx].Name
            $dirToRemove = $Versions[$idx].FullName
            Write-Nux "Removing" "v$vToRemove..." "Red"
            
            if ($ActiveTarget -like "*\$vToRemove") {
                cmd /c rmdir "$CurrentJunction" 2>$null
            }
            Remove-Item -Path $dirToRemove -Recurse -Force -ErrorAction SilentlyContinue
        }
    }
    
    if (!(Test-Path $CurrentJunction)) {
        $Remaining = @(Get-ChildItem -Path $InstallDirBase -Directory | Where-Object { $_.Name -ne 'current' })
        if ($Remaining.Count -gt 0) {
            $Latest = $Remaining[$Remaining.Count - 1]
            New-Item -ItemType Junction -Path $CurrentJunction -Target $Latest.FullName | Out-Null
            Write-Nux "Switched" "active version to $($Latest.Name)" "Green"
        } else {
            $UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
            $NewPath = ($UserPath -split ';' | Where-Object { $_ -ne $CurrentJunction -and $_ -ne "" }) -join ';'
            [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
        }
    }
    
    Write-NuxFinish "uninstalled" "Selected versions removed."
}

function Show-Menu {
    Write-Host "  Select an option:"
    Write-Host "    1) Install  2) Update  3) Uninstall  0) Exit"
    $global:Choice = Read-Host "  Choice"
}

[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12 -bor [Net.SecurityProtocolType]::Tls13

if ($Update) { Install-Nux; exit 0 }
if ($Uninstall) { Uninstall-Nux; exit 0 }

Write-Header "nux-installer" "interactive menu ..."
Show-Menu

switch ($Choice) {
    "1" { Install-Nux }
    "2" { Update-Nux }
    "3" { Uninstall-Nux }
    "0" { exit 0 }
    default { Write-NuxError "Invalid choice."; exit 1 }
}
