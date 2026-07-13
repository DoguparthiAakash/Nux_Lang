$ErrorActionPreference = "Stop"

Write-Host "Building Nux Compiler and VM..."
$nux_src = "E:\nux\Nux_Lang\nux\nux_oleg\nux_portable"
cd $nux_src
cargo build --release --bin nux

$nux_bin_dir = "$nux_src\target\release"
$nux_bin = "$nux_bin_dir\nux.exe"

if (Test-Path $nux_bin) {
    Write-Host "Nux built successfully at $nux_bin!"
    
    # Adding to Current Process PATH for immediate use
    $env:PATH = "$nux_bin_dir;" + $env:PATH
    Write-Host "Added Nux to the current session PATH."
    
    # Try to add permanently to User PATH
    try {
        $userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
        if ($userPath -notmatch [regex]::Escape($nux_bin_dir)) {
            $newPath = $userPath + ";$nux_bin_dir"
            [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
            Write-Host "Successfully added Nux permanently to your User PATH!"
        } else {
            Write-Host "Nux is already in your User PATH."
        }
    } catch {
        Write-Host "Failed to add to User PATH automatically. Please add $nux_bin_dir to your System PATH manually."
    }
} else {
    Write-Host "Build failed, nux.exe not found."
}
