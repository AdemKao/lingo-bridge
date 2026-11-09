//! Lingo Bridge native host. Runtime user credentials are never written to disk.

mod audio;
mod engine;
mod local;
mod realtime;
mod resample;

use audio::{DeviceInfo, DeviceSelection};
use lingo_core::{CallLanguageSettings, Language};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, State};
use tokio::sync::watch;

#[derive(Default)]
struct AppState {
    session: Mutex<Option<ActiveSession>>,
}

struct ActiveSession {
    stop: watch::Sender<bool>,
    alive: Arc<AtomicBool>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum EngineMode {
    Cloud,
    Local,
    Hybrid,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RuntimeStatus {
    running: bool,
    model: String,
    has_env_key: bool,
}

fn load_environment() {
    let _ = dotenvy::dotenv();
}

#[tauri::command]
fn get_runtime_status(state: State<'_, AppState>) -> Result<RuntimeStatus, String> {
    load_environment();
    let running = state
        .session
        .lock()
        .map_err(|_| "狀態鎖定失敗")?
        .as_ref()
        .is_some_and(|s| s.alive.load(Ordering::SeqCst));
    Ok(RuntimeStatus {
        running,
        model: configured_model(),
        has_env_key: std::env::var("OPENAI_API_KEY").is_ok_and(|value| !value.trim().is_empty()),
    })
}

fn configured_model() -> String {
    std::env::var("OPENAI_TRANSLATION_MODEL")
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| {
            !value.is_empty()
                && value
                    .chars()
                    .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
        })
        .unwrap_or_else(|| "gpt-realtime-translate".to_owned())
}

#[tauri::command]
fn list_audio_devices() -> Result<Vec<DeviceInfo>, String> {
    audio::list_devices().map_err(|error| error.to_string())
}

#[tauri::command]
fn test_outbound_tone(
    state: State<'_, AppState>,
    line_outgoing_output: String,
) -> Result<(), String> {
    if state
        .session
        .lock()
        .map_err(|_| "狀態鎖定失敗")?
        .as_ref()
        .is_some_and(|s| s.alive.load(Ordering::SeqCst))
    {
        return Err("請先停止音訊路由，再播放測試音".into());
    }
    audio::test_tone(&line_outgoing_output).map_err(|error| error.to_string())
}

#[tauri::command]
fn start_translation(
    app: AppHandle,
    state: State<'_, AppState>,
    selection: DeviceSelection,
    languages: CallLanguageSettings,
    engine_mode: EngineMode,
    api_key: Option<String>,
    consent_confirmed: bool,
) -> Result<(), String> {
    if !consent_confirmed {
        return Err("請先確認雙方了解並同意這次通話的音訊處理方式".into());
    }
    selection.validate().map_err(|error| error.to_string())?;
    load_environment();

    if languages.needs_translation() && !matches!(engine_mode, EngineMode::Cloud) {
        return Err("本地與混合模式目前僅支援文字翻譯預覽；離線 STT / TTS 尚未串接通話。可先選雲端或將語音方向設為不翻譯。".into());
    }
    let key = if languages.needs_translation() {
        let value = api_key
            .filter(|key| !key.trim().is_empty())
            .or_else(|| std::env::var("OPENAI_API_KEY").ok())
            .filter(|key| !key.trim().is_empty())
            .ok_or("請輸入自己的 OpenAI API Key，或在 .env 設定 OPENAI_API_KEY")?;
        if value.contains(['\r', '\n']) {
            return Err("API Key 格式不正確".into());
        }
        Some(value)
    } else {
        // Both directions are original audio: no key, model session, or network request.
        None
    };

    let mut guard = state.session.lock().map_err(|_| "狀態鎖定失敗")?;
    if guard
        .as_ref()
        .is_some_and(|s| s.alive.load(Ordering::SeqCst))
    {
        return Err("音訊路由已在執行中".into());
    }
    let (stop_tx, stop_rx) = watch::channel(false);
    let alive = Arc::new(AtomicBool::new(true));
    let running = alive.clone();
    let app_for_worker = app.clone();
    let model = configured_model();
    std::thread::Builder::new()
        .name("lingo-call-engine".into())
        .spawn(move || {
            let result = engine::run(app_for_worker.clone(), key, model, selection, languages, stop_rx);
            running.store(false, Ordering::SeqCst);
            if let Err(error) = result {
                let _ = app_for_worker.emit(
                    "lingo-error",
                    json!({
                        "message": format!("{error:#}").chars().take(350).collect::<String>()
                    }),
                );
            }
            let _ = app_for_worker.emit(
                "lingo-status",
                json!({"phase":"stopped","detail":"音訊處理已停止，裝置已釋放"}),
            );
        })
        .map_err(|error| format!("無法啟動音訊處理執行緒：{error}"))?;
    *guard = Some(ActiveSession {
        stop: stop_tx,
        alive,
    });
    let _ = app.emit(
        "lingo-status",
        json!({"phase":"connecting","detail":"正在初始化音訊裝置…"}),
    );
    Ok(())
}

#[tauri::command]
fn stop_translation(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    if let Some(session) = state.session.lock().map_err(|_| "狀態鎖定失敗")?.as_ref() {
        let _ = session.stop.send(true);
    }
    let _ = app.emit(
        "lingo-status",
        json!({"phase":"stopping","detail":"正在釋放音訊裝置…"}),
    );
    Ok(())
}

#[tauri::command]
async fn get_local_model_status() -> local::ModelStatus {
    local::status().await
}

#[tauri::command]
async fn translate_local_text(
    model: String,
    source: Language,
    target: Language,
    text: String,
) -> Result<String, String> {
    local::translate_text(&model, source, target, &text)
        .await
        .map_err(|error| format!("{error:#}"))
}

#[tauri::command]
async fn download_local_model(
    app: AppHandle,
    model: String,
    confirmed: bool,
) -> Result<(), String> {
    local::pull(app, &model, confirmed)
        .await
        .map_err(|error| format!("{error:#}"))
}

#[tauri::command]
async fn delete_local_model(model: String, confirmed: bool) -> Result<(), String> {
    local::remove(&model, confirmed)
        .await
        .map_err(|error| format!("{error:#}"))
}

pub fn run() {
    load_environment();
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            get_runtime_status,
            list_audio_devices,
            test_outbound_tone,
            start_translation,
            stop_translation,
            get_local_model_status,
            translate_local_text,
            download_local_model,
            delete_local_model,
        ])
        .run(tauri::generate_context!())
        .expect("failed to start Lingo Bridge");
}
