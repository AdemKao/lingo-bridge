# Publishing Lingo Bridge installers / 發行安裝包

This project uses **Semantic Versioning**. See
[VERSIONING.md](VERSIONING.md) and the English / Traditional Chinese
[CHANGELOG](../CHANGELOG.md) / [更新紀錄](../CHANGELOG.zh-TW.md).

**Current alpha: v0.1.0-alpha.1** — experimental, not validated for real LINE calls.

## Supported build targets

| OS | Standard GitHub Actions runner | Package |
| --- | --- | --- |
| Windows x64 | `windows-latest` | unsigned NSIS `.exe` |
| macOS Apple Silicon | `macos-15` | ad-hoc signed `.dmg` |
| macOS Intel | `macos-15-intel` | ad-hoc signed `.dmg` |

Public repositories can use GitHub-hosted standard runners without metered
Actions minutes, though artifact storage / feature limits still apply.

## First public alpha (automatic on approved main merge)

When the versioned release workflow / release notes are merged into **main**,
the [installer workflow](../.github/workflows/build-desktop.yml) builds all
three platforms and publishes a **public GitHub pre-release** *only after all
three builds succeed*. The release includes:

- Windows `.exe`, macOS arm64 and Intel `.dmg` files
- `SHA256SUMS.txt` checksum manifest
- Versioned English Release Notes linking to the Traditional Chinese notes

A new tag is created as `v` + the version from the aligned
`Cargo.toml`, `apps/desktop/package.json`, and
`apps/desktop/src-tauri/tauri.conf.json`.

**The workflow will not overwrite an already published version.**
For a new alpha, bump all three versions, change the two CHANGELOGs,
add new `docs/releases/vVERSION*.md` notes and merge to main.

Download location: https://github.com/AdemKao/lingo-bridge/releases

## Manual draft / test artifact process

The workflow can also be manually triggered via **Actions → Build Windows
and macOS desktop installers (alpha) → Run workflow**:

1. Leave **publish_release** unchecked to build only downloadable Actions
   artifacts. These expire after 14 days and may require login.
2. Or select **publish_release** with an unused tag matching the app version;
   it creates an **unpublished draft** after three successful builds.
3. Review the draft in GitHub Releases and manually publish it if appropriate.

Manual invocation should not reuse an existing tag or release.

## Safety and distribution

These are **unsigned/unnotarized alpha builds**, **not** approved production
installers. They do **not** contain virtual audio drivers, downloaded model
weights, API keys or `.env` files. End users configure their own audio
devices and supply their own OpenAI key (BYOK) if they choose cloud mode.

Do not claim automatic LINE integration or successful live two-way calls
until you have verified them on consenting Windows and Mac devices.
The separate offline speech CLI tools are not part of the app's live
call translation pipeline yet.

Before a production release: code signing/notarization, driver licensing,
security/privacy review, stable credential handling, installer testing,
and verified LINE compatibility are required.

## Repository sidebar (About)

The sidebar **About description and Topics are GitHub repository metadata**,
not README content. The repository owner must open the gear beside About
(or Settings → General) and set them manually. Suggested:

**Description:**
`Rust-powered experimental real-time AI voice translation for Windows & macOS calls. Cloud BYOK and offline AI research.`

**Topics:**
`rust`, `tauri`, `real-time-translation`, `voice-translation`,
`speech-to-text`, `text-to-speech`, `windows`, `macos`, `ollama`,
`offline-ai`.

The **Releases** sidebar section populates once the first public release
actually exists. No GitHub Packages are published by this project.
