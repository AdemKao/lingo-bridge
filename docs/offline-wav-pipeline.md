# Offline Chinese ↔ Japanese WAV pipeline (experimental)

This is a Rust command-line **file-to-file** test, not the live LINE call engine.
The CLI reads only explicitly provided WAV files. No OpenAI API key or cloud translation
is required. Text is sent only to Ollama on 127.0.0.1:11434, without proxy or redirects.

## Preparation

- Install and start Ollama, and download the model `qwen3:4b` or `translategemma:4b`.
- Obtain SenseVoice STT ONNX + tokens for 16 kHz mono Chinese/Japanese WAV.
- For Japanese speech, supply a Supertonic v3 int8 model directory.
- For Chinese speech, supply Matcha Baker Chinese model directory (model-steps-3.onnx,
  lexicon.txt, tokens.txt, dict/, phone.fst, date.fst, number.fst), and a separate
  vocos-22khz-univ.onnx vocoder from sherpa-onnx's Matcha example.
- Model weights must be provided by the user and reviewed for license compatibility.
- Matcha Baker is a Mandarin Chinese voice and may not match Taiwanese pronunciation.

Official sherpa-onnx Rust examples: https://github.com/k2-fsa/sherpa-onnx/tree/master/rust-api-examples

## Compile and test

~~~sh
cargo test -p lingo-speech
cargo check -p lingo-speech --features offline-pipeline --bin lingo-speech-smoke
mkdir -p outputs
~~~

## Individual TTS voice tests

~~~sh
cargo run -p lingo-speech --features sherpa --bin lingo-speech-smoke -- tts-ja ./models/supertonic ./outputs/japanese.wav "こんにちは"
cargo run -p lingo-speech --features sherpa --bin lingo-speech-smoke -- tts-zh ./models/matcha-zh ./models/vocos-22khz-univ.onnx ./outputs/chinese.wav "你好"
~~~

## One-shot WAV translation in both directions

~~~sh
# Chinese audio -> Japanese voice
cargo run -p lingo-speech --features offline-pipeline --bin lingo-speech-smoke -- translate-wav zh-ja ./samples/zh-16k.wav ./models/sensevoice/model.int8.onnx ./models/sensevoice/tokens.txt qwen3:4b ./models/supertonic - ./outputs/to-japanese.wav

# Japanese audio -> Mandarin Chinese voice
cargo run -p lingo-speech --features offline-pipeline --bin lingo-speech-smoke -- translate-wav ja-zh ./samples/ja-16k.wav ./models/sensevoice/model.int8.onnx ./models/sensevoice/tokens.txt qwen3:4b ./models/matcha-zh ./models/vocos-22khz-univ.onnx ./outputs/to-chinese.wav

# Text-only stage of the same pipeline
cargo run -p lingo-speech --features offline-pipeline --bin lingo-speech-smoke -- translate-text zh-ja qwen3:4b "明天見"
~~~

The CLI prints transcripts on the terminal and writes a WAV only where requested.
Do not use confidential recordings during debugging. Input/output WAV must differ.
Speech recognition must use 16 kHz mono WAV. The Chinese model is geared toward
Simplified Chinese and is not a Taiwanese Mandarin voice.

## Remaining work

- Run real model files on both OSes; measure accuracy, RAM usage and latency.
- Add robust VAD, cancellation and buffers to the Tauri live audio engine.
- Integrate speech models with LINE virtual audio routes and test with consenting callers.
- Make model installation and license review accessible from the desktop UI.

**Do not describe the application as a functioning offline LINE voice translator yet.**
