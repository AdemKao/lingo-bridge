# Lingo Bridge

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
