# Changelog

[English](CHANGELOG.md) · [繁體中文](CHANGELOG.zh-TW.md)

All notable user-facing changes are recorded here, using
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) conventions and
[Semantic Versioning](docs/VERSIONING.md). Entries describe **implemented**
or **build-validated** work; hardware validation is noted separately.

## [Unreleased]

### Planned
- Connect the offline speech pipeline into live LINE audio routing and measure latency.
- Validate real LINE calls on Windows and macOS; improve echo/feedback isolation.
- Improve cable setup, signing and update delivery.

## [0.1.0-alpha.2] - 2026-11-09

### Added
- One-second, user-triggered local-only microphone and LINE incoming virtual-cable level checks, showing RMS/peak without recording or cloud processing.
- Experimental Chinese Matcha-TTS WAV output in the source code.
- Offline Chinese↔Japanese **file-to-file** speech translation CLI: SenseVoice STT → local Ollama translation → Chinese Matcha/Japanese Supertonic TTS.
- Unit tests, explicit local model restrictions and separate Windows/macOS optional compile checks.

### Important limitations
- The offline speech pipeline is an **opt-in Rust CLI** and is NOT connected to the downloadable app's LINE live-call mode.
- Virtual audio routing, speech quality, echo isolation and end-to-end call functionality remain unverified on physical devices.
- Mac builds remain unsigned/unnotarized without a paid Apple Developer ID.

## [0.1.0-alpha.1] - 2026-11-09

### Added
- Rust/Tauri 2 desktop prototype with a React interface.
- Audio device selection, separate incoming/outgoing virtual routes and test tone.
- Experimental OpenAI realtime speech translation, subtitle events and BYOK support.
- Development-only `.env.example` and configurable translation/transcription models.
- Configurable spoken languages and original-audio bypass per direction.
- Local Ollama model management with offline **text** translation preview.
- Experimental offline audio segmentation, SenseVoice STT and Japanese TTS CLI via sherpa-onnx.
- Three-platform GitHub Actions alpha packaging: Windows x64, macOS arm64 and macOS Intel.
- English/Traditional Chinese documentation, contribution process and release notes.

### Known limitations
- LINE call translation is **not verified** end to end on real devices.
- Offline STT/TTS are **CLI-only tests**, not part of the desktop live-call engine.
- No offline Mandarin TTS or full local speech-to-speech pipeline yet.
- Installers are **not** officially signed/notarized, and do not include audio drivers.
- BYOK is session-only; no OS secure credential vault or updater yet.

[0.1.0-alpha.1]: https://github.com/AdemKao/lingo-bridge/releases/tag/v0.1.0-alpha.1

[0.1.0-alpha.2]: https://github.com/AdemKao/lingo-bridge/releases/tag/v0.1.0-alpha.2
