# Architecture

## Product requirements

- Only the local Taiwanese/Chinese-speaking user installs software on Windows or macOS.
- The remote Japanese participant uses ordinary LINE, without installing software.
- Outbound audio: local Mandarin -> Japanese speech -> virtual microphone -> LINE.
- Inbound audio: LINE Japanese speech -> Mandarin speech and translated subtitles -> local headphones.
- **No official LINE call-audio API is available for this design.** The OS audio-device route must be tested.

## Source modules

| Module | Purpose | Current state |
| --- | --- | --- |
| `lingo-core` | Typed language directions | Implemented, tests |
| `audio-core` | PCM metadata and duration helpers | Implemented, tests |
| Tauri Rust host | Native status IPC | Implemented |
| React renderer | Desktop view and progress statuses | Implemented |
| Windows WASAPI adapter | Application audio capture | Not implemented |
| macOS CoreAudio adapter | Process audio capture | Not implemented |
| Virtual microphone and route manager | Output translated speech to LINE | Not implemented |
| AI realtime stream client | Two independent translation directions | Not implemented |
| Caption renderer / audio playback | Live transcript and translated speech | Not implemented |

Two speaker directions must have independent buffers, timestamps, translation sessions and interruption handling. Avoid replaying locally synthesized speech into the incoming capture.

Keep provider API credentials out of the frontend. Future providers should be isolated behind Rust interfaces, with short-lived session credentials or a backend proxy where needed. See [privacy](privacy.md).

## Status

This is scaffolding, not a working LINE voice translator. Measure compatibility, latency, feedback, echo cancellation and permissions on real devices before making product promises.
