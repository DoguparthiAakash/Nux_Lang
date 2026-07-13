# make_zip.ps1
# Packages the Nux runtime into a distributable zip archive (Nux_Source.zip)
# Excludes heavy build artifacts (target/, .git/, __pycache__, etc.)

$rootDir = "E:\nux\Nux_Lang"
$outputZip = "E:\nux\Nux_Lang\Nux_Source.zip"
$tempDir = "E:\nux\Nux_Lang\_zip_staging"

# Clean old staging
if (Test-Path $tempDir) { Remove-Item $tempDir -Recurse -Force }
if (Test-Path $outputZip) { Remove-Item $outputZip -Force }

New-Item -ItemType Directory -Path $tempDir | Out-Null

# Define what to include. We copy the nux_portable/src, Cargo files,
# and the pre-built release binary so the target machine doesn't need cargo.
$includePaths = @(
    "nux\nux_oleg\nux_portable\src",
    "nux\nux_oleg\nux_portable\Cargo.toml",
    "nux\nux_oleg\nux_portable\Cargo.lock"
)

# Also include the pre-compiled binary
$binSrc = "nux\nux_oleg\nux_portable\target\release\nux.exe"

foreach ($rel in $includePaths) {
    $src = Join-Path $rootDir $rel
    $dst = Join-Path $tempDir $rel
    if (Test-Path $src -PathType Container) {
        # It's a directory — copy recursively
        Copy-Item -Path $src -Destination $dst -Recurse -Force -ErrorAction SilentlyContinue
    } elseif (Test-Path $src -PathType Leaf) {
        $dstParent = Split-Path $dst
        if (-not (Test-Path $dstParent)) { New-Item -ItemType Directory -Path $dstParent | Out-Null }
        Copy-Item -Path $src -Destination $dst -Force -ErrorAction SilentlyContinue
    } else {
        Write-Host "Skipping (not found): $rel"
    }
}

# Copy the pre-built binary to a top-level bin folder for easy PATH use
$binDst = Join-Path $tempDir "bin\nux.exe"
$binSrcFull = Join-Path $rootDir $binSrc
if (Test-Path $binSrcFull) {
    New-Item -ItemType Directory -Path (Split-Path $binDst) | Out-Null
    Copy-Item -Path $binSrcFull -Destination $binDst -Force
    Write-Host "Included nux.exe binary."
} else {
    Write-Warning "nux.exe not found! Run 'cargo build --release' first."
    Remove-Item $tempDir -Recurse -Force
    exit 1
}

# Write a README
$readme = @"
Nux Language Runtime
=====================
Version: 0.1.0-alpha
Architecture: x86_64-windows

Installation managed by NuxSetup_Full.exe.
The 'nux' executable is located in the 'bin' folder.
Add the 'bin' folder to your PATH to use 'nux' globally.

Usage:
  nux run myprogram.nux
  nux build myprogram.nux

For documentation, visit: https://github.com/DoguparthiAakash/Nux_Lang
"@
$readme | Out-File -FilePath (Join-Path $tempDir "README.txt") -Encoding utf8

# Compress everything
Add-Type -AssemblyName System.IO.Compression.FileSystem
[System.IO.Compression.ZipFile]::CreateFromDirectory($tempDir, $outputZip)

# Cleanup
Remove-Item $tempDir -Recurse -Force

$size = (Get-Item $outputZip).Length / 1KB
Write-Host ""
Write-Host "=== Done! ==="
Write-Host "Created: $outputZip"
Write-Host ("Size: {0:N1} KB" -f $size)
