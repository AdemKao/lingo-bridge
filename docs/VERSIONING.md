# Versioning · 版本規範

**The single release identifier is** `vMAJOR.MINOR.PATCH[-prerelease]`.
For this early preview: **`v0.1.0-alpha.2`**.

## Canonical files · 必須同步

| File / 檔案 | Value |
| --- | --- |
| `Cargo.toml` → `workspace.package.version` | `0.1.0-alpha.2` |
| `apps/desktop/package.json` → `version` | `0.1.0-alpha.2` |
| `apps/desktop/src-tauri/tauri.conf.json` → `version` | `0.1.0-alpha.2` |
| GitHub Release / Tag | `v0.1.0-alpha.2` |
| `CHANGELOG.md` + `CHANGELOG.zh-TW.md` | `0.1.0-alpha.2` |
| `docs/releases/v0.1.0-alpha.2.md` | English release notes |

Run `node .github/scripts/check-version.mjs` to verify alignment.
CI and the public alpha workflow run this same check.

## Rules · 規則

- **alpha.N**: experimental, breaking or missing functionality; no stability guarantee.
  實驗版，功能及設定可能隨時變動。
- **beta.N**: core user journeys tested across supported devices but known issues remain.
  核心使用流程已完成指定裝置驗收，但仍可能有問題。
- **rc.N**: release candidate; feature freeze and regression testing.
  正式發行候選版，進行完整回歸測試。
- **Stable**: only after full end-to-end audio/LINE and privacy testing.
  須通過音訊、LINE、隱私及安裝驗證才能進入穩定版。

Each new alpha number is **immutable**. Do not overwrite an already
published Release; increment `alpha.N` instead. 發布後不可重用舊版號。

## Release checklist · 發行檢查

1. Change **all three app version files** together.
2. Update both CHANGELOG languages and create matching
   `docs/releases/vVERSION.md` (+ `.zh-TW.md`).
3. Run version check, Rust tests, frontend build, Windows/macOS CI.
4. Build all three installers; verify file count and SHA-256 manifest.
5. Tag/publish a **prerelease** with clear experimental warnings.
6. Perform actual device testing; never infer real-call success from CI.

Public alpha Releases are created only by the protected release workflow
when a relevant main-branch change is merged. For later manual releases,
use the workflow_dispatch controls documented in [releasing.md](releasing.md).
