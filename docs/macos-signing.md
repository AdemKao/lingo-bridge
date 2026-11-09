# macOS Gatekeeper, signing and notarization

[English](macos-signing.md) · [繁體中文](macos-signing.zh-TW.md)

## Why do people see “Apple could not verify Lingo Bridge”?

The publicly available **alpha DMGs** are **ad-hoc signed**, not signed with an
Apple **Developer ID Application** identity and not notarized. As a result,
macOS Gatekeeper warns about an unidentified publisher. The warning is a
trust-verification problem, **not a malware scan declaring this app safe**.

**Do not advise users to disable Gatekeeper, remove quarantine attributes or
turn off macOS system security to install an experimental build.** If you
cannot verify the build and publisher, do not open it.

## Correct way to eliminate the distribution warning

For direct download outside the Mac App Store:

1. An eligible **Apple Developer Program** team/account obtains a
   **Developer ID Application** certificate.
2. Export the certificate **with its private key** as a password-protected
   PKCS#12 (`.p12`) file.
3. Store credentials in **GitHub Repository → Settings → Secrets and variables
   → Actions → New repository secret**:

| Secret | Purpose |
| --- | --- |
| `APPLE_CERTIFICATE` | Base64-encoded PKCS#12 certificate + private key |
| `APPLE_CERTIFICATE_PASSWORD` | Password chosen when exporting `.p12` |
| `APPLE_ID` | Apple Account email authorized to notarize |
| `APPLE_PASSWORD` | **App-specific password**, not the main Apple Account password |
| `APPLE_TEAM_ID` | Developer Program Team ID |

4. On the **main** branch, run GitHub Actions workflow
   [Sign and notarize macOS alpha (manual)](../.github/workflows/notarize-macos.yml).
   It builds **Apple Silicon and Intel** DMGs with Tauri using the Developer ID
   certificate, asks Apple to notarize them, and verifies the signature and
   Gatekeeper assessment.
5. Review the resulting Actions artifacts on real Mac hardware before creating
   an **incremented** prerelease containing those signed installers.

**The signing workflow does not automatically publish or replace a Release.**
This protects against incorrectly replacing an already published version.
Never commit `.p12`, password, Apple app-specific password or notarization key.

## Important details

- Apple signing/notarization requires authorized Apple Developer credentials;
  GitHub standard runners and Node.js 24 **do not substitute for notarization**.
- A free Apple Developer account used for testing generally cannot perform
  public Developer ID notarization. Follow Apple's enrollment rules.
- Do not add Apple credentials to pull-request jobs or expose their values
  in logs. The manual workflow only runs on the repository's main branch.
- Ad-hoc signed DMGs can still be used internally in controlled development,
  but should be prominently labeled as unnotarized.
- A cryptographic SHA-256 checksum verifies file integrity, **not** publisher
  identity or Apple notarization.
- Even a notarized app can contain vulnerabilities; source review, dependency
  audits, code signing and real-device testing are all still required.
- This repository's macOS App remains an **experimental LINE audio translator**,
  and actual end-to-end call operation has not yet been validated.

## References

- [Tauri v2 macOS code signing and notarization](https://v2.tauri.app/distribute/sign/macos/)
- [Tauri environment variables](https://v2.tauri.app/reference/environment-variables/)
- [Apple Developer Program](https://developer.apple.com/programs/)
