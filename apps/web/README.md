# SoundBridge

Web-first app for realtime peer-to-peer audio onboarding.

## Goals

- Same-network automatic peer discovery hints
- Human-readable device names
- One-click onboarding for non-technical users
- Browser-only sender/receiver flow

## Stack

- Next.js App Router
- React client-side WebRTC
- Lightweight in-memory signaling API (for development and single-instance hosting)

## Run Locally

```powershell
cd apps/web
npm install
npm run dev
```

Open `http://localhost:5173`.

## Production Notes

- Host behind HTTPS.
- Media remains peer-to-peer.
- Signaling API in this repo is in-memory and best for single-instance deployment.

## UX Flow

1. Open app on two devices.
2. Set recognizable device names.
3. Select discovered peer from list.
4. Start call as broadcaster.
5. Receiver hears stream in browser audio element.
