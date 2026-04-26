# SoundBridge OSS

SoundBridge OSS is a web-first, open-source, low-latency audio relay alternative focused on direct peer-to-peer browser audio.

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

## Repository map

- [apps/web/public/index.html](apps/web/public/index.html): web UI and workflow
- [apps/web/public/main.js](apps/web/public/main.js): WebRTC signaling and session logic
- [apps/web/public/style.css](apps/web/public/style.css): UI styles
- [apps/web/README.md](apps/web/README.md): web-specific notes
- [flutter/](flutter/): retained for future native/hybrid work
- [docs/architecture/WEB-COMPATIBILITY.md](docs/architecture/WEB-COMPATIBILITY.md): browser and OS constraints
- [docs/architecture/V1-LAUNCH-CHECKLIST.md](docs/architecture/V1-LAUNCH-CHECKLIST.md): release checklist

## Quick local run

1. Open terminal in repo root.
2. Run local static server:

```powershell
powershell -Command "Set-Location apps/web/public; python -m http.server 5173"
```

3. Open [http://localhost:5173](http://localhost:5173).
4. Test with two browser windows/devices:
	1. Broadcaster: Start Broadcaster, copy offer.
	2. Listener: paste offer, create answer, copy answer.
	3. Broadcaster: paste answer, apply answer.

## Web behavior and compatibility

Cross-OS support is generally good on modern browsers across Windows, macOS, Linux, Android, and iOS, with caveats:

- Safari/iOS has stricter media/autoplay/background rules.
- Some corporate or restricted NAT/firewall environments may fail pure P2P setup.
- HTTPS is required in production for stable media permissions.

Detailed notes: [docs/architecture/WEB-COMPATIBILITY.md](docs/architecture/WEB-COMPATIBILITY.md).

## Cloudflare deployment

For your requested model (no controlled relay), deploy only static web assets.

1. Deploy [apps/web/public](apps/web/public) as static content.
2. Enable HTTPS.
3. Validate two-device browser connection after deploy.

Cloudflare Workers is fine for static hosting/routing and optional signaling helpers, but not as a UDP media relay.

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
