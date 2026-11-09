//! Explicit, user-selected OS audio devices. Does not access LINE internals.
//! Use two independent virtual cables for inbound and outbound LINE audio.

use anyhow::{anyhow, bail, Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, SampleFormat, Stream, StreamConfig};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::f32::consts::TAU;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;

pub type PlaybackQueue = Arc<Mutex<VecDeque<f32>>>;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub kind: &'static str,
    pub is_default: bool,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceSelection {
    pub microphone_input: String,
    pub line_incoming_input: String,
    pub line_outgoing_output: String,
    pub headphones_output: String,
}

impl DeviceSelection {
    pub fn validate(&self) -> Result<()> {
        if self.microphone_input == self.line_incoming_input {
            bail!("麥克風與 LINE 接收音訊必須使用不同的輸入裝置");
        }
        if self.line_outgoing_output == self.headphones_output {
            bail!("LINE 虛擬麥克風與本機耳機必須使用不同的輸出裝置");
        }
        Ok(())
    }
}

pub fn list_devices() -> Result<Vec<DeviceInfo>> {
    let host = cpal::default_host();
    let default_input = host.default_input_device().and_then(|d| d.name().ok());
    let default_output = host.default_output_device().and_then(|d| d.name().ok());
    let mut found = Vec::new();
    for (index, item) in host
        .input_devices()
        .context("無法列出麥克風裝置")?
        .enumerate()
    {
        let name = item.name().unwrap_or_else(|_| "Unknown input".into());
        found.push(DeviceInfo {
            id: format!("in:{index}"),
            is_default: default_input.as_deref() == Some(name.as_str()),
            name,
            kind: "input",
        });
    }
    for (index, item) in host
        .output_devices()
        .context("無法列出喇叭裝置")?
        .enumerate()
    {
        let name = item.name().unwrap_or_else(|_| "Unknown output".into());
        found.push(DeviceInfo {
            id: format!("out:{index}"),
            is_default: default_output.as_deref() == Some(name.as_str()),
            name,
            kind: "output",
        });
    }
    Ok(found)
}

fn selected(id: &str, kind: &str) -> Result<Device> {
    let (actual_kind, raw_index) = id.split_once(':').ok_or_else(|| anyhow!("無效的裝置 ID"))?;
    if actual_kind != kind {
        bail!("裝置類型不正確，請重新選擇");
    }
    let index: usize = raw_index.parse().context("無效的裝置索引")?;
    let host = cpal::default_host();
    match kind {
        "in" => host
            .input_devices()
            .context("無法取得輸入裝置")?
            .nth(index)
            .ok_or_else(|| anyhow!("找不到指定輸入裝置，請重新整理")),
        "out" => host
            .output_devices()
            .context("無法取得輸出裝置")?
            .nth(index)
            .ok_or_else(|| anyhow!("找不到指定輸出裝置，請重新整理")),
        _ => bail!("不支援的裝置類型"),
    }
}

pub struct Capture {
    pub receiver: mpsc::Receiver<Vec<f32>>,
    pub sample_rate: u32,
    pub _stream: Stream,
}

pub struct Playback {
    pub queue: PlaybackQueue,
    pub sample_rate: u32,
    pub _stream: Stream,
}

pub fn start_capture(id: &str) -> Result<Capture> {
    let device = selected(id, "in")?;
    let supported = device
        .default_input_config()
        .context("音訊輸入格式不支援")?;
    let config: StreamConfig = supported.config();
    let sample_rate = config.sample_rate.0;
    let (sender, receiver) = mpsc::channel::<Vec<f32>>(64);

    let stream = match supported.sample_format() {
        SampleFormat::F32 => make_capture(&device, &config, sender, |s: f32| s)?,
        SampleFormat::I16 => make_capture(&device, &config, sender, |s: i16| s as f32 / 32768.0)?,
        SampleFormat::U16 => make_capture(&device, &config, sender, |s: u16| {
            (s as f32 - 32768.0) / 32768.0
        })?,
        other => bail!("目前不支援裝置輸入格式：{other:?}"),
    };
    stream.play().context("無法啟動音訊輸入")?;
    Ok(Capture {
        receiver,
        sample_rate,
        _stream: stream,
    })
}

fn make_capture<T: cpal::SizedSample + Copy + Send + 'static>(
    device: &Device,
    config: &StreamConfig,
    sender: mpsc::Sender<Vec<f32>>,
    to_f32: fn(T) -> f32,
) -> Result<Stream> {
    let channels = usize::from(config.channels);
    let stream = device.build_input_stream(
        config,
        move |data: &[T], _| {
            if channels == 0 {
                return;
            }
            let mut mono = Vec::with_capacity(data.len() / channels);
            for frame in data.chunks_exact(channels) {
                let sum: f32 = frame.iter().map(|&sample| to_f32(sample)).sum();
                mono.push(sum / channels as f32);
            }
            // Never block the real-time audio thread. An overloaded stream drops frames.
            if !mono.is_empty() {
                let _ = sender.try_send(mono);
            }
        },
        |error| eprintln!("Audio capture device error: {error}"),
        None,
    )?;
    Ok(stream)
}

