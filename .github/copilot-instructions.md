<!-- Workspace-specific instructions for GitHub Copilot -->

# SoundBridge OSS Copilot Instructions

SoundBridge OSS is an open-source, low-latency audio relay platform and a quality-focused alternative to AudioRelay.

## 1. Product Intent

- Prioritize low-latency, reliable browser-based streaming for v1, with cross-platform native paths paused.
- Keep core local streaming capabilities open source.
- Keep realtime media paths client-controlled and peer-to-peer where possible.
- Maintain transparent engineering: measurable benchmarks, reproducible tests, and clear architectural records.

## 2. Repository Structure

- `crates/sb-core`: session models, handshake/domain rules, shared types.
- `crates/sb-transport`: endpoint parsing, transport abstraction, session establishment.
- `crates/sb-audio`: audio configuration and mode abstractions.
- `apps/desktop`: paused for active development unless explicitly requested.
- `apps/mobile`: paused for active development unless explicitly requested.
- `apps/web/app`: browser-based sender/receiver app (Next.js, no centralized media relay control).
- `protocol/`: wire protocol specs and versioned message contracts.
- `docs/architecture`: ADRs and system decisions.
- `docs/benchmarks`: benchmark methodology and result templates.

## 3. Working Rules for Copilot

- Prefer minimal, focused changes over broad refactors.
- Preserve cross-platform behavior; avoid OS-specific logic without explicit fallback.
- Add tests for every non-trivial behavior change.
- Update protocol or ADR docs when changing message formats, transport behavior, or architecture.
- Do not introduce paid feature gates into local-first core paths.

## 4. Quality and Verification Gates

Before considering implementation complete, run:

1. `cargo fmt --all`
2. `cargo build --workspace`
3. `cargo test --workspace`

If a toolchain path issue exists in this environment, use:

- `C:\Users\kavin\.cargo\bin\cargo.exe`

Build task:

- VS Code task label: `Build Rust Workspace`

## 5. Protocol and Compatibility Policy

- Any protocol change in `protocol/` must include versioning notes.
- Do not silently break compatibility; document migration behavior.
- New transport options must preserve existing `quic` default behavior unless explicitly version-gated.

## 6. Testing Standards

- Unit tests for parsing, validation, and handshake logic.
- Integration tests for session establishment flows as modules mature.
- Add regression tests for every reported bug in core/session/transport areas.
- Keep tests deterministic; no hard dependency on external network services in default test path.

## 7. Benchmarking Standards

Track and improve these metrics:

- Connection success rate
- p50 and p95 end-to-end latency
- Jitter
- Dropout/glitch rate
- CPU utilization

Benchmark updates must include reproducible parameters (platform, network type, profile, run count).

## 8. Security and Privacy Expectations

- Encrypted transport by default.
- No telemetry by default; if introduced, it must be opt-in and documented.
- Never log sensitive pairing secrets or raw credentials.

## 9. Monetization Guardrails

- Free and open: local LAN streaming, profile selection, baseline quality controls.
- Paid optional services can include support/subscriptions, but core realtime paths must remain peer-controlled.
- Do not degrade free local quality or introduce artificial local time limits.

## 10. Documentation Requirements for Changes

When relevant, update at least one of:

- `README.md` for user-facing behavior
- `protocol/*.md` for wire/session changes
- `docs/architecture/*.md` for architectural decisions
- `docs/benchmarks/*.md` for measurement methodology

## 11. Delivery Phases

Phase 1: Web Foundation

- Stabilize web signaling and WebRTC connection flow.
- Ship deterministic checks for offer/answer and connection-state transitions.

Phase 2: Browser Connectivity

- Implement robust peer establishment across common browser stacks.
- Measure baseline browser latency and reliability.

Phase 3: Web UX Hardening

- Improve reconnect behavior, browser permissions UX, and session diagnostics.
- Reduce connection drop friction for non-technical users.

Phase 4: Web Peer Mode

- Add browser sender/receiver mode with static hosting only.
- Keep local mode independent of managed backend requirements.

Phase 5: Hardening and Public Beta

- Run soak tests, regression sweeps, and benchmark comparisons.
- Publish changelog and known limitations for beta users.

## 12. Definition of Done

All of the following must be true:

1. Code is formatted and builds successfully.
2. Relevant tests pass and new behavior is covered.
3. Docs and protocol/ADR notes are updated when applicable.
4. Changes preserve local-first open-source guarantees.
