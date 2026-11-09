# Packaging and distributing Lingo Bridge alpha builds

Lingo Bridge is an experimental desktop call-translation project, NOT a verified LINE
plugin. Distribute test installers only with clear limitations and informed audio consent.

## Targets

| OS | GitHub standard runner | Package | Intended users |
| --- | --- | --- | --- |
| Windows x64 | `windows-latest` | NSIS `.exe` installer | Windows 10/11 x64 |
| macOS arm64 | `macos-15` | `.dmg` | Apple Silicon Macs |
| macOS x86_64 | `macos-15-intel` | `.dmg` | Intel Macs |

The workflow is located at [build-desktop.yml](../.github/workflows/build-desktop.yml).
These are standard GitHub Actions runners, which GitHub provides free to public
repositories. Artifact storage limits still apply.

## A. Test build (no public release)

1. Open **Actions → Build Windows and macOS desktop installers (alpha)**.
2. Select **Run workflow** on `main`.
3. Leave **publish_release** unchecked.
4. Once the run succeeds, open the run page and download the three Actions
   artifacts at the bottom. Actions artifacts may require a GitHub login.
5. Confirm they contain `*.exe` for Windows and two `*.dmg` files for Mac.
6. Install and test on physical machines. Avoid publishing before verifying app launch,
   device permissions, basic audio route and unintended original-speech leaks.

## B. Prepare a GitHub prerelease for public download

1. Confirm the three platforms' builds have passed.
2. Open the same **Actions** workflow and choose **Run workflow** on `main`.
3. Tick **publish_release** and set `release_tag` to an unused tag such as
   `v0.1.0-alpha.1`.
4. The workflow builds all three installers again, then creates a
   **draft pre-release** with the installers attached only if all builds succeed.
5. Go to **Releases → Draft releases**, review warnings and downloads, then
   choose **Publish release**. This last step is intentionally manual.
6. Share https://github.com/AdemKao/lingo-bridge/releases

Releases are publicly downloadable without cloning the source repository.
GitHub Release binary assets do not use your OpenAI API key or bill your API account.

## Important distribution limitations

- **Unsigned test builds**: Windows SmartScreen may warn. macOS packages are
  ad-hoc signed, not Apple Developer ID signed or notarized; macOS may require
  the user to explicitly permit an identified download.
- **No virtual audio driver bundled**: users must separately install and configure
  two compatible independent virtual devices (Cable A + Cable B).
- **No embedded `.env` or API keys**: for Cloud mode, users supply their own API
  key at runtime. Never put developer credentials in a release.
- **Local-only voice remains incomplete**: Ollama text translation is available,
  but the sherpa ONNX STT/TTS smoke-test programs are separate Rust CLI tools,
  not a functioning local LINE call engine.
- **LINE real-call compatibility is unverified** on real Windows and Mac hardware.
- **No automatic updater**: publish each alpha version as a new prerelease.
- **No license selected**: a public GitHub repo is visible but doesn't
  automatically grant reuse or redistribution rights. Choose an appropriate
  source-code license before inviting external contributors.

## Production release gates

Before describing Lingo Bridge as a ready-to-use download, validate real two-way
calls, record supported device/OS versions, set up code signing, complete security
and privacy review, and investigate virtual device redistribution rights.
Tauri documentation: https://v2.tauri.app/distribute/pipelines/github/
