# Local model setup, language selection, and limitations

This phase adds **language routing and a local TEXT translation preview**. Full offline voice calling is a separate milestone and is **not** implemented by this change.

## Select call languages separately

The desktop app offers six target languages initially: Traditional Chinese (zh), Japanese (ja), English (en), Korean (ko), French (fr) and Spanish (es). OpenAI Realtime Translation currently supports these output languages, but quality and source-language recognition vary. For every direction:

- **You → remote**: set your source speech language and what the remote should hear.
- **Remote → you**: set the remote's source speech language and what you should hear.
- **No translation / original**: route the captured audio to the configured virtual output untouched semantically (sample-rate conversion still occurs), **without any cloud/model request for that direction**. Selecting the same input/output language also bypasses translation.
- When *both* directions are passthrough, OpenAI credentials are not required and no model WebSockets are created.
- Changing language during a call requires stopping the current audio route, then restarting.

Only the participant running Lingo Bridge changes these settings. The remote participant uses ordinary LINE.

## Provider modes

| Mode | Voice calls | Text preview | Data handling |
| --- | --- | --- | --- |
| Cloud | OpenAI Realtime Translation + original passthrough | Not provided | Translated speech goes to OpenAI using BYOK or development .env |
| Local | Original passthrough only **for now** | Ollama local text translation | Text preview is sent only to local Ollama service |
| Hybrid | Original passthrough only **for now** | Ollama local text translation | Speech STT/TTS integration remains TODO |

The app **blocks translated voice mode** if Local/Hybrid is selected; it must not silently fall back to cloud or claim offline speech is ready.

## Install and use local text translation

1. Install [Ollama](https://ollama.com/) on your own Windows/Mac device and start the local service.
2. Open Lingo Bridge and select the Local or Hybrid engine. The Model Manager calls **http://127.0.0.1:11434** only; remote Ollama endpoints are intentionally not configurable.
3. Review model **source and license** before clicking the explicit download button. Two curated choices:
   - [Qwen3 4B](https://ollama.com/library/qwen3) — `qwen3:4b`, approximately 2.5 GB.
   - [TranslateGemma 4B](https://ollama.com/library/translategemma) — `translategemma:4b`, approximately 3.3 GB.
4. Clicking Download asks for confirmation, then asks the *already running local Ollama service* to download the weights. It does not install Ollama itself. If model installation succeeds, it will appear as installed after refresh.
5. Use the **local text preview** to translate short Chinese, Japanese or other supported-language strings. No OpenAI key is required.
6. The model can be deleted via a separate explicit confirmation. **Warning:** deletion affects other applications using the same Ollama installation.

Ollama may require a network connection for initial installation/model downloads; once installed, **local text inference** does not need Internet. **LINE calls still require Internet**, even if future STT/translation/TTS are local.

## Future offline voice stack

- Offline STT: evaluate sherpa-onnx and whisper.cpp for each source language; select VAD and buffering strategy.
- On-device translation: use an installed Ollama model as a provider, initially as a **text** component only.
- Offline TTS: evaluate MeloTTS/Kokoro/Qwen3-TTS for correct local voices and commercial licensing.
- ModelManager: extend curated catalog to STT and TTS weights and include SHA256, distribution licenses and per-model hardware requirements.
- Add true streaming call inference with interruption, memory limits, language selection, and performance measurements.

Until STT + TTS are genuinely integrated and tested for BOTH directions, Local and Hybrid do not provide AI voice call translation.

## Privacy

The fixed Ollama URL is loopback-only. The model catalog is curated to avoid arbitrary remote-download URLs. The app does not store text previews by default and does not pass preview text to OpenAI. Cloud BYOK is not persisted. Review each model's license independently before commercialization.

## References

- Ollama API: https://docs.ollama.com/api
- OpenAI Realtime Translation: https://developers.openai.com/api/docs/guides/realtime-translation
- sherpa-onnx: https://github.com/k2-fsa/sherpa-onnx
- whisper.cpp: https://github.com/ggml-org/whisper.cpp
