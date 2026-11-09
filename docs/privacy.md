# Privacy and consent

The v0.1 scaffold does **not** capture microphone audio, read LINE call audio, connect to AI, or save transcripts.

Before shipping audio capture:

- Show clear capture and AI-processing consent; inform the remote participant.
- Keep recording and translation **off by default**, with an obvious Stop control.
- Do not retain raw audio or transcripts by default; make retention and deletion explicit.
- Do not hardcode long-lived provider secrets in UI assets, logs or source.
- Use encrypted transport and, where available, short-lived scoped session tokens.
- Avoid logs containing audio, speech content or personal information.
- Handle failed model connections safely; don't accidentally forward the un-translated microphone signal to LINE.
- Review OS audio permissions, local recording laws, third-party provider retention and virtual audio driver licensing.

This project is independent of LINE.
