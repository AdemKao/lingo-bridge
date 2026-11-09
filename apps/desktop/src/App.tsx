import { useCallback, useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./styles.css";

type Device = { id: string; name: string; kind: "input" | "output"; isDefault: boolean };
type Status = { running: boolean; model: string; hasEnvKey: boolean };
type Phase = "idle" | "connecting" | "active" | "stopping" | "stopped";
type Selection = {
  microphoneInput: string;
  lineIncomingInput: string;
  lineOutgoingOutput: string;
  headphonesOutput: string;
};
type StreamDirection = "outbound" | "inbound";
type CaptionKind = "original" | "translated";
type CaptionPayload = { direction: StreamDirection; kind: CaptionKind; delta: string };
type StatusPayload = { phase: Phase; detail: string };
type ErrorPayload = { message: string };

const initialSelection: Selection = {
  microphoneInput: "",
  lineIncomingInput: "",
  lineOutgoingOutput: "",
  headphonesOutput: "",
};

const trimSubtitle = (value: string) => value.slice(-1200);

function DevicePicker({
  title, help, devices, kind, value, onChange, disabled,
}: {
  title: string; help: string; devices: Device[]; kind: Device["kind"];
  value: string; onChange: (id: string) => void; disabled: boolean;
}) {
  return (
    <label className="device-field">
      <span className="device-title">{title}</span>
      <small>{help}</small>
      <select disabled={disabled} value={value} onChange={(e) => onChange(e.target.value)}>
        <option value="">選擇{kind === "input" ? "輸入" : "輸出"}裝置…</option>
        {devices.filter((device) => device.kind === kind).map((device) => (
          <option key={device.id} value={device.id}>
            {device.name}{device.isDefault ? " · 系統預設" : ""}
          </option>
        ))}
      </select>
    </label>
  );
}

function SubtitleCard({ title, original, translated }: {
  title: string; original: string; translated: string;
}) {
  return (
    <article className="subtitle-card">
      <h3>{title}</h3>
      <div className="subtitle-line"><span>原文</span><p>{original || "尚無語音字幕"}</p></div>
      <div className="subtitle-line translated"><span>翻譯</span><p>{translated || "等待翻譯結果…"}</p></div>
    </article>
  );
}

export default function App() {
  const [devices, setDevices] = useState<Device[]>([]);
  const [selection, setSelection] = useState<Selection>(initialSelection);
  const [status, setStatus] = useState<Status | null>(null);
  const [phase, setPhase] = useState<Phase>("idle");
  const [detail, setDetail] = useState("尚未開始");
  const [error, setError] = useState("");
  const [apiKey, setApiKey] = useState("");
  const [consent, setConsent] = useState(false);
  const [busy, setBusy] = useState(false);
  const [subtitles, setSubtitles] = useState<Record<StreamDirection, Record<CaptionKind, string>>>({
    outbound: { original: "", translated: "" },
    inbound: { original: "", translated: "" },
  });

  const refresh = useCallback(async () => {
    try {
      const [audioDevices, runtime] = await Promise.all([
        invoke<Device[]>("list_audio_devices"),
        invoke<Status>("get_runtime_status"),
      ]);
      setDevices(audioDevices);
      setStatus(runtime);
      setPhase(runtime.running ? "active" : "idle");
      setError("");
      setSelection((previous) => ({
        ...previous,
        microphoneInput: previous.microphoneInput ||
          audioDevices.find((device) => device.kind === "input" && device.isDefault)?.id || "",
        headphonesOutput: previous.headphonesOutput ||
          audioDevices.find((device) => device.kind === "output" && device.isDefault)?.id || "",
      }));
    } catch (reason) {
      setError("無法讀取原生音訊裝置。請以 Tauri 桌面模式啟動；" + String(reason));
    }
  }, []);

  useEffect(() => { void refresh(); }, [refresh]);

  useEffect(() => {
    let alive = true;
    const subscriptions: Array<() => void> = [];
    Promise.all([
      listen<StatusPayload>("lingo-status", (event) => {
        if (!alive) return;
        setPhase(event.payload.phase);
        setDetail(event.payload.detail);
        if (event.payload.phase === "stopped") setBusy(false);
      }),
      listen<ErrorPayload>("lingo-error", (event) => {
        if (alive) setError(event.payload.message);
      }),
      listen<CaptionPayload>("lingo-caption", (event) => {
        if (!alive) return;
        const { direction, kind, delta } = event.payload;
        if ((direction !== "inbound" && direction !== "outbound") ||
            (kind !== "original" && kind !== "translated")) return;
        setSubtitles((current) => ({
          ...current,
          [direction]: {
            ...current[direction],
            [kind]: trimSubtitle(current[direction][kind] + delta),
          },
        }));
      }),
    ]).then((unlisteners) => {
      if (alive) subscriptions.push(...unlisteners);
      else unlisteners.forEach((unsubscribe) => unsubscribe());
    }).catch(() => {});
    return () => {
      alive = false;
      subscriptions.forEach((unsubscribe) => unsubscribe());
    };
  }, []);

  const setDevice = (key: keyof Selection) => (id: string) =>
    setSelection((current) => ({ ...current, [key]: id }));

  const selectionsValid = useMemo(() =>
    Object.values(selection).every(Boolean) &&
    selection.microphoneInput !== selection.lineIncomingInput &&
    selection.lineOutgoingOutput !== selection.headphonesOutput,
    [selection],
  );
  const inCall = phase === "connecting" || phase === "active" || phase === "stopping";
  const ready = selectionsValid && consent && (status?.hasEnvKey || apiKey.trim()) && !inCall && !busy;

  async function start() {
    setError("");
    setBusy(true);
    setSubtitles({
      outbound: { original: "", translated: "" },
      inbound: { original: "", translated: "" },
    });
    try {
      await invoke("start_translation", {
        selection, apiKey: apiKey.trim() || null, consentConfirmed: consent,
      });
      setApiKey(""); // BYOK remains in native worker memory only for this session.
      setPhase("connecting");
      setDetail("正在初始化音訊裝置…");
    } catch (reason) {
      setError(String(reason));
      setPhase("idle");
    } finally {
      setBusy(false);
    }
  }

  async function stop() {
    setBusy(true);
    try {
      await invoke("stop_translation");
      setPhase("stopping");
    } catch (reason) {
      setError(String(reason));
      setBusy(false);
    }
  }

  async function playTestTone() {
    setBusy(true);
    setError("");
    try {
      await invoke("test_outbound_tone", { lineOutgoingOutput: selection.lineOutgoingOutput });
      setDetail("測試音已送到 LINE 虛擬麥克風輸出裝置；請向對方確認是否聽見。");
    } catch (reason) {
      setError(String(reason));
    } finally {
      setBusy(false);
    }
  }

  return (
    <main className="page">
      <header className="topbar">
        <div className="brand"><span className="brand-icon" aria-hidden="true">◎</span>
          <div><strong>Lingo Bridge</strong><small>雙向 AI 語音口譯 · Windows / macOS</small></div>
        </div>
        <span className="version-pill">Rust + Tauri 2</span>
      </header>

      <section className="hero">
        <span className="eyebrow">CHINESE ↔ JAPANESE · LIVE INTERPRETER</span>
        <h1>你說中文，<br /><span>讓對方聽見日文。</span></h1>
        <p>日本端不用安裝任何程式，繼續接聽普通 LINE 通話。你這邊安裝 Lingo Bridge，並設定兩組虛擬音訊線路，讓 AI 在中間處理聲音。</p>
        <div className="status-line">
          <span className={phase === "active" ? "live-dot active" : "live-dot"} />
          <strong>{phase === "active" ? "翻譯中" : phase === "connecting" ? "連線中" : phase === "stopping" ? "停止中" : "未啟用翻譯"}</strong>
          <span>{detail}</span>
        </div>
      </section>

      {error && <div className="error" role="alert">{error}</div>}

      <div className="layout">
        <section className="card">
          <div className="card-head">
            <div><span className="eyebrow">01 · AUDIO ROUTING</span><h2>裝置設定</h2></div>
            <button className="secondary compact" type="button" onClick={() => void refresh()} disabled={inCall || busy}>重新整理</button>
          </div>
          <p className="hint">需要兩組彼此隔離的虛擬音訊 Cable，避免把對方的日文或你的中文原音重複送入翻譯。</p>
          <div className="devices">
            <DevicePicker title="① 我的實體麥克風" help="你講中文的來源" kind="input" devices={devices} value={selection.microphoneInput} disabled={inCall} onChange={setDevice("microphoneInput")} />
            <DevicePicker title="② LINE 接收音訊（Cable B）" help="LINE 揚聲器設為 Cable B 的播放端；這裡選 Cable B 的錄音端" kind="input" devices={devices} value={selection.lineIncomingInput} disabled={inCall} onChange={setDevice("lineIncomingInput")} />
            <DevicePicker title="③ 送出日文到 LINE（Cable A）" help="這裡選 Cable A 的播放端；LINE 麥克風選 Cable A 的錄音端" kind="output" devices={devices} value={selection.lineOutgoingOutput} disabled={inCall} onChange={setDevice("lineOutgoingOutput")} />
            <DevicePicker title="④ 我的耳機" help="翻譯後的中文語音，只在你的耳機播放" kind="output" devices={devices} value={selection.headphonesOutput} disabled={inCall} onChange={setDevice("headphonesOutput")} />
          </div>
          <div className="test-panel">
            <div><strong>LINE 音訊路由測試</strong><small>先用原生 LINE 通話，確認日本端能聽見測試音。此按鈕不會錄音或使用 API。</small></div>
            <button className="secondary" type="button" disabled={!selection.lineOutgoingOutput || inCall || busy} onClick={() => void playTestTone()}>播放 0.5 秒測試音</button>
          </div>
        </section>

        <section className="card">
          <span className="eyebrow">02 · MODEL & CREDENTIALS</span><h2>翻譯服務設定</h2>
          <div className="provider"><div><strong>OpenAI Realtime Translation</strong><small>{status?.model || "gpt-realtime-translate"}</small></div><span>雙向語音 + 字幕</span></div>
          <label className="device-field key-field">
            <span className="device-title">OpenAI API Key（BYOK）</span>
            <small>只在本次通話期間由 Rust 在記憶體中使用；預設不儲存。</small>
            <input type="password" autoComplete="off" spellCheck={false} placeholder={status?.hasEnvKey ? "已偵測到 .env，可留空" : "sk-... 輸入自己的 Key"} value={apiKey} disabled={inCall} onChange={(event) => setApiKey(event.target.value)} />
          </label>
          <p className="hint">開發者也可以在 Repo 根目錄的 <code>.env</code> 設定 <code>OPENAI_API_KEY</code>；使用者輸入的 Key 會優先使用，並且不寫入檔案。</p>
          <div className="flow">
            <div><span>本機麥克風</span><strong>中文 → 日文</strong><small>→ LINE 的虛擬麥克風</small></div>
            <div><span>LINE 對方音訊</span><strong>日文 → 中文</strong><small>→ 本機耳機與字幕</small></div>
          </div>
          <label className="consent">
            <input type="checkbox" checked={consent} disabled={inCall} onChange={(event) => setConsent(event.target.checked)} />
            <span>我確認通話雙方都知悉並同意：聲音將透過網路送往 AI 翻譯服務處理，可能產生 API 費用。</span>
          </label>
          <div className="controls">
            {inCall ? (
              <button className="danger" type="button" disabled={busy || phase === "stopping"} onClick={() => void stop()}>停止翻譯</button>
            ) : (
              <button className="primary" type="button" disabled={!ready} onClick={() => void start()}>開始雙向即時翻譯</button>
            )}
          </div>
          {!selectionsValid && <p className="notice">請先選擇四個音訊端點，並確保輸入／輸出的裝置各自獨立。</p>}
          <p className="disclaimer">不會自動設定 LINE 音訊裝置。第一版需手動在 LINE 桌面版選擇 Cable A 麥克風及 Cable B 揚聲器；與 LINE 的實際相容性需要真機測試。</p>
        </section>
      </div>

      <section className="caption-section">
        <div className="card-head"><div><span className="eyebrow">03 · LIVE SUBTITLES</span><h2>雙向即時字幕</h2></div>
          <button className="secondary compact" type="button" onClick={() => setSubtitles({
            outbound: { original: "", translated: "" },
            inbound: { original: "", translated: "" },
          })}>清除字幕</button>
        </div>
        <div className="caption-grid">
          <SubtitleCard title="你 → 日本對方" original={subtitles.outbound.original} translated={subtitles.outbound.translated} />
          <SubtitleCard title="日本對方 → 你" original={subtitles.inbound.original} translated={subtitles.inbound.translated} />
        </div>
      </section>
      <footer>原生 LINE 通話整合屬於非官方 OS 音訊路由。請使用耳機、取得雙方同意並確認當地通話錄音與資料處理規範。本軟體不隸屬 LINE。</footer>
    </main>
  );
}
