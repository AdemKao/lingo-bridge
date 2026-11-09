# 參與 Lingo Bridge 開發

[English](CONTRIBUTING.md) · **繁體中文**

歡迎協助這個實驗性桌面音訊專案。適合貢獻的內容包括小範圍的錯誤修正、
跨平台相容性測試、文件與翻譯、測試案例，以及音訊處理程式碼。

> 目前本 Repository **尚未選擇明確開源授權**。公開瀏覽不代表自動允許再利用、
> 修改或轉載程式碼。如果預計進行大量程式貢獻，請先開 Issue 討論授權及貢獻權利。

## 開 Issue 前

- 先搜尋 [既有 Issues](https://github.com/AdemKao/lingo-bridge/issues)。
- 音訊問題請提供 Windows／macOS 版本、處理器架構、LINE 電腦版版本、
  麥克風、虛擬音訊裝置及其版本。
- 請明確區分**編譯成功**、**測試音成功**、**雲端 API 連線成功**
  與**真實 LINE 雙向語音翻譯成功**。
- 不要貼出 API Key、私人對話、錄音、電話或其他個資；請使用合成測試素材。

## 本機開發

請安裝 [Tauri 系統依賴](https://v2.tauri.app/start/prerequisites/)、
Rust stable 與 Node.js 22。

```sh
git clone https://github.com/AdemKao/lingo-bridge.git
cd lingo-bridge
node .github/scripts/check-version.mjs
cargo fmt --all -- --check
cargo test -p lingo-core -p audio-core -p lingo-speech
cd apps/desktop && npm install && npm run build
```

開啟桌面開發模式：`cd apps/desktop && npm run tauri dev`。
請勿提交 `.env`、模型權重與個人通話錄音。

## Pull Request 提交檢查

- 每次 PR 專注一個修改範圍，並附上相關 Issue。
- 說明受影響的平台與測試方式。
- 音訊平台程式碼保持 Rust 模組化；UI 不得把尚未完成的功能顯示為可用。
- CI 編譯成功不代表已驗證 LINE 實際通話。
- 變更使用者功能時，同步更新 [英文 README](README.md) 與
  [繁中 README](README.zh-TW.md)，及相關技術文件。
- 使用者可見的變動請更新 [英文 CHANGELOG](CHANGELOG.md)、
  [繁中 CHANGELOG](CHANGELOG.zh-TW.md)；必要時同步 Release Notes。
- 更新版號請遵守 [版本規範](docs/VERSIONING.md)。
- 不得提交 API 金鑰，也不得在日誌或安裝包包含敏感資訊。
- 涉及音訊擷取與第三方 AI，應描述隱私和通話同意流程。

## 貢獻其他語言

英文 README 為基準，繁體中文版會同步維護。
參考 [多語文件指引](docs/i18n/README.md) 提議日文或其他語言，
文件真的存在後才加入導覽連結。

請遵守 [社群守則](CODE_OF_CONDUCT.md)；安全漏洞請依
[安全回報指南](SECURITY.md) 私下通知，不要公開於 Issue。
