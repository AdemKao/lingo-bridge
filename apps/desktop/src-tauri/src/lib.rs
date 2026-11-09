//! Lingo Bridge native host.
//! The user's API key stays in the Rust process for the lifetime of the call.

mod audio;
mod engine;
mod realtime;
mod resample;

use audio::{DeviceInfo, DeviceSelection};
use serde::Serialize;
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RuntimeStatus {
    running: bool,
    model: String,
    has_env_key: bool,
}

fn load_environment() {
    // Look for .env in the working directory and parent directories during development.
    // No secret is included in the built application.
    let _ = dotenvy::dotenv();
}

#[tauri::command]
fn get_runtime_status(state: State<'_, AppState>) -> Result<RuntimeStatus, String> {
    load_environment();
    let running = state.session.lock().map_err(|_| "狀態鎖定失敗")?
        .as_ref().is_some_and(|s| s.alive.load(Ordering::SeqCst));
    Ok(RuntimeStatus {
        running,
        model: configured_model(),
        has_env_key: std::env::var("OPENAI_API_KEY").is_ok_and(|value| !value.trim().is_empty()),
    })
}

fn configured_model() -> String {
    // The dedicated translation endpoint currently supports this model.
    "gpt-realtime-translate".to_owned()
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
    if state.session.lock().map_err(|_| "狀態鎖定失敗")?
        .as_ref().is_some_and(|s| s.alive.load(Ordering::SeqCst)) {
        return Err("請先停止即時翻譯，再播放測試音".into());
    }
    audio::test_tone(&line_outgoing_output).map_err(|error| error.to_string())
}

#[tauri::command]
fn start_translation(
    app: AppHandle,
    state: State<'_, AppState>,
    selection: DeviceSelection,
    api_key: Option<String>,
    consent_confirmed: bool,
) -> Result<(), String> {
    if !consent_confirmed {
        return Err("請先確認通話雙方了解音訊會傳送到 AI 翻譯服務".into());
    }
    selection.validate().map_err(|error| error.to_string())?;
    load_environment();
    let key = api_key
        .filter(|key| !key.trim().is_empty())
        .or_else(|| std::env::var("OPENAI_API_KEY").ok())
        .filter(|key| !key.trim().is_empty())
        .ok_or("請輸入自己的 OpenAI API Key，或在 .env 設定 OPENAI_API_KEY")?;
    if key.contains(['\r', '\n']) {
        return Err("API Key 格式不正確".into());
    }

    let mut guard = state.session.lock().map_err(|_| "狀態鎖定失敗")?;
    if guard.as_ref().is_some_and(|s| s.alive.load(Ordering::SeqCst)) {
        return Err("翻譯已經在執行中".into());
    }
    let (stop_tx, stop_rx) = watch::channel(false);
    let alive = Arc::new(AtomicBool::new(true));
    let running = alive.clone();
    let app_for_worker = app.clone();
    let model = configured_model();
    std::thread::Builder::new()
        .name("lingo-translation".into())
        .spawn(move || {
            let result = engine::run(app_for_worker.clone(), key, model, selection, stop_rx);
            running.store(false, Ordering::SeqCst);
            if let Err(error) = result {
                let _ = app_for_worker.emit("lingo-error", json!({
                    "message": format!("{error:#}").chars().take(350).collect::<String>()
                }));
            }
            let _ = app_for_worker.emit("lingo-status", json!({
                "phase": "stopped", "detail": "翻譯已停止，音訊裝置已釋放"
            }));
        })
        .map_err(|error| format!("無法啟動翻譯執行緒：{error}"))?;
    *guard = Some(ActiveSession { stop: stop_tx, alive });
    let _ = app.emit("lingo-status", json!({
        "phase": "connecting", "detail": "正在初始化音訊裝置…"
    }));
    Ok(())
}

#[tauri::command]
fn stop_translation(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    if let Some(session) = state.session.lock().map_err(|_| "狀態鎖定失敗")?.take() {
        let _ = session.stop.send(true);
    }
    let _ = app.emit("lingo-status", json!({
        "phase": "stopping", "detail": "正在結束翻譯並釋放音訊裝置…"
    }));
    Ok(())
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
        ])
        .run(tauri::generate_context!())
        .expect("failed to start Lingo Bridge");
}
