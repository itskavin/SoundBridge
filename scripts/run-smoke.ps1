$ErrorActionPreference = "Stop"

$cargo = "$env:USERPROFILE\.cargo\bin\cargo.exe"
if (-not (Test-Path $cargo)) {
    throw "cargo not found at $cargo. Install rustup and ensure cargo is available."
}

Write-Host "==> Desktop smoke run (metadata)"
& $cargo run -p sb-desktop-app -- --mode client --server 127.0.0.1:7000 --bind 127.0.0.1:0 --seconds 2

Write-Host "==> Mobile smoke run (metadata + realtime client dry run)"
& $cargo run -p sb-mobile-app -- --server 127.0.0.1:7000 --bind 127.0.0.1:0 --seconds 2

Write-Host "==> Smoke runs completed (for live realtime test, run desktop server and mobile client in separate terminals)"
