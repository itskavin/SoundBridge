# SoundBridge OSS

SoundBridge OSS is now moving to a web-first roadmap for v1.

Native app development is paused for now. Current focus is a browser-based peer-to-peer audio experience that users can open instantly.

## Current v1 direction

- Primary product: web client
- Transport model: client-controlled P2P (WebRTC)
- Hosting model: static hosting only (no managed media relay)
- Goal: easier development and easier user onboarding

## What is in the repo

- [apps/web/public/index.html](apps/web/public/index.html): web app UI
- [apps/web/public/main.js](apps/web/public/main.js): WebRTC/manual signaling flow
- [apps/web/public/style.css](apps/web/public/style.css): styling
- [flutter/](flutter/): Flutter project for future cross-platform evolution
- [docs/architecture/WEB-COMPATIBILITY.md](docs/architecture/WEB-COMPATIBILITY.md): OS/browser compatibility notes
- [docs/architecture/V1-LAUNCH-CHECKLIST.md](docs/architecture/V1-LAUNCH-CHECKLIST.md): launch checklist

## Run web version locally

1. Open a terminal at project root.
2. Run: powershell -Command "Set-Location apps/web/public; python -m http.server 5173"
3. Open: http://localhost:5173

## Cloudflare hosting approach

- Host [apps/web/public](apps/web/public) as static assets.
- Keep media path peer-to-peer in browser clients.
- Do not run centralized media relay for v1.

## Push this repo to GitHub

1. Create a new empty GitHub repository.
2. Add remote: git remote add origin <your-repo-url>
3. Stage all: git add .
4. Commit: git commit -m "Web-first v1 cleanup"
5. Push: git push -u origin main

If your local branch is not main, replace main with your branch name.

## Contributor docs

- [apps/web/README.md](apps/web/README.md)
- [docs/architecture/IMPLEMENTATION-PHASES.md](docs/architecture/IMPLEMENTATION-PHASES.md)
- [docs/architecture/WEB-COMPATIBILITY.md](docs/architecture/WEB-COMPATIBILITY.md)
