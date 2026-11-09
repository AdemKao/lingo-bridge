//! OpenAI dedicated realtime translation WebSocket client.
//! Source audio remains in RAM, is not saved, and is only sent after explicit start.

use crate::audio::{self, PlaybackQueue};
use crate::resample::{from_pcm16, pcm16_bytes, Resampler, MODEL_FRAME, MODEL_RATE};
use anyhow::{anyhow, bail, Context, Result};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::sync::{mpsc, watch};
use tokio_tungstenite::tungstenite::{http::HeaderValue, Message};
use tokio_tungstenite::{connect_async, tungstenite::client::IntoClientRequest};

#[derive(Clone, Copy)]
pub enum Path {
    Outbound,
    Inbound,
}

impl Path {
    fn as_str(self) -> &'static str {
        match self {
            Self::Outbound => "outbound",
            Self::Inbound => "inbound",
        }
    }
    fn output_language(self) -> &'static str {
        match self {
            Self::Outbound => "ja",
            Self::Inbound => "zh",
        }
    }
}

/// A separate session is required for each direction of the call.
#[allow(clippy::too_many_arguments)]
pub async fn translate(
    key: String,
    model: String,
    path: Path,
    source_rate: u32,
    output_rate: u32,
    mut capture: mpsc::Receiver<Vec<f32>>,
    playback: PlaybackQueue,
    app: AppHandle,
    ready_count: Arc<AtomicUsize>,
    mut stop: watch::Receiver<bool>,
) -> Result<()> {
    let url = format!("wss://api.openai.com/v1/realtime/translations?model={model}");
    let mut request = url
        .into_client_request()
        .context("無法建立翻譯 WebSocket")?;
    request.headers_mut().insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {key}")).context("無效 API Key 格式")?,
    );
    let (mut socket, _) = connect_async(request)
        .await
        .context("無法連線至 OpenAI Realtime Translation（請確認 Key、權限及網路）")?;

    let transcription_model = std::env::var("OPENAI_TRANSCRIPTION_MODEL")
        .unwrap_or_else(|_| "gpt-realtime-whisper".to_owned());
    let update = json!({
        "type": "session.update",
        "session": {
            "audio": {
                "input": { "transcription": { "model": transcription_model } },
                "output": { "language": path.output_language() }
            }
        }
    });
    socket
        .send(Message::Text(update.to_string().into()))
        .await?;

    // Do not append audio before server confirms the target language.
    tokio::time::timeout(Duration::from_secs(20), async {
        loop {
            let message = socket
                .next()
                .await
                .ok_or_else(|| anyhow!("翻譯連線中斷"))??;
            if !message.is_text() {
                continue;
            }
            let value: Value = serde_json::from_str(message.to_text()?)?;
            match value["type"].as_str().unwrap_or_default() {
                "session.updated" => return Ok::<(), anyhow::Error>(()),
                "error" => bail!("翻譯服務拒絕設定：{}", server_error(&value)),
                _ => {}
            }
        }
    })
    .await
    .context("等待翻譯設定回應逾時")??;

    let ready = ready_count.fetch_add(1, Ordering::SeqCst) + 1;
    let _ = app.emit(
        "lingo-status",
        json!({
            "phase": if ready == 2 { "active" } else { "connecting" },
            "detail": if ready == 2 { "兩個翻譯方向皆已連線" } else { "其中一個翻譯方向已連線" }
        }),
    );

    let mut source_resampler = Resampler::new(source_rate, MODEL_RATE);
    let mut target_resampler = Resampler::new(MODEL_RATE, output_rate);
    let mut pending = VecDeque::<f32>::with_capacity(MODEL_FRAME * 2);
    let mut tick = tokio::time::interval(Duration::from_millis(200));
    tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    loop {
        tokio::select! {
            changed = stop.changed() => {
                if changed.is_err() || *stop.borrow() {
                    // Ask the server to flush remaining output before closing.
                    let _ = socket.send(Message::Text(json!({"type":"session.close"}).to_string().into())).await;
                    let _ = tokio::time::timeout(Duration::from_secs(5), async {
                        while let Some(result) = socket.next().await {
                            let message = result?;
                            if message.is_text() {
                                let event: Value = serde_json::from_str(message.to_text()?)?;
                                if event["type"] == "session.closed" { break; }
                                process_event(&event, path, output_rate, &mut target_resampler, &playback, &app)?;
                            }
                        }
                        Ok::<(), anyhow::Error>(())
                    }).await;
                    break;
                }
            }
            data = capture.recv() => {
                match data {
                    Some(samples) => {
                        pending.extend(source_resampler.convert(&samples));
                        // Never build an unbounded backlog after an audio-device stall.
                        while pending.len() > MODEL_FRAME * 8 { pending.pop_front(); }
                    }
                    None => bail!("音訊擷取已中斷，請重新選擇裝置"),
                }
            }
            _ = tick.tick() => {
                let mut packet = Vec::with_capacity(MODEL_FRAME);
                for _ in 0..MODEL_FRAME {
                    packet.push(pending.pop_front().unwrap_or(0.0));
                }
                let encoded = STANDARD.encode(pcm16_bytes(&packet));
                let event = json!({
                    "type": "session.input_audio_buffer.append",
                    "audio": encoded
                });
                socket.send(Message::Text(event.to_string().into())).await?;
            }
            event = socket.next() => {
                match event {
                    Some(Ok(message)) if message.is_text() => {
                        let value: Value = serde_json::from_str(message.to_text()?)?;
                        match value["type"].as_str().unwrap_or_default() {
                            "error" => bail!("翻譯服務錯誤：{}", server_error(&value)),
                            "session.closed" => bail!("翻譯服務已結束連線"),
                            _ => process_event(&value, path, output_rate, &mut target_resampler, &playback, &app)?,
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => bail!("翻譯 WebSocket 連線已關閉"),
                    Some(Err(error)) => return Err(error.into()),
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn server_error(event: &Value) -> String {
    let code = event["error"]["code"].as_str().unwrap_or("unknown");
    let message = event["error"]["message"]
        .as_str()
        .unwrap_or("Please verify API access");
    let shortened: String = message.chars().take(220).collect();
    format!("{code}: {shortened}")
}

fn process_event(
    event: &Value,
    path: Path,
    output_rate: u32,
    resampler: &mut Resampler,
    queue: &PlaybackQueue,
    app: &AppHandle,
) -> Result<()> {
    match event["type"].as_str().unwrap_or_default() {
        "session.input_transcript.delta" | "session.output_transcript.delta" => {
            let source = event["type"] == "session.input_transcript.delta";
            if let Some(delta) = event["delta"].as_str() {
                let _ = app.emit(
                    "lingo-caption",
                    json!({
                        "direction": path.as_str(),
                        "kind": if source { "original" } else { "translated" },
                        "delta": delta,
                    }),
                );
            }
        }
        "session.output_audio.delta" => {
            let encoded = event["delta"].as_str().context("音訊事件遺失資料")?;
            let bytes = STANDARD.decode(encoded).context("音訊資料編碼無效")?;
            let channels = event["channels"].as_u64().unwrap_or(1);
            let sample_rate = event["sample_rate"]
                .as_u64()
                .unwrap_or(u64::from(MODEL_RATE));
            if channels != 1 || sample_rate != u64::from(MODEL_RATE) {
                bail!("音訊服務回傳不支援格式：{channels} channel(s), {sample_rate} Hz");
            }
            if bytes.len() % 2 != 0 {
                bail!("不完整的 PCM16 音訊資料");
            }
            let samples = resampler.convert(&from_pcm16(&bytes));
            audio::enqueue(queue, &samples, output_rate);
        }
        _ => {}
    }
    Ok(())
}
