# V1 Launch Checklist (No Managed Relay)

## Product Model

- Native apps (desktop/mobile) handle realtime sender/receiver roles
- Optional web peer mode for browser users
- No managed backend control plane for media flow

## Pre-Launch Verification

1. Run native validation:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/test-all.ps1
```

2. Run realtime two-terminal check:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/realtime-server.ps1 -Bind 0.0.0.0:7000 -Seconds 60
powershell -ExecutionPolicy Bypass -File scripts/realtime-client.ps1 -Server 127.0.0.1:7000 -Seconds 20
```

3. Verify web mode manually:

- Open two browser sessions of web app
- Broadcaster creates offer from mic
- Listener creates answer and starts receiving audio
- Broadcaster applies answer and connection reaches connected state

## Online Web Launch

1. Publish static folder `apps/web/public` to host
2. Enable HTTPS
3. Verify microphone permission prompt is working
4. Validate P2P connection and audio in real browser devices

## Release Notes Minimum

- Supported OS list for native apps
- Web mode constraints (manual signaling, peer-to-peer)
- Known limitations and troubleshooting notes

## Privacy Promise

- No server-side audio relay in v1 architecture
- Clients directly exchange realtime media paths
