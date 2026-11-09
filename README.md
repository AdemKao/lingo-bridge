# Lingo Bridge

Rust-first desktop app for Chinese ↔ Japanese two-way call translation.

> **Status: runnable desktop scaffold, NOT a working LINE translator yet.** Native LINE audio capture, virtual microphone routing, realtime translation and captions are future milestones. Compatibility with actual LINE calls is unverified.

## Product goal

The **Taiwan/Chinese-speaking participant alone** installs Lingo Bridge on Windows or macOS and places an ordinary **LINE Desktop** call. They speak Mandarin and the remote Japanese-speaking person hears Japanese; the remote party speaks Japanese and the local user hears Mandarin and reads translated captions. The Japanese-speaking participant installs nothing.

This approach would use OS audio routing outside of LINE. It is **not** an official LINE plugin, and there is no public native LINE call audio API.

## Monorepo

- `apps/desktop`: Tauri 2 + React + TypeScript + Vite app.
- `apps/desktop/src-tauri`: Rust native desktop host, currently a status command only.
- `crates/lingo-core`: typed language routes for both directions.
- `crates/audio-core`: PCM format validation and frame-duration calculation.
- `docs/`: architecture, audio-routing experiments, privacy and roadmap.

## Prerequisites

- Rust stable with Cargo
- Node.js 20.19+ or 22.12+ and npm
- [Tauri 2 prerequisites](https://v2.tauri.app/start/prerequisites/) for Windows or macOS

## Start the desktop shell

    cd apps/desktop
    npm install
    npm run tauri dev

To preview the frontend without Rust: `npm run dev`. The native status command works only inside Tauri.

## Tests

    cargo fmt --all -- --check
    cargo test -p lingo-core -p audio-core
    cd apps/desktop
    npm run build

CI also checks the native Tauri host on Windows and macOS. Commit generated `Cargo.lock` and `apps/desktop/package-lock.json` after initial dependency resolution.

## What comes next?

1. Prove capture of incoming LINE audio and routing of a Japanese prerecorded test sound via a virtual microphone.
2. Prove isolation of source and translated audio (no feedback loops or leakage).
3. Add two independent AI translation streams and captions, with explicit consent.
4. Measure latency, compatibility, failure handling and cost with real calls.

See [architecture](docs/architecture.md), [routing experiments](docs/audio-routing.md), [roadmap](docs/roadmap.md), and [privacy](docs/privacy.md).

No open-source license has been selected. Independent project; not affiliated with or endorsed by LINE or LY Corporation.
