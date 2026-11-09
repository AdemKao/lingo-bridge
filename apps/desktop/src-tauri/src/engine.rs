//! Both call directions use the same OS audio router. Original-audio bypass never
//! connects to OpenAI (or any other model service).

use crate::audio::{self, Capture, DeviceSelection, Playback, PlaybackQueue};
use crate::realtime::{self, Path};
use crate::resample::Resampler;
use anyhow::{Context, Result};
use lingo_core::{AudioDirection, CallLanguageSettings};
use serde_json::json;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::{mpsc, watch};

pub fn run(
    app: AppHandle,
    key: Option<String>,
    model: String,
    selection: DeviceSelection,
    languages: CallLanguageSettings,
    stop: watch::Receiver<bool>,
) -> Result<()> {
    selection.validate()?;
    let microphone =
        audio::start_capture(&selection.microphone_input).context("無法啟動實體麥克風")?;
    let remote_speech = audio::start_capture(&selection.line_incoming_input)
        .context("無法啟動 LINE 輸入音訊裝置")?;
    let japanese_to_line = audio::start_playback(&selection.line_outgoing_output)
        .context("無法啟動 LINE 虛擬麥克風輸出")?;
    let chinese_headphones =
        audio::start_playback(&selection.headphones_output).context("無法啟動本機耳機輸出")?;
    let _ = app.emit(
        "lingo-status",
        json!({"phase":"connecting","detail":"音訊設備已啟動，正在啟動各方向的音訊處理…"}),
    );
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("建立音訊翻譯執行環境失敗")?;
    let ready = Arc::new(AtomicUsize::new(0));
    runtime.block_on(async move {
        tokio::try_join!(
            route(
                key.clone(),
                model.clone(),
                Path::Outbound,
                languages.outbound,
                microphone,
                japanese_to_line,
                app.clone(),
                ready.clone(),
                stop.clone()
            ),
            route(
                key,
                model,
                Path::Inbound,
                languages.inbound,
                remote_speech,
                chinese_headphones,
                app,
                ready,
                stop
            )
        )?;
        Ok(())
    })
}

#[allow(clippy::too_many_arguments)]
async fn route(
    key: Option<String>,
    model: String,
    path: Path,
    language: AudioDirection,
    capture: Capture,
    playback: Playback,
    app: AppHandle,
    ready: Arc<AtomicUsize>,
    stop: watch::Receiver<bool>,
) -> Result<()> {
    let Capture {
        receiver,
        sample_rate: source_rate,
        _stream: _capture_stream,
    } = capture;
    let Playback {
        queue,
        sample_rate: output_rate,
        _stream: _output_stream,
    } = playback;
    match language.output_language() {
        Some(target) => {
            realtime::translate(
                key.context("翻譯方向需要 API Key")?,
                model,
                path,
                source_rate,
                output_rate,
                receiver,
                queue,
                app,
                ready,
                stop,
                target,
            )
            .await
        }
        None => passthrough(receiver, source_rate, queue, output_rate, app, ready, stop).await,
    }
}

async fn passthrough(
    mut receiver: mpsc::Receiver<Vec<f32>>,
    source_rate: u32,
    playback: PlaybackQueue,
    output_rate: u32,
    app: AppHandle,
    ready: Arc<AtomicUsize>,
    mut stop: watch::Receiver<bool>,
) -> Result<()> {
    let count = ready.fetch_add(1, Ordering::SeqCst) + 1;
    if count == 2 {
        let _ = app.emit(
            "lingo-status",
            json!({"phase":"active","detail":"音訊管線已就緒；不翻譯的方向只傳送原音"}),
        );
    }
    let mut resampler = Resampler::new(source_rate, output_rate);
    if *stop.borrow() {
        return Ok(());
    }
    loop {
        tokio::select! {
            changed = stop.changed() => {
                if changed.is_err() || *stop.borrow() {
                    return Ok(());
                }
            }
            chunk = receiver.recv() => {
                match chunk {
                    Some(samples) => {
                        audio::enqueue(&playback, &resampler.convert(&samples), output_rate);
                    }
                    None => anyhow::bail!("原音轉送裝置已中斷"),
                }
            }
        }
    }
}
