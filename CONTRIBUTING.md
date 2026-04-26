# Contributing

## Workflow

1. Open an issue describing the problem or feature.
2. Propose protocol or architecture changes with an ADR when needed.
3. Submit small, focused pull requests.

## Quality Gates

- Workspace must build successfully.
- New behavior requires tests or benchmark notes.
- Keep APIs documented and backward-compatible when possible.

## Commit Guidance

Use clear messages such as:

- `feat(transport): add QUIC handshake validation`
- `fix(audio): handle invalid sample rate gracefully`
