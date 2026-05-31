# test_tamper.ps1 - Tests Nux bytecode security checks
$ErrorActionPreference = "Stop"

Write-Host "Compiling test_hello.nux..."
Set-Content -Path test_hello.nux -Value 'func main() { println(42); }'
cargo run --bin nux -- compile test_hello.nux --output test_hello.nuxc

Write-Host "Running untouched file..."
cargo run --bin nux -- run test_hello.nuxc

Write-Host "Tampering with bytecode..."
# Read file as bytes
$bytes = [System.IO.File]::ReadAllBytes("test_hello.nuxc")
# Flip a bit in the beginning
$bytes[4] = $bytes[4] -bxor 0xFF
[System.IO.File]::WriteAllBytes("test_hello_tampered.nuxc", $bytes)

Write-Host "Running tampered file..."
$proc = Start-Process -FilePath "cargo" -ArgumentList "run --bin nux -- run test_hello_tampered.nuxc" -NoNewWindow -PassThru -Wait
if ($proc.ExitCode -eq 1) {
    Write-Host "SUCCESS: VM correctly rejected tampered file!" -ForegroundColor Green
} else {
    Write-Host "FAIL: VM executed tampered file or returned wrong code ($($proc.ExitCode))" -ForegroundColor Red
}
