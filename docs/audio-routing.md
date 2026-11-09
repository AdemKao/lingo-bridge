# Audio routing proof of concept

This document is a test plan, **not** confirmation of LINE integration.

## Windows

Explore application WASAPI loopback capture for incoming LINE audio. For outbound Japanese test audio, use a licensed virtual audio cable and select its virtual recording endpoint as LINE Desktop's microphone. Confirm the original Mandarin microphone signal never reaches LINE directly.

Microsoft: https://learn.microsoft.com/en-us/windows/win32/coreaudio/loopback-recording

## macOS

Explore CoreAudio Process Tap (version and capture permissions vary) and/or BlackHole for the incoming route. Verify LINE's microphone selection and route a synthesized voice into the virtual input. Prevent local translation playback from feeding back into capture.

BlackHole: https://github.com/ExistentialAudio/BlackHole

## Acceptance tests

1. Local Windows/macOS user calls a normal LINE mobile participant with no added app.
2. When local user speaks Chinese, remote participant hears **only Japanese** playback.
3. When remote participant speaks Japanese, local user hears **only Mandarin**, with two-line original/translated captions.
4. No self-feedback, duplicate speech or unintended original-audio leakage.
5. Verify audio remains stable for at least 10 minutes, then test reconnection and device unplug/replug.
6. Stop must disable capture and transmission promptly.

Record test conditions (OS, LINE version, input/output devices, latency, failures). Inform both participants about AI processing and obtain appropriate consent.
