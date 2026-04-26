# Web Compatibility and Constraints

## Does this concept work on all OS?

Short answer: mostly yes, with browser and network caveats.

- Works on Windows, macOS, Linux, Android, and iOS through modern browsers with WebRTC and microphone permission support.
- Chromium, Firefox, and Safari all support WebRTC audio paths, but behavior differs by platform and browser version.

## Important limitations

- iOS/Safari has stricter autoplay/background behavior.
- Browser tabs cannot always keep realtime media alive when app is backgrounded for long periods.
- Some enterprise/campus networks block UDP or strict NAT paths.
- Pure peer-to-peer can fail in some NAT combinations without TURN.

## Practical reliability expectations

- Same LAN: high success probability.
- Typical home internet: generally good, but occasional failed peer setup.
- Strict enterprise NAT/firewall: lower success without fallback relay.

## Cloudflare hosting fit

- Static hosting is a good fit for the web app UI and signaling payload exchange UX.
- Client-side P2P media still remains between users directly.
- Cloudflare Workers can host signaling APIs if ever needed, but they are not a UDP media relay.

## Recommendation for v1

- Keep current no-central-relay model for privacy and simplicity.
- Document known network caveats clearly.
- Add optional self-hosted TURN fallback in later versions if connection success becomes a product blocker.
