//! Orchestrates 4 explicitly selected devices and 2 independent translation sockets.

use crate::audio::{self, DeviceSelection};
use crate::realtime::{self, Path};
use anyhow::{Context, Result};
use serde_json::json;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::watch;

pub fn run(
    app: AppHandle,
    key: String,
    model: String,
    selection: DeviceSelection,
    stop: watch::Receiver<bool>,
) -> Result<()> {
    selection.validate()?;
    let microphone = audio::start_capture(&selection.microphone_input)
        .context("無法啟動實體麥克風")?;
    let remote_speech = audio::start_capture(&selection.line_incoming_input)
        .context("無法啟動 LINE 輸入音訊裝置")?;
    let japanese_to_line = audio::start_playback(&selection.line_outgoing_output)
        .context("無法啟動 LINE 虛擬麥克風輸出")?;
    let chinese_headphones = audio::start_playback(&selection.headphones_output)
        .context("無法啟動本機耳機輸出")?;
    let _ = app.emit("lingo-status", json!({
        "phase": "connecting",
        "detail": "音訊設備已啟動，正在建立兩個 AI 翻譯連線…"
    }));

    // The CPAL streams remain owned by this worker thread while the runtime runs.
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("建立音訊翻譯執行環境失敗")?;
    let ready = Arc::new(AtomicUsize::new(0));
    runtime.block_on(async move {
        tokio::try_join!(
            realtime::translate(
                key.clone(), model.clone(), Path::Outbound,
                microphone.sample_rate, japanese_to_line.sample_rate,
                microphone.receiver, japanese_to_line.queue.clone(),
                app.clone(), ready.clone(), stop.clone(),
            ),
            realtime::translate(
                key, model, Path::Inbound,
                remote_speech.sample_rate, chinese_headphones.sample_rate,
                remote_speech.receiver, chinese_headphones.queue.clone(),
                app, ready, stop,
            )
        )?;
        Ok(())
    })
}
