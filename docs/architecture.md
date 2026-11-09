# Technical architecture

## Goal

Only the Taiwanese/Chinese-speaking participant installs a Windows/macOS app. The Japanese-speaking participant uses normal LINE without software changes.

The LINE native call is **never modified or accessed through private LINE APIs**. We bridge its OS audio endpoints using two separately routed virtual audio cables.

## Executable audio graph

```text
                OUTBOUND (zh -> ja)
physical mic  --CPAL input--> resample 24k mono PCM16
    -> OpenAI Realtime Translation session A (output ja)
    -> 24k PCM16 speech -> resample to Cable A output rate
    -> Cable A playback -> LINE microphone on Cable A recording
    -> Japan LINE user hears Japanese

                 INBOUND (ja -> zh)
Japan user speaks Japanese -> LINE Desktop speakers = Cable B playback
    -> Cable B recording --CPAL input--> 24k mono PCM16
    -> OpenAI Realtime Translation session B (output zh)
    -> Chinese speech -> physical headphones (local only)
    -> original / translated text deltas -> Tauri event -> subtitles
```

No source audio goes straight to LINE from this app. The app does not monitor original Japanese on headphones.

## Components

- Rust / Tauri host: explicit device enumeration, session lifecycle, consent gate, status events.
- CPAL 0.15: OS default WASAPI/CoreAudio input/output streams, explicit virtual endpoint selection.
- Resampler: stateful streaming linear interpolation; model input 24kHz mono PCM16 with 200ms packets, silence preserved.
- OpenAI dedicated Translation WebSocket: `wss://api.openai.com/v1/realtime/translations?model=gpt-realtime-translate`, session output `ja` or `zh`, optional source transcription using `gpt-realtime-whisper`.
- React: local BYOK field and settings, progress, captions, stop, 0.5-second cable test tone.
- `.env`: local developer configuration, read by Rust and never copied into frontend assets.

API specification: https://developers.openai.com/api/docs/guides/realtime-translation

## Current implementation boundaries

This is an *integrated prototype*, not a validated production application. OS device routing and API message wiring are coded, but cannot be verified end to end without two virtual audio cables, a valid permitted API key, a real Windows/macOS machine and a live LINE call. Tests cover the pure functions and CI compiles the native host.

There is **no** automatic virtual device installation, auto capture of LINE's process audio, account system, installer signing, professional audio resampler, speaker diarization, billing backend or failover. Network failures currently end the session and require restart.

## Performance and safety

- Audio callbacks use bounded channels and bounded playback queues. Dropped frames under overload can degrade translations; instrument queue drops before production.
- Output is limited to a short queue to prevent growing audio delay.
- BYOK values go over local Tauri IPC to the Rust process; they are not written to disk by the app. A malicious local machine could still inspect process memory.
- Incoming call audio is sent to a third-party AI provider **only** after the user checks the consent box and clicks Start.
- Source and translation captions remain in memory, not automatically persisted.
- Per-provider price, rate limits and access permissions are controlled by the user's API account.

## Flexible language routing and local provider (Issue #1)

The `lingo-core` crate now defines `CallLanguageSettings` with independent `AudioDirection { source, target }`. `target: None` (and a target equal to the source) means local passthrough; the engine sends sample-rate-converted original audio through the existing audio router without starting a model session. For cloud translated routes, the OpenAI Realtime session output language is chosen at runtime rather than hardcoded to Japanese or Chinese. Initial UI choices are zh/ja/en/ko/fr/es.

The `local.rs` Ollama adapter supports installing curated text models, checking installation, removing a model with explicit confirmation, and local TEXT translation previews. It is explicitly **not a speech STT/TTS replacement**. In Local or Hybrid mode with translated audio, the start command returns a clear error instead of silently using cloud AI. The local-only text API is fixed to 127.0.0.1 for privacy. See [local-models](local-models.md).
