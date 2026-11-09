# Privacy, API keys and consent

## API Key modes

1. **Developer .env**: copy `.env.example` to `.env` at repo root, populate `OPENAI_API_KEY`. Rust loads the environment at startup. The file is gitignored and must never be shared.
2. **User BYOK**: the desktop UI accepts a key at runtime. It is passed to the native Rust worker process only on Start, used in HTTPS/WSS Authorization for OpenAI, and not saved to disk. The field is cleared after successfully starting a call. BYOK takes priority over .env.
3. Do **not** embed a shared developer API key in an app release or ship a populated .env. Production hosted billing needs a secure backend and short-lived credentials.

The developer can set `OPENAI_TRANSLATION_MODEL` and `OPENAI_TRANSCRIPTION_MODEL` in `.env` (defaults: `gpt-realtime-translate` and `gpt-realtime-whisper`).

The app currently offers **one provider**: OpenAI Realtime Translation. It doesn't claim compatibility with arbitrary third-party model APIs.

## Consent and handling

- Obtain informed consent from **both** call participants before starting AI processing; local UI requires confirmation.
- Capture is off by default; Start explicitly enables audio capture. Stop releases streams and requests session close.
- Live audio and transcript content is not saved on disk by Lingo Bridge. The AI service receives live audio for translation and handles it according to its own retention terms.
- React displays captions in memory, not local storage.
- No API keys are included in debug errors, versioned files or UI event payloads.
- Use separate incoming/outgoing cable devices to prevent transmitting unintended original speech.
- Understand the privacy, data processing and billing policies of your chosen API provider.
- This is not an official LINE plugin. It cannot promise the original LINE call's encryption characteristics apply to the separately streamed translation data.

## Limitations before distribution

The runtime key is not persisted (no OS keychain integration). Native installer signing, automatic permission prompts, transcript deletion controls for future storage and security audit are not yet provided.
