# Session Protocol v1 (Draft)

## Handshake

1. Client sends `hello` with client id, platform, and preferred transport.
2. Server/peer replies with accepted transport and session id.
3. Client sends audio profile selection (`voice`, `music`, `ultra_low_latency`).

## Transport Modes

- `quic` (default)
- `rtp_udp` (experimental fast path)

## Security

- Encrypted transport required.
- Pairing secret or trusted local key required for session join.
