# 更新紀錄（CHANGELOG）

[English](CHANGELOG.md) · **繁體中文**

本文件採用 [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) 與
[語意化版本](docs/VERSIONING.md) 原則。僅記錄真正**已實作**或**已驗證能編譯打包**的項目，
如需實機驗證會另行標明。

## [未發布]

### 規劃中
- 將本地語音辨識、翻譯與合成接入 LINE 即時通話，量測延遲並處理回音。
- Windows／macOS 真機驗證 LINE 雙向通話。
- 改善虛擬音訊設定、正式簽章和自動更新。

## [0.1.0-alpha.2] - 2026-11-09

### 新增
- 桌面版加入麥克風、LINE 接收音訊線路的一秒手動 RMS／峰值音量檢查；不儲存音訊、不傳送雲端。
- 原始碼新增中文 Matcha-TTS 離線 WAV 合成。
- 新增中日雙向**音訊檔**翻譯 CLI：SenseVoice → 本機 Ollama → 中文／日文 TTS。
- Rust 測試及 Windows／macOS 選用本地模型功能的編譯驗證。

### 注意事項
- 離線語音目前仍是需自行準備模型的**命令列研究工具**，並未接入下載版的 LINE 即時語音翻譯。
- 真實 LINE 通話、延遲、回音隔離尚未通過 Windows／Mac 真機驗證。
- 沒有 Apple 付費開發者身分時，Mac 安裝包仍未公證。

## [0.1.0-alpha.1] - 2026-11-09

### 新增
- Rust／Tauri 2 + React 桌面應用原型。
- 音訊裝置選擇、雙方向獨立虛擬線路與短測試音。
- OpenAI 即時語音翻譯原型、字幕事件與使用者自備 Key（BYOK）。
- 開發用 `.env.example` 及可設定的翻譯／轉錄模型。
- 各方向語言選擇和保留原音的模式。
- Ollama 本地模型管理及離線**文字**翻譯預覽。
- sherpa-onnx 離線語音切分、SenseVoice STT／日文 TTS CLI 技術實驗。
- GitHub Actions 三平台 Alpha 打包：Windows x64、Mac Apple Silicon 與 Intel。
- 英文／繁體中文文件、貢獻規範與發行說明。

### 已知限制
- **尚未以真機完成 LINE 雙向通話翻譯驗證**。
- STT／TTS 目前是獨立 CLI 實驗，**未接入桌面即時音訊通話**。
- 尚無中文離線 TTS 與完整本地語音互譯管線。
- 安裝包未正式簽章／Apple 公證，也不包含虛擬音訊驅動。
- 使用者 Key 僅供本次工作階段使用，尚無作業系統保管庫及自動更新。

[0.1.0-alpha.1]: https://github.com/AdemKao/lingo-bridge/releases/tag/v0.1.0-alpha.1

[0.1.0-alpha.2]: https://github.com/AdemKao/lingo-bridge/releases/tag/v0.1.0-alpha.2
