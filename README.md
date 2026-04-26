# SoundBridge OSS

SoundBridge OSS is a web-first, open-source audio relay alternative focused on direct peer-to-peer browser audio.

Current strategy:

- Primary product is web.
- Native app development is paused for v1 milestones.
- Media transport is client-controlled P2P (WebRTC).
- No managed backend audio relay in v1.

## Why this architecture

- Faster iteration and shipping speed.
- Instant user onboarding (open URL, no install).
- Better transparency: users control offer/answer and direct peer path.
- Lower operational complexity for v1.

## What is working now

- Next.js web app is running and production-buildable.
- Device name based onboarding is implemented.
- Automatic same-network peer discovery hint is implemented.
- One-click broadcaster flow is implemented.
- Receiver audio playback in browser is implemented.

## Repository map

- [apps/web/app/page.js](apps/web/app/page.js): web UI and onboarding flow
- [apps/web/app/api/signal/route.js](apps/web/app/api/signal/route.js): lightweight signaling and peer list API
- [apps/web/app/api/network/route.js](apps/web/app/api/network/route.js): network hint API
- [apps/web/app/globals.css](apps/web/app/globals.css): web styling
- [apps/web/app/lib/signalStore.js](apps/web/app/lib/signalStore.js): in-memory signal store
- [apps/web/README.md](apps/web/README.md): web-specific notes
- [docs/architecture/WEB-COMPATIBILITY.md](docs/architecture/WEB-COMPATIBILITY.md): browser and OS constraints
- [docs/architecture/V1-LAUNCH-CHECKLIST.md](docs/architecture/V1-LAUNCH-CHECKLIST.md): release checklist

## Quick local run

1. Open terminal in repo root.
2. Run the Next.js app:

```powershell
cd apps/web
npm install
npm run dev
```

3. Open [http://localhost:5173](http://localhost:5173).
4. Open a second device/browser on the same network.

## User onboarding flow

1. Set a clear device name on each device.
2. Wait for auto-discovered peers to appear.
3. Select the target peer.
4. Click Start Call as Broadcaster.
5. Accept microphone permission.
6. Receiver hears stream in browser.

## Same-WiFi auto discovery behavior

- Discovery uses same public network hint (request IP) to prioritize nearby peers.
- This avoids manual offer/answer copy-paste for the default flow.
- Media remains direct peer-to-peer.

## Web behavior and compatibility

Cross-OS support is generally good on modern browsers across Windows, macOS, Linux, Android, and iOS, with caveats:

- Safari/iOS has stricter media/autoplay/background rules.
- Some corporate or restricted NAT/firewall environments may fail pure P2P setup.
- HTTPS is required in production for stable media permissions.

Detailed notes: [docs/architecture/WEB-COMPATIBILITY.md](docs/architecture/WEB-COMPATIBILITY.md).

## Cloudflare deployment

For your requested model (no controlled media relay), deploy the web app over HTTPS.

1. Deploy this Next.js app with a Cloudflare-compatible workflow.
2. Keep signaling lightweight and never relay audio through backend.
3. Enable HTTPS.
4. Validate two-device browser connection after deploy.

Cloudflare Workers is fine for static hosting/routing and optional signaling helpers, but not as a UDP media relay.

## Build verification

```powershell
cd apps/web
npm run build
```

Current status: build passes.

## Current development priorities

1. Web connection reliability and UX.
2. Better diagnostics for failed ICE/peer setup.
3. Easier signaling handoff UX (copy/share flows).
4. Progressive fallback strategy for restrictive networks.

## GitHub push guide

If remote is not set:

```powershell
git remote add origin YOUR_GITHUB_REPO_URL
git push -u origin main
```

## Contributor references

- [apps/web/README.md](apps/web/README.md)
- [docs/architecture/IMPLEMENTATION-PHASES.md](docs/architecture/IMPLEMENTATION-PHASES.md)
- [docs/architecture/WEB-COMPATIBILITY.md](docs/architecture/WEB-COMPATIBILITY.md)
- [docs/architecture/V1-LAUNCH-CHECKLIST.md](docs/architecture/V1-LAUNCH-CHECKLIST.md)
