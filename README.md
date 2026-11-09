# Lingo Bridge

> **2026-11 language/local phase:** The desktop app now includes independent outbound/inbound language selectors with a **No translation / original audio** option, plus local Ollama text translation and opt-in model download. Offline STT/TTS and actual local voice interpretation are **not implemented yet**. See [Local model guide](docs/local-models.md) and [Issue #1](https://github.com/AdemKao/lingo-bridge/issues/1).

**Rust-first experimental Chinese ↔ Japanese real-time call translation for Windows and macOS.**

The **local Chinese-speaking user alone** installs the app, while the Japanese-speaking participant uses ordinary LINE without additional software.

> **Status: integrated prototype, not a proven production LINE translator.** Device capture/playback and the OpenAI Realtime Translation WebSocket pipeline are implemented, but **real LINE calls have not yet been verified end to end on physical Windows and macOS machines**. LINE integration is unofficial and depends on manually configured virtual audio devices.

## What works in code

- Tauri 2 + React/TypeScript desktop interface with device picker and status.
- Cross-platform CPAL (Windows WASAPI / macOS CoreAudio) audio input and output.
- Two independent streaming translation sessions, Mandarin→Japanese and Japanese→Mandarin.
- OpenAI `gpt-realtime-translate` dedicated WebSockets, 24 kHz PCM16 audio in 200 ms packets and streamed speech output.
- Original / translated transcript deltas displayed in the app.
- Startup/shutdown controls and bounded audio buffers.
- **Developer `.env`** and **per-session BYOK** (user enters their own API key in the UI; no key saved to disk).
- **Test tone** to verify LINE's virtual microphone route without an API key.

## Requirements

- Windows 10/11 or macOS supported by Tauri 2, with Rust stable and Node.js 22.
- [Tauri 2 prerequisites](https://v2.tauri.app/start/prerequisites/).
- **Two separately configured virtual audio cable devices**, not one shared cable.
  - Windows examples: VB-Audio Cable / similar independent cable instances (check commercial licensing).
  - Mac example: separately installed/configured BlackHole instances or equivalent (requires manual routing).
- LINE Desktop, headphones and a consenting test participant.
- An OpenAI API account with access to `gpt-realtime-translate`; realtime translation incurs metered costs.

## Configure API access

```sh
git clone https://github.com/AdemKao/lingo-bridge.git
cd lingo-bridge
cp .env.example .env
```

Set values in **the root `.env` file**:

```dotenv
OPENAI_API_KEY=your-key-here
OPENAI_TRANSLATION_MODEL=gpt-realtime-translate
OPENAI_TRANSCRIPTION_MODEL=gpt-realtime-whisper
```

`.env` is ignored by Git. **Never commit real credentials or distribute a populated `.env` with your app.** If you prefer BYOK, leave `OPENAI_API_KEY` empty and enter your own key in the desktop UI each time. BYOK overrides environment configuration and is erased from the UI after starting the call.

## Start desktop development

```sh
cd apps/desktop
npm install
npm run tauri dev
```

`npm run dev` is web-only: native audio controls require the desktop host.

## Configure LINE audio routing

**Two separate virtual cables are mandatory.**

| Lingo Bridge | What to choose | LINE setting |
| --- | --- | --- |
| ① Physical mic | Your actual microphone | Do NOT select your physical mic in LINE |
| ② Incoming LINE audio | Recording/input endpoint of Cable B | LINE **speaker/output** = Cable B playback |
| ③ Japanese output to LINE | Playback/output endpoint of Cable A | LINE **microphone/input** = Cable A recording |
| ④ Mandarin headphones | Your physical headphones | No direct original LINE audio |

Read [the audio routing setup guide](docs/audio-routing.md) before starting a call. Press the **0.5-second test tone** button to verify your outgoing cable first. Only start AI processing after informing both call participants and getting consent.

## Run tests

```sh
cargo fmt --all -- --check
cargo test -p lingo-core -p audio-core
cd apps/desktop && npm run build
```

Native compilation on Windows and macOS is checked by GitHub Actions.

## Architecture and limitations

The app uses **OS audio device routing**, not an official LINE SDK or API. It cannot automatically intercept every LINE call or set all LINE audio devices. There is no automatic driver installer, OS credential vault, signed release installer, robust reconnection, or guaranteed translation latency yet. **Real calls and model access must be tested with your own devices and credentials.**

See [architecture](docs/architecture.md), [privacy/BYOK](docs/privacy.md), [roadmap](docs/roadmap.md).

Independent project, not affiliated with or endorsed by LINE or LY Corporation. No open-source license has been selected.

## Unsigned preview builds

After generating icons, local preview installers can also be built using `npm run tauri -- build --config src-tauri/tauri.bundle.conf.json` from `apps/desktop`.

The [Desktop preview installers workflow](https://github.com/AdemKao/lingo-bridge/actions/workflows/build-desktop.yml) provides a **manual** GitHub Actions build for Windows NSIS and a macOS app archive. Trigger it from Actions → Run workflow. These are **unsigned developer builds** for testing, not production releases. Virtual audio drivers are **not bundled**; on macOS, unsigned/unnotarized apps may require platform trust or signing before they can run. Production releases require signing, driver licensing checks and real-call verification.

## Language routing

Each direction independently selects its spoken language and desired listener language. The app initially exposes Traditional Chinese, Japanese, English, Korean, French and Spanish as targets, matching supported cloud output options. Select **No translation / original audio** to relay audio without creating a translation session for that direction. If both directions are original audio, no OpenAI API Key is needed. Local/Hybrid voice translation remains disabled until offline STT and TTS exist.

To test offline text translation, start Ollama locally and use the in-app Model Manager; install Qwen3 4B or TranslateGemma 4B only after reviewing model licenses. See [Local model guide](docs/local-models.md).

## Offline speech experiments (opt-in)

The repository now contains `crates/lingo-speech`, an offline speech **utterance segmenter** and an optional, model-dependent **sherpa-onnx** command-line smoke test for Chinese/Japanese SenseVoice STT and Japanese Supertonic TTS. No API key is required for these local tests. **These adapters are not yet connected to the LINE live-call engine.** See [offline speech setup](docs/offline-speech-poc.md).

```sh
cargo test -p lingo-speech
cargo run -p lingo-speech --features sherpa --bin lingo-speech-smoke -- stt INPUT_16KHZ.WAV MODEL.ONNX TOKENS.TXT
cargo run -p lingo-speech --features sherpa --bin lingo-speech-smoke -- tts-ja MODEL_DIR OUTPUT.WAV "こんにちは"
```

The Sherpa native library and model weights are separate downloads. Do not commit model weights into this repository.

## Downloadable alpha installers

The public-repository [desktop installer workflow](.github/workflows/build-desktop.yml) builds Windows x64 `.exe`, macOS Apple Silicon `.dmg` and macOS Intel `.dmg` packages using standard GitHub Actions runners. The workflow produces downloadable Actions artifacts for private tests; the optional **manual** publish mode creates a **draft pre-release**, which must be reviewed and published separately. This is a development preview, not a verified LINE calling product. [Read the release guide](docs/releasing.md).

When available, public installers appear under [GitHub Releases](https://github.com/AdemKao/lingo-bridge/releases).
