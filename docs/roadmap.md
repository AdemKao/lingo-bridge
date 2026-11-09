# Roadmap

## v0.1 — Bootstrap

- [x] Rust workspace, language domain and audio-format primitives with tests
- [x] Tauri 2 / React / TypeScript desktop shell
- [x] GitHub CI: Rust tests, web build, Windows/macOS native checks
- [x] Architecture, routing and privacy docs
- [ ] First actual Windows/macOS dev builds and lockfiles

## v0.2 — Native audio experiment

- [ ] Enumerate audio devices and install/configure supported virtual audio cable
- [ ] Capture LINE incoming audio via WASAPI or CoreAudio
- [ ] Send prerecorded Japanese voice into LINE's virtual microphone input
- [ ] Test ordinary Japanese LINE mobile recipient (no install)
- [ ] Prevent feedback and accidental original microphone leakage

## v0.3 — Translation engine

- [ ] Two speech-to-speech streaming sessions (zh-TW → ja-JP, ja-JP → zh-TW)
- [ ] Live partial/final transcripts and subtitles
- [ ] Audio buffers, interruption handling, error recovery, latency metrics
- [ ] Secure session credentials and consent UX

## v0.4 — Pilot

- [ ] Test latency, wording quality, robustness and cost in real LINE calls
- [ ] Windows/macOS installers, signing, privacy and virtual device licensing
- [ ] Publish a supported OS/LINE version matrix

**Gate:** do not advertise native LINE compatibility until both-direction real-call tests pass.
