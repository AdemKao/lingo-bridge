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

## Issue #1 — Local / Hybrid and flexible languages

- [x] Independently configurable source and target for outbound/inbound voice paths (zh/ja/en/ko/fr/es)
- [x] Per-direction original-audio passthrough; zero AI sessions when both bypass translation
- [x] Provider mode selection and explicit unsupported mode validation
- [x] Local Ollama service inventory, curated model catalog, explicit pull/delete operations
- [x] Local Ollama text translation preview without cloud credentials
- [ ] True offline STT + TTS for Chinese and Japanese voice calls
- [ ] Hybrid provider combination for end-to-end real-time calls
- [ ] Real hardware / LINE validation, latency and privacy verification
- [ ] Full STT/TTS model management, integrity hashes and license audits

Progress and acceptance criteria: https://github.com/AdemKao/lingo-bridge/issues/1

## Issue #1 follow-up — Offline speech POC

- [x] Add provider-independent bounded speech segmenter with Rust unit tests
- [x] Add optional sherpa-onnx based SenseVoice STT and Japanese Supertonic TTS WAV smoke-test adapters
- [ ] Verify sherpa-native feature compilation and inference with actual model files on Windows and macOS
- [ ] Add Chinese-capable local TTS (Japanese TTS alone is not bidirectional voice translation)
- [ ] Wire native local speech streams through Ollama and existing LINE audio router with VAD/latency tests

See [offline speech POC](docs/offline-speech-poc.md). This is **not** complete offline voice calling.
