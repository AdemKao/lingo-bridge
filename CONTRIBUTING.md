# Contributing to Lingo Bridge

**English** · [繁體中文](CONTRIBUTING.zh-TW.md)

Thanks for helping improve this experimental desktop audio project. We welcome
well-scoped bug reports, platform compatibility tests, documentation fixes and
code contributions.

> Before contributing: this repository currently has **no explicit open-source
> license**. Public access does not imply permission to reuse or distribute code.
> If you plan significant code contributions, first open an issue to discuss
> license and ownership expectations with the maintainer.

## Before opening an issue

- Search [existing issues](https://github.com/AdemKao/lingo-bridge/issues).
- State your OS version, CPU architecture, LINE Desktop version, microphone and
  virtual cable software versions when reporting audio bugs.
- Distinguish **compiled**, **test-tone passed**, **cloud API connected**, and
  **real two-way LINE call validated** in your report.
- Do **not** post API keys, transcripts, recordings, phone numbers or any private
  communication. Prefer reproducible synthetic audio.

## Local development

Install the [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/),
Rust stable and Node.js 22.

```sh
git clone https://github.com/AdemKao/lingo-bridge.git
cd lingo-bridge
node .github/scripts/check-version.mjs
cargo fmt --all -- --check
cargo test -p lingo-core -p audio-core -p lingo-speech
cd apps/desktop && npm install && npm run build
```

Tauri app in development: `cd apps/desktop && npm run tauri dev`.
Never commit `.env`, model weights or personal audio samples.

## Pull request checklist

- Keep changes focused and link the issue being addressed.
- Explain the affected OS and how changes were tested.
- Prefer Rust abstractions for platform audio and keep UI/engine status honest.
- Never mark real LINE calling as verified based solely on CI.
- If you change supported features, update **both** [README.md](README.md)
  and [README.zh-TW.md](README.zh-TW.md), along with the relevant docs.
- For user-visible changes, update [CHANGELOG.md](CHANGELOG.md),
  [CHANGELOG.zh-TW.md](CHANGELOG.zh-TW.md) and release notes as appropriate.
- For a release bump, follow [VERSIONING](docs/VERSIONING.md).
- Keep API credentials out of the repository, logs and build artifacts.
- Include privacy/consent considerations for audio capture and third-party AI.

## Translation contributions

English is the canonical README; Traditional Chinese is maintained in parallel.
Use [docs/i18n/README.md](docs/i18n/README.md) to propose more languages.
Only link to fully created translations — do not create dead language links.

## Conduct and security

Follow [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) and report vulnerabilities
through [SECURITY.md](SECURITY.md), **not** a public issue.
