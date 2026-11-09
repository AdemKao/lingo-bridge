<div align="center">

# Lingo Bridge

**使用 Rust 開發的桌面通話即時翻譯實驗專案**

支援 Windows／macOS 音訊橋接、雲端 AI 自備金鑰（BYOK），以及本地離線模型的研究與驗證。

[**English**](README.md) · [**繁體中文**](README.zh-TW.md) · [**協助翻譯**](docs/i18n/README.md)

[![CI](https://github.com/AdemKao/lingo-bridge/actions/workflows/ci.yml/badge.svg)](https://github.com/AdemKao/lingo-bridge/actions/workflows/ci.yml)
[![Installers](https://github.com/AdemKao/lingo-bridge/actions/workflows/build-desktop.yml/badge.svg)](https://github.com/AdemKao/lingo-bridge/actions/workflows/build-desktop.yml)
![Windows and macOS](https://img.shields.io/badge/platform-Windows%20%7C%20macOS-4069c9)
![Alpha](https://img.shields.io/badge/status-experimental%20alpha-orange)

**[⬇ 下載 Alpha 測試版](https://github.com/AdemKao/lingo-bridge/releases)** ·
**[快速開始](#快速開始)** ·
**[macOS 簽署與公證](docs/macos-signing.zh-TW.md)** ·
**[開發路線圖](docs/roadmap.md)** ·
**[問題回報](https://github.com/AdemKao/lingo-bridge/issues)** ·
**[更新紀錄](CHANGELOG.zh-TW.md)**

</div>

> [!WARNING]
> **本專案仍是 Alpha 實驗軟體，並非已驗證的正式 LINE 翻譯外掛。**
> Windows／Mac 可以成功打包，不代表真實 LINE 雙向通話已經順暢運作。
> 本專案透過作業系統的音訊裝置進行路由，不使用 LINE 私有通話介面。
> 測試時請先告知對方 AI 會處理聲音，並取得雙方同意。

## Lingo Bridge 是什麼？

我們想實現的使用情境是：**只需要說中文的一方安裝 Lingo Bridge**。
你在 Windows 或 Mac 講中文，日本使用者照常接聽 LINE，無須安裝額外工具；
理想情況下，日本端會聽到日文語音，而你會聽到中文翻譯及看到字幕。

目前已建立**雲端語音翻譯原型**，以及獨立的**本地文字模型與離線語音 CLI 實驗功能**。
LINE 實際通話的相容性與延遲仍需要真機驗證。

```text
  送出：中文麥克風 ──► 雲端語音翻譯 ──► Cable A ──► LINE ──► 日本使用者
  接收：你的中文耳機／字幕 ◄── 雲端翻譯 ◄── Cable B ◄── LINE 日文音訊
                       你的 Windows／macOS 電腦
```

## 目前功能狀態

| 項目 | 現況 |
| --- | --- |
| Windows x64、Mac Apple Silicon／Intel 安裝包 | ✅ 已驗證能打包；**未正式簽章** |
| 四組音訊裝置選擇、雙線路路由與測試音 | 🧪 原型已實作，需手動設定 |
| OpenAI 即時翻譯、字幕與使用者自備 Key | 🧪 原型已實作，尚未完成 LINE 真機驗證 |
| 雙方向獨立語言設定、可選擇原音直通 | 🧪 已實作 |
| 本地 Ollama 文字翻譯、模型下載 | 🧪 文字模式可預覽 |
| SenseVoice 中日離線 STT | 🧪 sherpa-onnx 獨立 CLI 實驗 |
| Supertonic 日文離線 TTS | 🧪 sherpa-onnx 獨立 CLI 實驗 |
| 中文離線 TTS、完整本地語音翻譯 | ⏳ 尚未串接 |
| 真實 LINE 通話相容性、延遲與回音安全 | ⏳ 尚待驗證 |
| 音訊驅動自動安裝、正式簽章、自動更新 | ⏳ 尚未提供 |

詳細請見 [Roadmap](docs/roadmap.md)、[離線語音實驗](docs/offline-speech-poc.md)
與 [更新紀錄](CHANGELOG.zh-TW.md)。

## 下載安裝

請到 **[GitHub Releases](https://github.com/AdemKao/lingo-bridge/releases)** 下載最新
**Pre-release（預覽版）**，不要把 Alpha 當成穩定產品。

| 電腦 | 下載檔案 | 注意事項 |
| --- | --- | --- |
| Windows 10／11 x64 | NSIS 安裝檔 `.exe` | 未正式簽章，可能出現 SmartScreen 提示 |
| Apple Silicon Mac | arm64 `.dmg` | 尚未通過 Apple 正式公證 |
| Intel Mac | x86_64 `.dmg` | 尚未通過 Apple 正式公證 |

**安裝包不包含虛擬音訊驅動、模型權重或 API Key。**
若 Release 尚未上架，可以到
[Actions 打包紀錄](https://github.com/AdemKao/lingo-bridge/actions/workflows/build-desktop.yml)
下載測試 Artifacts；發行步驟請看 [Release 指南](docs/releasing.md)。

## 快速開始

1. **準備兩條相互獨立的虛擬音訊線路**：Windows 可研究
   [VB-CABLE](https://vb-audio.com/Cable/index.htm)，Mac 可研究
   [BlackHole](https://github.com/ExistentialAudio/BlackHole)。
   請確認各自的商業授權與安裝權限，單一線路無法安全隔離雙方向。
2. **設定 LINE 麥克風為 Cable A 的錄音端**、LINE 喇叭為 Cable B 的播放端。
   Lingo Bridge 的日文輸出選 Cable A 播放端、LINE 日文輸入選 Cable B 錄音端；
   此外還需選你的實體麥克風及耳機。
3. **先播放 0.5 秒測試音**，確認 LINE 對方聽得到；這項測試不會消耗 API 費用。
4. **雲端語音模式**：在 App 輸入使用者自己的 OpenAI API Key（BYOK），不會
   預設存到硬碟。**本地文字模式**：先安裝
   [Ollama](https://ollama.com/download)，再透過 App 內 Model Manager 主動下載模型。
   目前本地文字模式**不等於離線即時語音通話**。
5. 在正式啟動 AI 語音處理前**取得通話雙方同意**，使用耳機並測試有無回音、
   原始語音外洩及裝置切換失敗。

完整文件：**[音訊路由](docs/audio-routing.md)** ·
**[本地模型](docs/local-models.md)** ·
**[隱私與 Key](docs/privacy.md)**

## 開發者：從原始碼執行

需要 Rust stable、Node.js 22 與 npm，並先完成
[Tauri 2 作業系統依賴](https://v2.tauri.app/start/prerequisites/)。

```sh
git clone https://github.com/AdemKao/lingo-bridge.git
cd lingo-bridge/apps/desktop
npm install
npm run tauri dev
```

僅供開發時，可複製根目錄 `.env.example` 成 `.env` 並設定 Key。
請勿提交真實金鑰，更不可隨安裝包附上已填入 Key 的 `.env`。
一般使用者直接於 App 透過 BYOK 輸入即可。

```sh
# 請從 Repository 根目錄執行
node .github/scripts/check-version.mjs
cargo fmt --all -- --check
cargo test -p lingo-core -p audio-core -p lingo-speech
cargo check -p lingo-bridge-desktop
# 另需 sherpa-onnx 原生推論工具鏈
cargo check -p lingo-speech --features sherpa --bin lingo-speech-smoke
```

## 專案架構

| 路徑 | 用途 |
| --- | --- |
| `apps/desktop` | Tauri 2 + React/TypeScript 桌面 UI |
| `apps/desktop/src-tauri` | Rust 原生核心、音訊處理、雲端翻譯 |
| `crates/lingo-core` | 語言方向與資料型別 |
| `crates/audio-core` | PCM 音訊基礎處理 |
| `crates/lingo-speech` | 離線 STT／日文 TTS 和語音切分實驗 |
| `docs/` | 音訊、模型、隱私、發行與多語文件 |

## 社群參與與版號

- [貢獻指南](CONTRIBUTING.zh-TW.md) · [社群守則](CODE_OF_CONDUCT.md) ·
  [安全政策](SECURITY.md)
- [更新紀錄](CHANGELOG.zh-TW.md) · [版本規範](docs/VERSIONING.md) ·
  [測試版 Release Notes](docs/releases/v0.1.0-alpha.1.zh-TW.md)
- [文件多語化](docs/i18n/README.md) · [English README](README.md)
- [Issues](https://github.com/AdemKao/lingo-bridge/issues) ·
  [Pull Requests](https://github.com/AdemKao/lingo-bridge/pulls)

> **授權提醒：** 目前尚未決定原始碼授權條款。即使 Repository 已公開，
> 也不代表他人自動取得重製、修改或再散布原始碼的權利。
> 第三方模型與音訊驅動另有授權。若希望其他人能正式引用或重新發行，
> 請先選擇適合的開源授權。

---

<sub>本專案為獨立實驗性專案，並非 LINE／LY Corporation 官方提供或認可的外掛。</sub>


### 離線中日雙向 WAV 語音翻譯（原始碼實驗）

新增 Rust CLI，可將 SenseVoice 語音辨識、本機 Ollama 翻譯、Supertonic 日文或 Matcha 中文 TTS 串接，輸出新 WAV。**不需要 OpenAI Key，但不代表桌面版已能離線翻譯 LINE 通話。** 模型由使用者自行下載、確認授權。參考 [離線 WAV 指南](docs/offline-wav-pipeline.md)。

### 本機音訊輸入測試

桌面介面新增「測試我的麥克風」及「測試 LINE 接收音訊（Cable B）」；使用者按下後才擷取約一秒的音量，僅顯示 RMS／峰值，不儲存錄音、也不連接 AI。可以協助確認虛擬音訊線路是否收到聲音，但**不代表已驗證 LINE 通話翻譯成功**。
