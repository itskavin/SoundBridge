param(
    [string]$Bind = "0.0.0.0:7000",
    [int]$Seconds = 60
)

$ErrorActionPreference = "Stop"

$cargo = "$env:USERPROFILE\.cargo\bin\cargo.exe"
if (-not (Test-Path $cargo)) {
    throw "cargo not found at $cargo. Install rustup and ensure cargo is available."
}

Write-Host "==> Starting desktop realtime server on $Bind for $Seconds seconds"
& $cargo run -p sb-desktop-app -- --mode server --bind $Bind --seconds $Seconds
