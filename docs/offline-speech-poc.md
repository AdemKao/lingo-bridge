# Offline speech proof of concept (Rust / sherpa-onnx)

This increment introduces a tested **streaming energy-based utterance segmenter** and optional
**offline STT/Japanese TTS inference adapters**. They are deliberately **not connected to LINE**
or the Tauri audio runtime yet. The existing Cloud voice and Ollama local TEXT preview
remain unchanged.

## Goals and honest limitations

- Segment mono PCM speech into bounded utterance buffers using `lingo-speech::EnergySegmenter`.
  This is an energy threshold prototype, not production Silero VAD. Background noise,
  simultaneous speakers and quiet speech require better VAD and echo isolation.
- Run **offline speech recognition** on a 16kHz WAV using SenseVoice ONNX, which recognizes
  Chinese and Japanese (plus additional languages).
- Run **offline Japanese text-to-speech** using Supertonic v3 ONNX.
- Mandarin TTS requires a **separate Chinese-capable model**. Supertonic v3 does not support Chinese,
  so complete Chinese↔Japanese local voice calls remain **not implemented**.
- Models must be acquired and checked by the user. No pre-trained weights or native libraries
  are bundled into the source repository. The optional native `sherpa-onnx` crate may download its
  compiled inference library during the first build.

## 1. Run the pure-Rust speech segmentation tests

```sh
cargo test -p lingo-speech
```

This does not need any model file, API Key, GPU or Ollama.

## 2. Download compatible offline model files

Follow the official sherpa-onnx model instructions:

- [SenseVoice ASR](https://github.com/k2-fsa/sherpa-onnx/tree/master/rust-api-examples)
  (e.g. `sherpa-onnx-sense-voice-zh-en-ja-ko-yue-2024-07-17-int8` with `model.int8.onnx` and `tokens.txt`).
- [Supertonic v3 TTS](https://k2-fsa.github.io/sherpa/onnx/tts/supertonic.html)
  (the 2026-05-11 int8 package with seven ONNX/metadata files).

Review licensing and model distributions independently, especially before commercial use.
Store model files **outside Git**. Do not redistribute them without checking each license.
Try a short public-domain/test phrase, not private conversation audio.

## 3. Execute local STT and Japanese TTS tests

The project has an *optional* sherpa-onnx Rust dependency:

```sh
cargo run -p lingo-speech --features sherpa --bin lingo-speech-smoke -- \
  stt ./test-16khz-mono.wav \
  ./models/sensevoice/model.int8.onnx ./models/sensevoice/tokens.txt
```

Or synthesize Japanese WAV:

```sh
cargo run -p lingo-speech --features sherpa --bin lingo-speech-smoke -- \
  tts-ja ./models/supertonic ./translated-ja.wav "明日は何時に会えますか？"
```

A WAV file is written only where you explicitly request. This is **not a background recorder**.
The library's default build has no sherpa-onnx native dependency; opt in with `--features sherpa`.
To check feature compilation, use `cargo check -p lingo-speech --features sherpa` on
the target OS, with the sherpa ONNX toolchain allowed to download its native runtime.

## 4. Integrate into live calls (future, unverified)

1. Replace RMS energy gating with Silero VAD, preserve pre-roll, avoid feedback loops.
2. Separate capture and inference workers with bounded queues and cancellation.
3. Feed extracted segments into SenseVoice and track recognition timing.
4. Translate source text through installed local Ollama model.
5. Synthesize Japanese through an offline TTS, Chinese through a distinct Chinese model.
6. Convert output sample rate and route to LINE's virtual microphone/headphones.
7. Validate with a consenting Japanese caller on physical Windows and macOS machines.

**Do not show a “Local live voice ready” button before both languages are wired and tested.**

## Sources

- https://docs.rs/sherpa-onnx/latest/sherpa_onnx/
- https://github.com/k2-fsa/sherpa-onnx/tree/master/rust-api-examples
- https://k2-fsa.github.io/sherpa/onnx/tts/supertonic.html


## Offline WAV pipeline

See [offline-wav-pipeline.md](offline-wav-pipeline.md) for the new two-direction file-to-file speech translation test using local Ollama and the Chinese Matcha-TTS model. It is not a live LINE call.
