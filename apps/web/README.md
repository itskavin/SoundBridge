# SoundBridge Web

SoundBridge Web is a serverless peer mode.

- No managed relay required
- No centralized media control
- Browser peers connect directly over WebRTC
- Offer/answer is exchanged manually by users

## Run Locally

```powershell
cd apps/web/public
python -m http.server 5173
```

Open `http://localhost:5173`.

## Host Online (Static Only)

Deploy `apps/web/public` to any static host (GitHub Pages, Netlify, Cloudflare Pages, Vercel static).

Requirement:

- Serve over HTTPS in production so browser media APIs work.

## Common Use Cases

- Guest listener joins from browser without installing app
- Team demo sessions across OSes in minutes
- Fast fallback path when native app installation is blocked
- Classroom/workshop broadcasting from one device to many listeners