pub fn start_playback(id: &str) -> Result<Playback> {
    let device = selected(id, "out")?;
    let supported = device
        .default_output_config()
        .context("音訊輸出格式不支援")?;
    let config: StreamConfig = supported.config();
    let sample_rate = config.sample_rate.0;
    let queue: PlaybackQueue = Arc::new(Mutex::new(VecDeque::with_capacity(48_000)));
    let stream = match supported.sample_format() {
        SampleFormat::F32 => make_playback(&device, &config, queue.clone(), |s| s)?,
        SampleFormat::I16 => make_playback(&device, &config, queue.clone(), |s| {
            (s.clamp(-1.0, 1.0) * 32767.0) as i16
        })?,
        SampleFormat::U16 => make_playback(&device, &config, queue.clone(), |s| {
            ((s.clamp(-1.0, 1.0) * 32767.0) + 32768.0) as u16
        })?,
        other => bail!("目前不支援裝置輸出格式：{other:?}"),
    };
    stream.play().context("無法啟動音訊輸出")?;
    Ok(Playback {
        queue,
        sample_rate,
        _stream: stream,
    })
}

fn make_playback<T: cpal::SizedSample + Copy + Send + 'static>(
    device: &Device,
    config: &StreamConfig,
    queue: PlaybackQueue,
    from_f32: fn(f32) -> T,
) -> Result<Stream> {
    let channels = usize::from(config.channels);
    let silence = from_f32(0.0);
    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _| {
            if channels == 0 {
                data.fill(silence);
                return;
            }
            if let Ok(mut queue) = queue.try_lock() {
                for frame in data.chunks_mut(channels) {
                    let mono = from_f32(queue.pop_front().unwrap_or(0.0));
                    frame.fill(mono);
                }
            } else {
                data.fill(silence);
            }
        },
        |error| eprintln!("Audio playback device error: {error}"),
        None,
    )?;
    Ok(stream)
}

/// Bound playback latency if network or output hardware gets behind.
pub fn enqueue(queue: &PlaybackQueue, samples: &[f32], sample_rate: u32) {
    if let Ok(mut queue) = queue.lock() {
        let maximum = (sample_rate as usize).saturating_mul(3);
        for &sample in samples {
            queue.push_back(sample);
        }
        while queue.len() > maximum {
            queue.pop_front();
        }
    }
}

/// Produce a short audible tone to manually verify LINE virtual-microphone routing.
/// Does not capture any user audio or call external APIs.
pub fn test_tone(outgoing_device_id: &str) -> Result<()> {
    let output = start_playback(outgoing_device_id)?;
    let samples: Vec<f32> = (0..(output.sample_rate / 2))
        .map(|i| {
            let time = i as f32 / output.sample_rate as f32;
            let envelope = (time * 15.0).min(1.0) * ((0.5 - time).max(0.0) * 15.0).min(1.0);
            (time * 440.0 * TAU).sin() * envelope * 0.15
        })
        .collect();
    enqueue(&output.queue, &samples, output.sample_rate);
    std::thread::sleep(Duration::from_millis(650));
    Ok(())
}


/// A local-only diagnostic. Explicit user action captures up to one second
/// of mono audio in RAM, computes RMS and peak, then drops the stream.
/// No samples are written to disk or sent to any network service.
#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InputLevel {
    pub rms: f32,
    pub peak: f32,
    pub sample_count: usize,
    pub sample_rate: u32,
    pub duration_ms: u64,
}

#[derive(Default)]
struct LevelAccumulator {
    squared_sum: f64,
    peak: f32,
    count: usize,
}

impl LevelAccumulator {
    fn push(&mut self, frames: &[f32]) {
        for &sample in frames {
            let finite = if sample.is_finite() { sample } else { 0.0 };
            let absolute = finite.abs().min(1.0);
            self.peak = self.peak.max(absolute);
            self.squared_sum += f64::from(absolute) * f64::from(absolute);
        }
        self.count += frames.len();
    }

    fn result(&self, sample_rate: u32, duration_ms: u64) -> InputLevel {
        let rms = if self.count == 0 {
            0.0
        } else {
            (self.squared_sum / self.count as f64).sqrt() as f32
        };
        InputLevel {
            rms,
            peak: self.peak,
            sample_count: self.count,
            sample_rate,
            duration_ms,
        }
    }
}

pub fn check_input_level(input_device_id: &str) -> Result<InputLevel> {
    let capture = start_capture(input_device_id)?;
    let mut receiver = capture.receiver;
    // Keep the stream alive for the entire one-second sample interval.
    let _stream = capture._stream;
    let start = std::time::Instant::now();
    let mut meter = LevelAccumulator::default();
    while start.elapsed() < Duration::from_millis(900) {
        match receiver.try_recv() {
            Ok(samples) => meter.push(&samples),
            Err(mpsc::error::TryRecvError::Empty) => {
                std::thread::sleep(Duration::from_millis(12));
            }
            Err(mpsc::error::TryRecvError::Disconnected) => {
                bail!("音訊裝置已中斷，請重新整理裝置清單");
            }
        }
    }
    // Drain samples already captured before the one-second window closes.
    while let Ok(samples) = receiver.try_recv() {
        meter.push(&samples);
    }
    if meter.count == 0 {
        bail!("在測試時間內沒有收到裝置音訊樣本");
    }
    Ok(meter.result(capture.sample_rate, start.elapsed().as_millis() as u64))
}

#[cfg(test)]
mod input_level_tests {
    use super::*;

    #[test]
    fn measures_peak_and_rms_without_exposing_samples() {
        let mut meter = LevelAccumulator::default();
        meter.push(&[0.5, -0.5, 0.0, 0.0]);
        let report = meter.result(48_000, 100);
        assert!((report.rms - 0.35355338).abs() < 0.0001);
        assert_eq!(report.peak, 0.5);
        assert_eq!(report.sample_count, 4);
    }

    #[test]
    fn nonfinite_audio_is_silenced() {
        let mut meter = LevelAccumulator::default();
        meter.push(&[f32::NAN, f32::INFINITY, -f32::INFINITY]);
        assert_eq!(meter.result(16_000, 100).rms, 0.0);
    }
}
