# macOS 安全警告、程式簽章與 Apple 公證

[English](macos-signing.md) · [繁體中文](macos-signing.zh-TW.md)

## 為什麼顯示「Apple could not verify Lingo Bridge」？

目前公開測試用的 Mac `.dmg` 使用 **Ad-hoc（臨時）簽署**，
並非 Apple **Developer ID Application** 正式開發者簽章，亦未通過 Apple
**Notarization（公證）**。因此 macOS Gatekeeper 不能確認發布者身分。

這不代表 Apple 已經斷定程式含有惡意軟體，**也不代表程式已被證明安全**。

**不建議使用者關閉 Gatekeeper、刪除檔案安全隔離標記或停用 macOS
系統安全保護來安裝測試版。** 無法確認檔案與來源可信時應避免安裝。

## 正式解決方法

透過 Mac App Store 以外的管道發布，需要符合資格的 Apple Developer
Program 帳號／團隊，建立 **Developer ID Application** 憑證並提交公證。

在 GitHub Repository 的 **Settings → Secrets and variables → Actions**
配置：

| Secret | 用途 |
| --- | --- |
| `APPLE_CERTIFICATE` | 以 Base64 編碼的 `.p12` 簽章憑證與私鑰 |
| `APPLE_CERTIFICATE_PASSWORD` | 匯出 `.p12` 時設定的密碼 |
| `APPLE_ID` | 有權限提交公證的 Apple Account 電子郵件 |
| `APPLE_PASSWORD` | **App 專用密碼**，不是 Apple Account 一般登入密碼 |
| `APPLE_TEAM_ID` | Apple Developer Team ID |

接著在 GitHub Actions 的 **main** 分支，手動執行
[Sign and notarize macOS alpha](../.github/workflows/notarize-macos.yml)。
它會分別產生 Apple Silicon 與 Intel 的簽署、公證測試包，
並檢查 Developer ID 簽章與 Gatekeeper 評估結果。

此流程**不會自動對外發布新版本，也不會覆蓋既有 Release**。
需要先完成真機測試，再增加版本號並將簽署的安裝包納入新發行版。

### 注意事項

- GitHub Actions 使用 Node.js 24 不會替代 Apple 正式簽章與公證。
- Apple Developer 免費測試帳號通常無法完成對外發布所需的 Developer ID 公證。
- 不要把憑證、私鑰、密碼提交到 Git，也不要貼在 Issue 或日誌中。
- SHA-256 可以驗證檔案完整性，但無法取代開發者簽章或 Apple 公證。
- 即使通過 Apple 公證，仍需要安全審核與實際通話測試。
- Lingo Bridge 目前仍是尚未完成 LINE 雙向通話實機驗證的 Alpha 原型。

參考：[Tauri 官方 macOS 簽章與公證說明](https://v2.tauri.app/distribute/sign/macos/)。
