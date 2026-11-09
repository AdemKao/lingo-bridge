<div align="center">

# Lingo Bridge

**Real-time translation experiments for desktop calls — powered by Rust.**

Build a language bridge between two people using Windows or macOS, with cloud BYOK and on-device AI experiments.

[**English**](README.md) · [**繁體中文**](README.zh-TW.md) · [**Contribute translations**](docs/i18n/README.md)

[![CI](https://github.com/AdemKao/lingo-bridge/actions/workflows/ci.yml/badge.svg)](https://github.com/AdemKao/lingo-bridge/actions/workflows/ci.yml)
[![Installers](https://github.com/AdemKao/lingo-bridge/actions/workflows/build-desktop.yml/badge.svg)](https://github.com/AdemKao/lingo-bridge/actions/workflows/build-desktop.yml)
![Windows and macOS](https://img.shields.io/badge/platform-Windows%20%7C%20macOS-4069c9)
![Alpha](https://img.shields.io/badge/status-experimental%20alpha-orange)

**[⬇ Download Alpha](https://github.com/AdemKao/lingo-bridge/releases)** ·
**[Quick Start](#quick-start)** ·
**[macOS signing](docs/macos-signing.md)** ·
**[Roadmap](docs/roadmap.md)** ·
**[Issues](https://github.com/AdemKao/lingo-bridge/issues)** ·
**[Changelog](CHANGELOG.md)**

</div>

> [!WARNING]
> **Alpha software — not a production-ready or officially supported LINE translation plugin.**
> Working Windows/macOS installers and green CI do **not** establish that native LINE calls
> translate correctly. The audio routing is based on OS devices, not LINE's private interfaces.
> Always tell call participants before sending audio to an AI provider, and test with consent.

## What is Lingo Bridge?

Our goal: **only the local speaker installs Lingo Bridge**. They speak Chinese on a PC or Mac
while someone using ordinary LINE (no additional software) could hear Japanese speech; replies
in Japanese would be converted back to Chinese audio and captions.

The codebase currently has a **cloud voice translation prototype** and a separate
**offline text/model and speech CLI research track**. End-to-end LINE compatibility
and latency still require real-device testing.

```text
   Outbound: Mic (ZH) ──► Cloud speech translation ──► Cable A ──► LINE ──► Remote (JA)
    Inbound: Local (ZH) ◄── Headphones / captions ◄── Cloud ◄── Cable B ◄── LINE (JA)
                            Your Windows / macOS computer
```

## Feature status

| Capability | Current state |
| --- | --- |
| Windows x64, macOS Apple Silicon, macOS Intel installers | ✅ Builds verified; unsigned **alpha** |
| Device selection, two separate cable routes and test tone | 🧪 Implemented prototype; manual setup |
| OpenAI live translation, subtitles and BYOK | 🧪 Implemented prototype; unverified LINE call |
| Independently configured directions and original-audio bypass | 🧪 Implemented |
| Local Ollama text translation and model downloads | 🧪 Available as text preview |
| Offline Chinese/Japanese STT (SenseVoice) | 🧪 Separate sherpa-onnx CLI experiment |
| Offline Japanese TTS (Supertonic) | 🧪 Separate sherpa-onnx CLI experiment |
| Offline Mandarin TTS and local live voice pipeline | ⏳ Not yet integrated |
| Real-world LINE compatibility, latency and echo safety | ⏳ Not yet validated |
| Driver auto-install, signed installers and auto-update | ⏳ Not available |

For current milestones see [Roadmap](docs/roadmap.md), [Offline Speech POC](docs/offline-speech-poc.md)
and [Changelog](CHANGELOG.md).

## Download

Download the latest **pre-release** from **[GitHub Releases](https://github.com/AdemKao/lingo-bridge/releases)**.
Do not confuse a successfully compiled alpha with a verified stable release.

| Computer | File | Important notes |
| --- | --- | --- |
| Windows 10/11 x64 | `.exe` (NSIS installer) | Unsigned alpha; SmartScreen may warn |
| Mac Apple Silicon | arm64 `.dmg` | Ad-hoc signed, **not** Apple notarized |
| Mac Intel | x86_64 `.dmg` | Ad-hoc signed, **not** Apple notarized |

Installers include **neither virtual audio cables nor model weights**.
No developer API key is embedded. If a Release is not yet published, see
the [Actions build artifacts](https://github.com/AdemKao/lingo-bridge/actions/workflows/build-desktop.yml)
or [release instructions](docs/releasing.md).

## Quick start

1. **Configure two independent virtual audio cables.** On Windows consider
   [VB-CABLE](https://vb-audio.com/Cable/index.htm); on Mac consider
   [BlackHole](https://github.com/ExistentialAudio/BlackHole) or compatible audio routing.
   Check each third-party license. A single cable cannot safely provide both directions.
2. **Set LINE's microphone to Cable A's recording endpoint**, and LINE's speaker
   to Cable B's playback endpoint. In Lingo Bridge, choose Cable A's playback
   endpoint for outgoing translated speech, Cable B's recording endpoint for
   incoming LINE speech, your physical microphone and your headphones.
3. **Use the test tone** to check that your remote test participant hears the sound.
   This does not consume API credits.
4. **Cloud mode:** enter your own OpenAI API key (BYOK); no key is saved to disk
   by the app. **Local text mode:** install [Ollama](https://ollama.com/download),
   then explicitly download a supported model from the in-app Model Manager.
   Local text mode is **not** yet local live-call translation.
5. Obtain **informed consent** from both participants before enabling speech
   processing. Use headphones and test for audio feedback or unintended original speech.

Full setup: **[Audio routing](docs/audio-routing.md)** ·
**[Local models](docs/local-models.md)** · **[Privacy and keys](docs/privacy.md)**

## Build from source

Requires Rust stable, Node.js 22 + npm and the
[Tauri 2 OS prerequisites](https://v2.tauri.app/start/prerequisites/).

```sh
git clone https://github.com/AdemKao/lingo-bridge.git
cd lingo-bridge/apps/desktop
npm install
npm run tauri dev
```

For developer-only cloud credentials, copy `.env.example` into a **root `.env`**.
Never commit a real key, and **never distribute a populated `.env`**.
End users enter BYOK in the UI.

```sh
# From repository root:
node .github/scripts/check-version.mjs
cargo fmt --all -- --check
cargo test -p lingo-core -p audio-core -p lingo-speech
cargo check -p lingo-bridge-desktop
# Optional, requires the sherpa-onnx native toolchain:
cargo check -p lingo-speech --features sherpa --bin lingo-speech-smoke
```

## Architecture

| Path | Responsibility |
| --- | --- |
| `apps/desktop` | Tauri 2 + React/TypeScript desktop UI |
| `apps/desktop/src-tauri` | Native Rust app, audio devices, cloud translation |
| `crates/lingo-core` | Direction and language types |
| `crates/audio-core` | PCM format helpers |
| `crates/lingo-speech` | Experimental offline STT / Japanese TTS and segmentation |
| `docs/` | Audio setup, models, privacy, releases, localization |

## Community and release process

- [Contributing](CONTRIBUTING.md) · [Code of Conduct](CODE_OF_CONDUCT.md) ·
  [Security policy](SECURITY.md)
- [Changelog](CHANGELOG.md) · [Semantic Versioning](docs/VERSIONING.md) ·
  [Release notes](docs/releases/v0.1.0-alpha.1.md)
- [Documentation localization](docs/i18n/README.md) · [繁體中文 README](README.zh-TW.md)
- [Issues](https://github.com/AdemKao/lingo-bridge/issues) ·
  [Pull requests](https://github.com/AdemKao/lingo-bridge/pulls)

> **Licensing:** A source-code license has **not** been selected. Public visibility
> alone does not grant reuse or redistribution rights. Third-party models and audio
> drivers have their own licenses. License selection is required before a broadly
> reusable open-source release.

---

<sub>Independent experiment; not affiliated with, approved by, or endorsed by LINE / LY Corporation.</sub>


### Offline Chinese/Japanese WAV translation (source-only POC)

An opt-in Rust CLI supports **both directions**: SenseVoice STT → localhost Ollama text translation → Japanese Supertonic or Chinese Matcha TTS → new WAV. This is not connected to LINE or bundled with desktop installers. It requires user-supplied model files; no OpenAI key. See [offline WAV guide](docs/offline-wav-pipeline.md).

### Local audio input signal check

The desktop UI now includes explicit one-second **microphone** and **LINE incoming Cable B** audio-level checks in addition to the outgoing test tone. Samples stay in RAM, are not stored or sent to AI, and only the RMS/peak levels are shown. This helps diagnose virtual cable setup; it does not by itself prove LINE translation works.
