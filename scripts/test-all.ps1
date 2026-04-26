$ErrorActionPreference = "Stop"

$cargo = "$env:USERPROFILE\.cargo\bin\cargo.exe"
if (-not (Test-Path $cargo)) {
    throw "cargo not found at $cargo. Install rustup and ensure cargo is available."
}

Write-Host "==> Formatting workspace"
& $cargo fmt --all

Write-Host "==> Building workspace"
& $cargo build --workspace

Write-Host "==> Running tests"
& $cargo test --workspace

Write-Host "==> Running desktop smoke test"
& $cargo run -p sb-desktop-app

Write-Host "==> Running mobile smoke test"
& $cargo run -p sb-mobile-app -- 127.0.0.1:7000

Write-Host "==> All checks passed"
