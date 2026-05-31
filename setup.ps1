$ErrorActionPreference = "Stop"

Write-Host "Building Nux Compiler and VM..."
cd E:\nux\Nux_Lang\nux\nux_oleg\nux_dist
cargo build --release --bin nux

$nux_bin = "E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\target\release\nux.exe"

if (Test-Path $nux_bin) {
    Write-Host "Nux built successfully!"
    
    # Adding to Current Process PATH for immediate use
    $env:PATH = "E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\target\release;" + $env:PATH
    Write-Host "Added Nux to the current session PATH."
    Write-Host "To add it globally, you can add E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\target\release to your System PATH."
} else {
    Write-Host "Build failed, nux.exe not found."
}
