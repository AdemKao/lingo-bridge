# Changelog

[English](CHANGELOG.md) · [繁體中文](CHANGELOG.zh-TW.md)

All notable user-facing changes are recorded here, using
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) conventions and
[Semantic Versioning](docs/VERSIONING.md). Entries describe **implemented**
or **build-validated** work; hardware validation is noted separately.

## [Unreleased]

### Added
- One-second, user-triggered local-only microphone and incoming LINE virtual-cable input-level diagnostics in the desktop UI.
- Experimental Chinese Matcha-TTS WAV output; offline Chinese↔Japanese file-to-file pipeline using SenseVoice, local Ollama and two TTS voices (CLI only, no LINE routing).
- Optional pipeline feature compilation in Windows/macOS CI.

### Planned
- Validate LINE audio capture and bilingual speech on physical Windows/Mac devices.
- Complete offline Mandarin TTS and connect local STT/translation/TTS to live audio.
- Improve cable setup, signing and update delivery.

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
