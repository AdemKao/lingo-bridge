# Development status and remaining work

## Integrated functional prototype

- [x] Tauri 2 + React desktop UI with explicit audio input/output selection
- [x] Rust workspace and domain tests
- [x] Windows/macOS CPAL capture/playback adapter
- [x] Two-way virtual-cable audio architecture (manual OS + LINE audio settings)
- [x] OpenAI dedicated Realtime Translation WebSocket client
- [x] 24 kHz PCM16 encode/decode, per-direction packet scheduling and resampling
- [x] Two independent sessions and subtitle deltas
- [x] Start/Stop and consent gate, failure status
- [x] Short device test tone without a key
- [x] .env developer key and runtime user-supplied BYOK, no default persistence
- [x] GitHub CI source checks for Rust, frontend and native Windows/macOS
- [ ] Verified **real** LINE call with a Japanese-speaking participant
- [ ] Verified real-time **both-way** translated audio and latency on Windows + macOS
- [ ] Publish supported device / LINE / OS version matrix

## Before a public release

- [ ] Windows/MSIX and macOS signed/notarized installer
- [ ] Automated virtual audio routing assistant and permissions
- [ ] Stable device identifiers beyond enumeration indices
- [ ] Echo suppression / echo leakage and failure recovery tests
- [ ] Proper bandlimited resampling and adaptive jitter buffers
- [ ] Reconnection, connection health and model usage metrics
- [ ] Secure OS credential vault option; current BYOK is ephemeral
- [ ] Billing protections, test fixtures for quality and latency
- [ ] Third-party license review for bundled virtual audio drivers

**Product promise gate:** native LINE call compatibility is not verified until actual end-to-end recordings/monitoring measurements with consent on both OSes succeed.
