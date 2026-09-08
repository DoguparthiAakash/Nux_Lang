# build_office_windows.ps1
# Script to build and package Tulasi Office Apps for Windows

$ErrorActionPreference = "Stop"

$ScriptDir = $PSScriptRoot
$OutputDir = Join-Path $ScriptDir "Tulasi_Office_Windows"
$NuxInterpreter = "nux.exe" # Assuming Nux is in PATH or bundled

Write-Host ">>> Cleaning output directory..." -ForegroundColor Cyan
if (Test-Path $OutputDir) { Remove-Item -Recurse -Force $OutputDir }
New-Item -ItemType Directory -Force $OutputDir | Out-Null

$Apps = @("Tulasi_Lekhana", "Tulasi_Pattika", "Tulasi_Darshana")
$NuxSrcDir = "e:\nux"

foreach ($app in $Apps) {
    Write-Host ">>> Packaging $app..." -ForegroundColor Magenta
    
    # Create App Directory
    $AppOut = Join-Path $OutputDir $app
    New-Item -ItemType Directory -Force $AppOut | Out-Null
    
    # Copy Nux Source Files
    $AppSrc = Join-Path $NuxSrcDir $app
    if (Test-Path $AppSrc) {
        Copy-Item -Path "$AppSrc\*" -Destination $AppOut -Recurse -Force
    } else {
        Write-Host "Warning: Source path $AppSrc not found!" -ForegroundColor Yellow
    }

    # Generate Windows Batch Wrapper
    $BatPath = Join-Path $AppOut "$app.bat"
    $BatContent = "@echo off`r`nTITLE $app`r`n$NuxInterpreter run main.nux`r`npause"
    Set-Content -Path $BatPath -Value $BatContent

    Write-Host "    Created wrapper: $BatPath" -ForegroundColor Green
}

Write-Host ">>> Tulasi Office Suite packaging complete! Available in $OutputDir" -ForegroundColor Green
