import { useCallback, useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./styles.css";
import appPackage from "../package.json";

type Device = { id: string; name: string; kind: "input" | "output"; isDefault: boolean };
type Status = { running: boolean; model: string; hasEnvKey: boolean };
type Phase = "idle" | "connecting" | "active" | "stopping" | "stopped";
type Engine = "cloud" | "local" | "hybrid";
type Locale = "zh" | "ja" | "en" | "ko" | "fr" | "es";
type Target = Locale | "original";
type Direction = { source: Locale; target: Target };
type Languages = { outbound: Direction; inbound: Direction };
type Selection = {
  microphoneInput: string;
  lineIncomingInput: string;
  lineOutgoingOutput: string;
  headphonesOutput: string;
};
type Caption = "original" | "translated";
type EventDirection = "inbound" | "outbound";
type CaptionPayload = { direction: EventDirection; kind: Caption; delta: string };
type StatusPayload = { phase: Phase; detail: string };
type ModelInfo = {
  id: string;
  name: string;
  sizeNote: string;
  sourceUrl: string;
  licenseUrl: string;
  installed: boolean;
};
type LocalStatus = { available: boolean; message: string; models: ModelInfo[] };
type ModelProgress = { model: string; status: string; percent: number | null };
type InputLevel = { rms: number; peak: number; sampleCount: number; sampleRate: number; durationMs: number };

const locales: Array<{ id: Locale; label: string }> = [
  { id: "zh", label: "中文（繁體）" },
  { id: "ja", label: "日本語" },
  { id: "en", label: "English" },
  { id: "ko", label: "한국어" },
  { id: "fr", label: "Français" },
  { id: "es", label: "Español" },
];
const initialDevices: Selection = {
  microphoneInput: "", lineIncomingInput: "", lineOutgoingOutput: "", headphonesOutput: "",
};
const initialLanguages: Languages = {
  outbound: { source: "zh", target: "ja" },
  inbound: { source: "ja", target: "zh" },
};
const needsTranslation = (d: Direction) => d.target !== "original" && d.target !== d.source;
const actualTarget = (d: Direction) => needsTranslation(d) ? d.target : null;
const displayRoute = (d: Direction) =>
  needsTranslation(d) ? locales.find((l) => l.id === d.target)?.label : "不翻譯（原音）";

function DevicePicker({
  title, help, devices, kind, value, onChange, disabled,
}: {
  title: string; help: string; devices: Device[]; kind: Device["kind"];
  value: string; onChange: (id: string) => void; disabled: boolean;
}) {
  return <label className="device-field">
    <span className="device-title">{title}</span><small>{help}</small>
    <select value={value} disabled={disabled} onChange={(event) => onChange(event.target.value)}>
      <option value="">選擇{kind === "input" ? "輸入" : "輸出"}裝置…</option>
      {devices.filter((d) => d.kind === kind).map((d) =>
        <option key={d.id} value={d.id}>{d.name}{d.isDefault ? " · 系統預設" : ""}</option>)}
    </select>
  </label>;
}

function DirectionPicker({
  title, help, route, onChange, disabled,
}: {
  title: string; help: string; route: Direction; onChange: (value: Direction) => void; disabled: boolean;
}) {
  return <div className="route-editor">
    <h3>{title}</h3><p className="hint">{help}</p>
    <div className="route-inputs">
      <label className="device-field">
        <span className="device-title">說話語言</span>
        <select value={route.source} disabled={disabled}
          onChange={(event) => onChange({ ...route, source: event.target.value as Locale })}>
          {locales.map((l) => <option key={l.id} value={l.id}>{l.label}</option>)}
        </select>
      </label>
      <label className="device-field">
        <span className="device-title">對方／自己聽到</span>
        <select value={route.target} disabled={disabled}
          onChange={(event) => onChange({ ...route, target: event.target.value as Target })}>
          <option value="original">不翻譯（傳送原音）</option>
          {locales.map((l) => <option key={l.id} value={l.id}>{l.label}</option>)}
        </select>
      </label>
    </div>
    <div className="route-note">{needsTranslation(route) ? "AI 翻譯為：" + displayRoute(route) : "此方向不會呼叫 AI，會直接傳送原始語音。"}</div>
  </div>;
}

function SubtitleCard({ title, original, translated }: {
  title: string; original: string; translated: string;
}) {
  return <article className="subtitle-card">
    <h3>{title}</h3>
    <div className="subtitle-line"><span>辨識原文</span><p>{original || "尚無語音字幕"}</p></div>
    <div className="subtitle-line translated">
      <span>翻譯字幕</span><p>{translated || "等待翻譯結果…"}</p>
    </div>
  </article>;
}

export default function App() {
  const [devices, setDevices] = useState<Device[]>([]);
  const [selection, setSelection] = useState<Selection>(initialDevices);
  const [languages, setLanguages] = useState<Languages>(initialLanguages);
  const [status, setStatus] = useState<Status | null>(null);
  const [engine, setEngine] = useState<Engine>("cloud");
  const [phase, setPhase] = useState<Phase>("idle");
  const [detail, setDetail] = useState("尚未開始");
  const [error, setError] = useState("");
  const [apiKey, setApiKey] = useState("");
  const [consent, setConsent] = useState(false);
  const [busy, setBusy] = useState(false);
  const [levelMessage, setLevelMessage] = useState("");
  const [subtitles, setSubtitles] = useState<Record<EventDirection, Record<Caption, string>>>({
    outbound: { original: "", translated: "" },
    inbound: { original: "", translated: "" },
  });
  const [localStatus, setLocalStatus] = useState<LocalStatus | null>(null);
  const [modelBusy, setModelBusy] = useState(false);
  const [modelMessage, setModelMessage] = useState("");
  const [modelProgress, setModelProgress] = useState<ModelProgress | null>(null);
  const [previewModel, setPreviewModel] = useState("qwen3:4b");
  const [previewSource, setPreviewSource] = useState<Locale>("zh");
  const [previewTarget, setPreviewTarget] = useState<Locale>("ja");
  const [previewText, setPreviewText] = useState("你好，明天方便開會嗎？");
  const [previewResult, setPreviewResult] = useState("");

  const refresh = useCallback(async () => {
    try {
      const [result, runtime] = await Promise.all([
        invoke<Device[]>("list_audio_devices"),
        invoke<Status>("get_runtime_status"),
      ]);
      setDevices(result);
      setStatus(runtime);
      setPhase(runtime.running ? "active" : "idle");
      setSelection((previous) => ({
        ...previous,
        microphoneInput: previous.microphoneInput ||
          result.find((d) => d.kind === "input" && d.isDefault)?.id || "",
        headphonesOutput: previous.headphonesOutput ||
          result.find((d) => d.kind === "output" && d.isDefault)?.id || "",
      }));
    } catch (reason) {
      setError("無法讀取原生音訊裝置。請使用 Tauri 桌面模式：" + String(reason));
    }
  }, []);
  const refreshModels = useCallback(async () => {
    try {
      const result = await invoke<LocalStatus>("get_local_model_status");
      setLocalStatus(result);
      setModelMessage(result.message);
    } catch (reason) {
      setModelMessage(String(reason));
    }
  }, []);
  useEffect(() => { void refresh(); void refreshModels(); }, [refresh, refreshModels]);
  useEffect(() => {
    let alive = true;
    const unlisteners: Array<() => void> = [];
    Promise.all([
      listen<StatusPayload>("lingo-status", (event) => {
        if (!alive) return;
        setPhase(event.payload.phase);
        setDetail(event.payload.detail);
        if (event.payload.phase === "stopped") setBusy(false);
      }),
      listen<{ message: string }>("lingo-error", (event) => {
        if (alive) setError(event.payload.message);
      }),
      listen<CaptionPayload>("lingo-caption", (event) => {
        if (!alive) return;
        const { direction, kind, delta } = event.payload;
        if ((direction !== "outbound" && direction !== "inbound") ||
            (kind !== "original" && kind !== "translated")) return;
        setSubtitles((current) => ({
          ...current,
          [direction]: {
            ...current[direction],
            [kind]: (current[direction][kind] + delta).slice(-1200),
          },
        }));
      }),
      listen<ModelProgress>("lingo-model-progress", (event) => {
        if (alive) setModelProgress(event.payload);
      }),
    ]).then((subs) => {
      if (alive) unlisteners.push(...subs);
      else subs.forEach((sub) => sub());
    }).catch(() => {});
    return () => { alive = false; unlisteners.forEach((sub) => sub()); };
  }, []);

  const inCall = phase === "connecting" || phase === "active" || phase === "stopping";
  const selectedOkay = useMemo(() =>
    Object.values(selection).every(Boolean) &&
    selection.microphoneInput !== selection.lineIncomingInput &&
    selection.lineOutgoingOutput !== selection.headphonesOutput,
    [selection],
  );
  const needsAi = needsTranslation(languages.outbound) || needsTranslation(languages.inbound);
  const unsupportedVoiceEngine = needsAi && engine !== "cloud";
  const hasKey = Boolean(status?.hasEnvKey || apiKey.trim());
  const ready = selectedOkay && consent && !inCall && !busy &&
    !unsupportedVoiceEngine && (!needsAi || hasKey);

  const updateDevice = (key: keyof Selection) => (value: string) =>
    setSelection((current) => ({ ...current, [key]: value }));
  const updateRoute = (key: keyof Languages) => (route: Direction) =>
    setLanguages((current) => ({ ...current, [key]: route }));

  async function start() {
    setError("");
    setBusy(true);
    setSubtitles({
      outbound: { original: "", translated: "" },
      inbound: { original: "", translated: "" },
    });
    try {
      await invoke("start_translation", {
        selection,
        languages: {
          outbound: { source: languages.outbound.source, target: actualTarget(languages.outbound) },
          inbound: { source: languages.inbound.source, target: actualTarget(languages.inbound) },
        },
        engineMode: engine,
        apiKey: needsAi ? apiKey.trim() || null : null,
        consentConfirmed: consent,
      });
      setApiKey("");
      setPhase("connecting");
      setDetail("正在初始化音訊路由…");
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
  async function testTone() {
    setBusy(true);
    setError("");
    try {
      await invoke("test_outbound_tone", { lineOutgoingOutput: selection.lineOutgoingOutput });
      setDetail("測試音已送到虛擬麥克風；請向 LINE 通話對方確認。");
    } catch (reason) {
      setError(String(reason));
    } finally {
      setBusy(false);
    }
  }

  async function checkInputLevel(kind: "microphoneInput" | "lineIncomingInput") {
    const inputId = selection[kind];
    const label = kind === "microphoneInput" ? "實體麥克風" : "LINE 接收線路（Cable B）";
    setBusy(true);
    setError("");
    setLevelMessage(label + "：正在測試約 1 秒…");
    try {
      const level = await invoke<InputLevel>("check_audio_input_level", { inputId });
      const result = level.rms < 0.003 ? "接近靜音，請確認是否有聲音進入" : "已偵測到聲音";
      setLevelMessage(label + "：" + result + " · RMS " +
        (level.rms * 100).toFixed(1) + "% · Peak " +
        (level.peak * 100).toFixed(1) + "% · " + level.durationMs + "ms");
    } catch (reason) {
      setLevelMessage(label + " 測試失敗：" + String(reason));
    } finally {
      setBusy(false);
    }
  }

  async function installModel(model: ModelInfo) {
    if (!window.confirm("下載 " + model.name + " (" + model.sizeNote + ")？\n請先閱讀授權條款：" + model.licenseUrl + "\n下載後將儲存在 Ollama 的本地模型目錄。")) return;
    setModelBusy(true);
    setModelProgress({ model: model.id, status: "準備下載", percent: null });
    try {
      await invoke("download_local_model", { model: model.id, confirmed: true });
      setModelMessage(model.name + " 安裝完成");
      await refreshModels();
    } catch (reason) {
      setModelMessage("下載失敗：" + String(reason));
    } finally {
      setModelBusy(false);
      setModelProgress(null);
    }
  }
  async function removeModel(model: ModelInfo) {
    if (!window.confirm("確定刪除已安裝的 " + model.name + "？這也可能影響其他使用 Ollama 的程式。")) return;
    setModelBusy(true);
    try {
      await invoke("delete_local_model", { model: model.id, confirmed: true });
      setModelMessage(model.name + " 已刪除");
      await refreshModels();
    } catch (reason) {
      setModelMessage("刪除失敗：" + String(reason));
    } finally {
      setModelBusy(false);
    }
  }
  async function previewTranslation() {
    setModelBusy(true);
    setPreviewResult("");
    try {
      const result = await invoke<string>("translate_local_text", {
        model: previewModel, source: previewSource, target: previewTarget, text: previewText,
      });
      setPreviewResult(result);
    } catch (reason) {
      setPreviewResult("本地翻譯未完成：" + String(reason));
    } finally {
      setModelBusy(false);
    }
  }

  return <main className="page">
    <header className="topbar">
      <div className="brand"><span className="brand-icon" aria-hidden="true">◎</span>
        <div><strong>Lingo Bridge</strong><small>多語言語音口譯 · Windows / macOS</small></div></div>
      <span className="version-pill">v{appPackage.version} · Alpha</span>
    </header>
    <section className="hero">
      <span className="eyebrow">BIDIRECTIONAL INTERPRETER · LANGUAGE ROUTING</span>
      <h1>說自己的語言，<br /><span>讓對方聽見想聽的語言。</span></h1>
      <p>每個通話方向都可以獨立選擇語言，或選擇不翻譯。LINE 使用原本的原生通話，只有本機端需要設定音訊路由。</p>
      <div className="status-line">
        <span className={phase === "active" ? "live-dot active" : "live-dot"} />
        <strong>{phase === "active" ? "音訊處理中" : inCall ? "連線／停止中" : "尚未啟動"}</strong>
        <span>{detail}</span>
      </div>
    </section>
    {error && <div className="error" role="alert">{error}</div>}
    <div className="layout">
      <section className="card">
        <div className="card-head"><div><span className="eyebrow">01 · LANGUAGE ROUTING</span><h2>雙向語言設定</h2></div></div>
        <DirectionPicker title="你 → 對方" help="你說的語言、希望對方聽到的語言" route={languages.outbound} onChange={updateRoute("outbound")} disabled={inCall} />
        <DirectionPicker title="對方 → 你" help="對方說的語言、你希望聽到的語言" route={languages.inbound} onChange={updateRoute("inbound")} disabled={inCall} />
        <p className="hint">同語言或選擇「不翻譯」，都會傳送原音，不呼叫翻譯模型。變更語言前需要先停止目前的通話翻譯。</p>
      </section>
      <section className="card">
        <div className="card-head"><div><span className="eyebrow">02 · AUDIO DEVICES</span><h2>LINE 音訊路由</h2></div>
          <button className="secondary compact" disabled={inCall || busy} onClick={() => void refresh()}>重新整理</button></div>
        <p className="hint">建議兩條獨立的虛擬音訊 Cable。Cable A 送聲音給 LINE；Cable B 取回 LINE 對方的聲音。</p>
        <div className="devices">
          <DevicePicker title="① 我的實體麥克風" help="本機說話來源" devices={devices} kind="input" value={selection.microphoneInput} disabled={inCall} onChange={updateDevice("microphoneInput")} />
          <DevicePicker title="② LINE 接收音訊（Cable B）" help="LINE 喇叭設 Cable B 播放端；此處選錄音端" devices={devices} kind="input" value={selection.lineIncomingInput} disabled={inCall} onChange={updateDevice("lineIncomingInput")} />
          <DevicePicker title="③ 送聲音到 LINE（Cable A）" help="LINE 麥克風設 Cable A 錄音端；此處選播放端" devices={devices} kind="output" value={selection.lineOutgoingOutput} disabled={inCall} onChange={updateDevice("lineOutgoingOutput")} />
          <DevicePicker title="④ 我的耳機" help="只在本機播放對方的聲音或翻譯聲" devices={devices} kind="output" value={selection.headphonesOutput} disabled={inCall} onChange={updateDevice("headphonesOutput")} />
        </div>
        <div className="test-panel">
          <div><strong>無 API 的音訊測試</strong><small>先在 LINE 原生通話中確認對方能聽見測試音。</small></div>
          <button className="secondary" disabled={!selection.lineOutgoingOutput || inCall || busy} onClick={() => void testTone()}>播放 0.5 秒測試音</button>
        </div>
        <div className="input-diagnostics">
          <div><strong>音訊輸入偵測（本機）</strong><small>手動量測約一秒的音量，不保存聲音，也不傳送到雲端。LINE 通話時請先告知對方。</small></div>
          <div className="diag-buttons">
            <button className="secondary compact" type="button" disabled={!selection.microphoneInput || inCall || busy}
              onClick={() => void checkInputLevel("microphoneInput")}>測試我的麥克風</button>
            <button className="secondary compact" type="button" disabled={!selection.lineIncomingInput || inCall || busy}
              onClick={() => void checkInputLevel("lineIncomingInput")}>測試 LINE 接收音訊</button>
          </div>
          {levelMessage && <p className="diag-result" role="status">{levelMessage}</p>}
        </div>
      </section>
    </div>

    <section className="card engine-section">
      <span className="eyebrow">03 · AI ENGINE</span><h2>翻譯引擎</h2>
      <div className="engine-options" role="group" aria-label="選擇 AI 模式">
        {([
          ["cloud", "Cloud 雲端", "OpenAI BYOK · 即時語音"],
          ["local", "Local 本地", "Ollama 文字翻譯預覽"],
          ["hybrid", "Hybrid 混合", "語音整合仍在開發"],
        ] as const).map(([value, label, note]) =>
          <button key={value} type="button" className={"engine-choice " + (engine === value ? "selected" : "")}
            disabled={inCall} onClick={() => setEngine(value)}>
            <strong>{label}</strong><small>{note}</small>
          </button>)}
      </div>
      {engine === "cloud" && <div className="key-area">
        <label className="device-field key-field">
          <span className="device-title">OpenAI API Key（BYOK）</span>
          <small>翻譯方向啟用時才會使用金鑰；使用者的 Key 不儲存。</small>
          <input type="password" autoComplete="off" spellCheck={false} value={apiKey}
            disabled={inCall} placeholder={status?.hasEnvKey ? "已偵測到 .env，可留空" : "sk-..."}
            onChange={(e) => setApiKey(e.target.value)} />
        </label>
        <p className="hint">亦可用專案根目錄 .env 設定 OPENAI_API_KEY。若兩個方向都不翻譯，不需要 API Key，也不會呼叫 OpenAI。</p>
      </div>}
      {(engine === "local" || engine === "hybrid") && <p className="notice">
        本地引擎目前只開放「文字翻譯預覽」與模型管理；離線 STT/TTS 尚未接通通話，因此有翻譯需求時不能啟動本地語音翻譯。原音直通仍可使用。
      </p>}
      <label className="consent">
        <input type="checkbox" checked={consent} disabled={inCall} onChange={(e) => setConsent(e.target.checked)} />
        <span>我已告知通話雙方音訊將經過本機處理；選擇雲端翻譯時，雙方也同意聲音送往 OpenAI 並可能產生費用。</span>
      </label>
      {unsupportedVoiceEngine && <p className="notice">目前所選引擎尚不能進行離線語音翻譯；請改選 Cloud 或將兩個方向設為不翻譯。</p>}
      {!selectedOkay && <p className="notice">請先設定四個不同用途的音訊端點，確保兩個輸入與兩個輸出各自分離。</p>}
      <div className="controls">
        {inCall
          ? <button className="danger" disabled={busy || phase === "stopping"} onClick={() => void stop()}>停止音訊處理</button>
          : <button className="primary" disabled={!ready} onClick={() => void start()}>{needsAi ? "開始雙向語音翻譯" : "開始原音直通（無 AI）"}</button>}
      </div>
      <p className="disclaimer">Tauri 原生程式已具備音訊路由與雲端串流程式碼；LINE 真實通話仍需要在 Windows/macOS 真機測試。不能假設所有 LINE 版本可用。</p>
    </section>

    <section className="card engine-section">
      <div className="card-head"><div><span className="eyebrow">04 · LOCAL MODEL MANAGER</span><h2>本地模型管理 / 文字翻譯測試</h2></div>
        <button className="secondary compact" disabled={modelBusy} onClick={() => void refreshModels()}>檢查 Ollama</button></div>
      <p className="hint">需要使用者自行安裝並啟動 Ollama。翻譯文字只送至 127.0.0.1；模型首次下載可能需要網路，之後文字推論可以離線進行。</p>
      <div className="model-runtime">
        <span className={localStatus?.available ? "live-dot active" : "live-dot"} />
        <strong>{localStatus?.available ? "Ollama 已連線" : "Ollama 尚未連線"}</strong>
        <small>{modelMessage || "正在偵測本地模型服務…"}</small>
      </div>
      <div className="model-grid">
        {localStatus?.models.map((model) => <div className="model-card" key={model.id}>
          <div><strong>{model.name}</strong><small>{model.id} · {model.sizeNote}</small></div>
          <div className="model-links">
            <a href={model.sourceUrl} target="_blank" rel="noreferrer">來源</a>
            <a href={model.licenseUrl} target="_blank" rel="noreferrer">授權條款</a>
          </div>
          <div className="model-card-footer">
            <span className={model.installed ? "installed" : "not-installed"}>{model.installed ? "已安裝" : "尚未安裝"}</span>
            {model.installed
              ? <button className="secondary compact" disabled={modelBusy} onClick={() => void removeModel(model)}>刪除模型</button>
              : <button className="secondary compact" disabled={modelBusy || !localStatus.available} onClick={() => void installModel(model)}>確認後下載</button>}
          </div>
        </div>)}
      </div>
      {modelProgress && <p className="model-progress" role="status">
        {modelProgress.model}：{modelProgress.status}
        {modelProgress.percent !== null ? " · " + modelProgress.percent + "%" : ""}
      </p>}
      <div className="preview-editor">
        <h3>本地翻譯文字預覽</h3>
        <div className="preview-fields">
          <label className="device-field"><span className="device-title">已安裝模型</span>
            <select value={previewModel} disabled={modelBusy} onChange={(e) => setPreviewModel(e.target.value)}>
              {localStatus?.models.map((model) => <option key={model.id} value={model.id}>{model.name}</option>)}
            </select></label>
          <label className="device-field"><span className="device-title">原文語言</span>
            <select value={previewSource} disabled={modelBusy} onChange={(e) => setPreviewSource(e.target.value as Locale)}>
              {locales.map((l) => <option key={l.id} value={l.id}>{l.label}</option>)}
            </select></label>
          <label className="device-field"><span className="device-title">目標語言</span>
            <select value={previewTarget} disabled={modelBusy} onChange={(e) => setPreviewTarget(e.target.value as Locale)}>
              {locales.map((l) => <option key={l.id} value={l.id}>{l.label}</option>)}
            </select></label>
        </div>
        <textarea value={previewText} maxLength={2000} disabled={modelBusy}
          placeholder="輸入一段文字，使用本地模型翻譯…" onChange={(e) => setPreviewText(e.target.value)} />
        <button className="secondary" disabled={modelBusy || !localStatus?.models.some((m) => m.id === previewModel && m.installed) || !previewText.trim()}
          onClick={() => void previewTranslation()}>測試本地文字翻譯</button>
        {previewResult && <div className="preview-result" role="status">{previewResult}</div>}
      </div>
    </section>

    <section className="caption-section">
      <div className="card-head"><div><span className="eyebrow">05 · SUBTITLES</span><h2>即時字幕</h2></div>
        <button className="secondary compact" onClick={() => setSubtitles({
          outbound: { original: "", translated: "" },
          inbound: { original: "", translated: "" },
        })}>清除字幕</button></div>
      <div className="caption-grid">
        <SubtitleCard title="你 → 對方" {...subtitles.outbound} />
        <SubtitleCard title="對方 → 你" {...subtitles.inbound} />
      </div>
    </section>
    <footer>本工具不是 LINE 官方 Plugin。音訊處理需要雙方知情同意；本地模型文字功能不是已驗證的離線語音通話。</footer>
  </main>;
}
