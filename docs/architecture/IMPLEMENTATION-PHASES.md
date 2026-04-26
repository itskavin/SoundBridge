# SoundBridge OSS Implementation Phases

This file is the execution playbook for contributors and AI agents.

## Current Status

- Phase 1: Completed (core models and handshake validation with tests)
- Phase 2: Completed (endpoint parsing, transport negotiation, and session establishment tests)
- Phase 3: Completed (audio profile and runtime mode integration in app flows)
- Phase 4: Completed for prototype scope (in-memory discovery and connection lifecycle state machine)
- Phase 5: In progress (web peer mode and optional self-hosted static site delivery)
- Phase 6: Planned (hardening, benchmark publication, and beta checklist)

## Phase 1: Core Contract Stability

### Scope

- Finalize core data models for sessions, handshake, and profile settings.
- Keep APIs minimal and stable.

### Required outputs

- Stable public types in `sb-core`.
- Unit tests for validation and error paths.
- Protocol v1 handshake docs aligned with code.

### Exit criteria

- `cargo fmt --all`
- `cargo build --workspace`
- `cargo test --workspace`
- No breaking rename of public core types without protocol note.

## Phase 2: Transport Maturity

### Scope

- Expand endpoint parsing robustness.
- Implement session establishment behavior for default transport.
- Prepare compatibility hooks for alternate transport modes.

### Required outputs

- Deterministic transport tests.
- Error model with clear mapping for invalid endpoint/handshake failures.
- Protocol section covering transport negotiation.

### Exit criteria

- Test coverage includes success and failure transport paths.
- Transport default remains `quic` unless version-gated.

## Phase 3: Audio Pipeline Integration

### Scope

- Define audio config and runtime mode boundaries.
- Integrate profile-driven buffer and mode decisions.

### Required outputs

- Shared audio configuration contracts.
- Integration tests between transport and audio profile selection.

### Exit criteria

- No profile silently ignored during session setup.
- Config defaults documented and tested.

## Phase 4: Discovery and Session UX

### Scope

- Implement discoverability for local peers.
- Improve connection setup success and reconnection behavior.

### Required outputs

- Discovery module with deterministic simulation tests.
- Connection state machine documentation.

### Exit criteria

- Documented setup flow and recovery behavior.
- Reconnect tests for interrupted sessions.

## Phase 5: Web Peer Mode and Optional Static Hosting

### Scope

- Add browser-based sender/receiver mode without centralized relay control.
- Keep desktop/mobile local mode fully functional without cloud.

### Required outputs

- Static web app that supports sender/receiver behavior via WebRTC.
- Documentation for static hosting only (no managed relay requirement).

### Exit criteria

- Web mode works over manual signaling and direct peer connection.
- Core workflows require no cloud login or centralized control plane.

## Phase 6: Hardening and Beta Release

### Scope

- Reliability and performance tuning.
- Packaging, release notes, and known-issues handling.

### Required outputs

- Benchmark runbook and latest baseline.
- Soak test results summary.
- Public beta release checklist.

### Exit criteria

- Performance metrics published.
- Regression test suite green.
- Release notes include compatibility details.

## Ongoing Process Rules

1. Every feature PR includes tests.
2. Every protocol change updates `protocol/` docs.
3. Every architecture change adds/updates an ADR.
4. Every release candidate has benchmark evidence.
5. Never paywall core local streaming paths.

## How To Execute and Verify

From the workspace root, run:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/test-all.ps1
```

Quick smoke-only run:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/run-smoke.ps1
```

Realtime validation (two terminals):

```powershell
powershell -ExecutionPolicy Bypass -File scripts/realtime-server.ps1 -Bind 0.0.0.0:7000 -Seconds 60
powershell -ExecutionPolicy Bypass -File scripts/realtime-client.ps1 -Server 127.0.0.1:7000 -Seconds 20
```

Realtime success criteria:

- Client reports non-zero `sent` and `echoed` values.
- Server reports non-zero `received` and `echoed` values.
- Client reports `avg_rtt_ms` greater than or equal to zero.

Manual equivalent commands:

```powershell
$cargo="$env:USERPROFILE\.cargo\bin\cargo.exe"
& $cargo fmt --all
& $cargo build --workspace
& $cargo test --workspace
& $cargo run -p sb-desktop-app
& $cargo run -p sb-mobile-app -- 127.0.0.1:7000
```
