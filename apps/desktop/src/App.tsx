import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

type BootstrapStatus = {
  os: string;
  outbound: string;
  inbound: string;
  lineAudioCaptureReady: boolean;
  virtualMicrophoneReady: boolean;
  realtimeTranslationReady: boolean;
};

const phases = [
  ["取得 LINE 接收音訊", "Windows WASAPI / macOS CoreAudio Process Tap 或虛擬裝置"],
  ["雙向即時翻譯", "中文→日文、日本→中文，各自獨立串流與緩衝"],
  ["送出日文聲音", "將合成日文導入 LINE 選擇的虛擬麥克風"],
  ["消除回音", "確保翻譯語音不會重新被擷取翻譯"],
];

export default function App() {
  const [status, setStatus] = useState<BootstrapStatus | null>(null);
  const [message, setMessage] = useState("正在連線至 Rust 桌面核心…");

  useEffect(() => {
    invoke<BootstrapStatus>("bootstrap_status")
      .then((data) => {
        setStatus(data);
        setMessage("Rust 桌面核心已啟動");
      })
      .catch(() => {
        setMessage("瀏覽器預覽模式：請使用 npm run tauri dev 啟動桌面核心");
      });
  }, []);

  const checks = [
    ["LINE 音訊擷取", status?.lineAudioCaptureReady],
    ["虛擬麥克風路由", status?.virtualMicrophoneReady],
    ["AI 翻譯串流", status?.realtimeTranslationReady],
  ] as const;

  return (
    <main className="page">
      <header>
        <div className="brand"><span className="brand-icon">◎</span><div>
          <strong>Lingo Bridge</strong><small>Rust · Windows & macOS</small>
        </div></div>
        <span className="tag">v0.1 · Scaffold</span>
      </header>
      <section className="hero">
        <span className="eyebrow">REAL-TIME TRANSLATION · TECHNICAL PREVIEW</span>
        <h1>用自己的語言，<br /><em>與世界直接對話。</em></h1>
        <p>目標：你講中文、對方聽日文；對方講日文、你聽中文。日本使用者無須安裝任何新程式，繼續用原本的 LINE 通話。</p>
        <div className="warning"><strong>尚未提供通話翻譯。</strong> 目前版本只包含桌面視窗與 Rust 核心基礎結構；不會錄音、不會讀取 LINE 音訊，也不會連線至 AI 服務。</div>
      </section>
      <section className="panels">
        <article className="panel">
          <span className="eyebrow">LANGUAGE ROUTES</span>
          <h2>雙向翻譯路線</h2>
          <div className="route"><span className="direction">我方傳出</span><div><strong>中文 → 日文</strong><small>對方聽到日文合成語音</small></div><code>{status?.outbound ?? "zh-TW → ja-JP"}</code></div>
          <div className="route"><span className="direction incoming">對方傳入</span><div><strong>日文 → 中文</strong><small>本機播放中文並顯示字幕</small></div><code>{status?.inbound ?? "ja-JP → zh-TW"}</code></div>
          <p className="aside">以上是預計支援的音訊路徑，不代表 LINE 已經成功整合。</p>
        </article>
        <article className="panel">
          <span className="eyebrow">NATIVE RUNTIME</span>
          <h2>技術狀態</h2>
          <p className="runtime">{message}{status ? " · " + status.os : ""}</p>
          {checks.map(([label, ready]) => (
            <div className="check" key={label}>
              <span className={ready ? "dot ready" : "dot"}>{ready ? "✓" : "–"}</span>
              <strong>{label}</strong>
              <small className={ready ? "ready-text" : ""}>{ready ? "Ready" : "Not implemented"}</small>
            </div>
          ))}
          <p className="aside">功能必須在真實 LINE 雙向通話驗證成功後，才會標示為 Ready。</p>
        </article>
      </section>
      <section className="roadmap">
        <div><span className="eyebrow">NEXT MILESTONE</span><h2>先完成音訊 POC</h2><p>確認 LINE 能用虛擬麥克風收到翻譯聲音，再接上 AI 服務。</p></div>
        <div className="steps">{phases.map(([title, detail], index) => (
          <div className="step" key={title}><b>{index + 1}</b><div><strong>{title}</strong><small>{detail}</small></div></div>
        ))}</div>
      </section>
      <footer>Independent project, not affiliated with LINE. Future audio processing requires informed consent.</footer>
    </main>
  );
}
