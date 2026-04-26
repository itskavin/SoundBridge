param(
    [string]$Server = "127.0.0.1:7000",
    [string]$Bind = "127.0.0.1:0",
    [int]$Seconds = 20
)

$ErrorActionPreference = "Stop"

$cargo = "$env:USERPROFILE\.cargo\bin\cargo.exe"
if (-not (Test-Path $cargo)) {
    throw "cargo not found at $cargo. Install rustup and ensure cargo is available."
}

Write-Host "==> Starting mobile realtime client to $Server for $Seconds seconds"
& $cargo run -p sb-mobile-app -- --server $Server --bind $Bind --seconds $Seconds
