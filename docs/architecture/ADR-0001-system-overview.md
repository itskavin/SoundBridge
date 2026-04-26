# ADR-0001: System Overview

## Status

Accepted

## Context

SoundBridge OSS needs a cross-platform, low-latency architecture that supports open-source core development and optional managed cloud monetization.

## Decision

- Rust workspace for core modules (`sb-core`, `sb-transport`, `sb-audio`)
- QUIC-first transport with future RTP/UDP fast-path support
- Local-first operation with optional managed relay service
- Public protocol specifications in `protocol/`

## Consequences

- Shared core logic can be reused across desktop and mobile wrappers.
- Transport can evolve without breaking app-facing contracts.
- Governance and benchmark transparency are first-class deliverables.
