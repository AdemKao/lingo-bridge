//! Offline speech building blocks for Lingo Bridge.
//!
//! The energy gate is deliberately lightweight, intended for initial WAV-based
//! testing. It is NOT a production VAD and does not infer language or transcribe.
//! Optional Sherpa ONNX adapters live behind the opt-in `sherpa` feature.

use std::collections::VecDeque;

#[cfg(feature = "sherpa")]
pub mod sherpa;

#[cfg(feature = "offline-pipeline")]
pub mod offline;

#[derive(Clone, Copy, Debug)]
pub struct SegmentConfig {
    pub sample_rate: u32,
    pub speech_rms_threshold: f32,
    pub min_speech_ms: u32,
    pub end_silence_ms: u32,
    pub max_segment_ms: u32,
    pub pre_roll_ms: u32,
}

impl SegmentConfig {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            speech_rms_threshold: 0.015,
            min_speech_ms: 220,
            end_silence_ms: 550,
            max_segment_ms: 12_000,
            pre_roll_ms: 160,
        }
    }

    fn samples(self, millis: u32) -> usize {
        (u64::from(self.sample_rate) * u64::from(millis) / 1000) as usize
    }

    pub fn validate(self) -> Result<Self, &'static str> {
        if !(8_000..=192_000).contains(&self.sample_rate) {
            return Err("unsupported sample rate");
        }
        if !self.speech_rms_threshold.is_finite()
            || !(0.0..=1.0).contains(&self.speech_rms_threshold)
        {
            return Err("invalid RMS threshold");
        }
        if self.min_speech_ms == 0
            || self.end_silence_ms == 0
            || self.max_segment_ms <= self.min_speech_ms + self.end_silence_ms
            || self.pre_roll_ms > self.max_segment_ms
        {
            return Err("invalid speech timing");
        }
        Ok(self)
    }
}

#[derive(Clone, Debug)]
pub struct Utterance {
    pub sample_rate: u32,
    pub samples: Vec<f32>,
}

impl Utterance {
    pub fn duration_seconds(&self) -> f32 {
        self.samples.len() as f32 / self.sample_rate as f32
    }
}

/// Streaming energy gate. Feed small (10–200 ms) mono chunks.
///
/// It keeps at most `max_segment_ms` worth of speech plus one input chunk.
/// It is not a substitute for Silero VAD or a speaker-aware recognizer.
pub struct EnergySegmenter {
    config: SegmentConfig,
    pre_roll: VecDeque<f32>,
    active: Vec<f32>,
    speaking: bool,
    trailing_silence_samples: usize,
    voiced_samples: usize,
}

impl EnergySegmenter {
    pub fn new(config: SegmentConfig) -> Result<Self, &'static str> {
        let config = config.validate()?;
        Ok(Self {
            config,
            pre_roll: VecDeque::with_capacity(config.samples(config.pre_roll_ms)),
            active: Vec::new(),
            speaking: false,
            trailing_silence_samples: 0,
            voiced_samples: 0,
        })
    }

    pub fn push(&mut self, mono: &[f32]) -> Option<Utterance> {
        if mono.is_empty() {
            return None;
        }

        let rms = (mono
            .iter()
            .map(|sample| {
                let clamped = if sample.is_finite() { *sample } else { 0.0 };
                clamped * clamped
            })
            .sum::<f32>()
            / mono.len() as f32)
            .sqrt();
        let voiced = rms >= self.config.speech_rms_threshold;

        if !self.speaking {
            if !voiced {
                self.remember_pre_roll(mono);
                return None;
            }
            self.speaking = true;
            self.active.extend(self.pre_roll.drain(..));
            self.active.extend_from_slice(mono);
            self.voiced_samples = mono.len();
            self.trailing_silence_samples = 0;
        } else {
            self.active.extend_from_slice(mono);
            if voiced {
                self.voiced_samples = self.voiced_samples.saturating_add(mono.len());
                self.trailing_silence_samples = 0;
            } else {
                self.trailing_silence_samples =
                    self.trailing_silence_samples.saturating_add(mono.len());
            }
        }

        if self.trailing_silence_samples >= self.config.samples(self.config.end_silence_ms)
            || self.active.len() >= self.config.samples(self.config.max_segment_ms)
        {
            return self.finish();
        }
        None
    }

    /// Flush a remaining speech segment when the input device or call stops.
    pub fn flush(&mut self) -> Option<Utterance> {
        self.finish()
    }

    fn finish(&mut self) -> Option<Utterance> {
        let enough = self.voiced_samples >= self.config.samples(self.config.min_speech_ms);
        let samples = std::mem::take(&mut self.active);
        self.pre_roll.clear();
        self.speaking = false;
        self.trailing_silence_samples = 0;
        self.voiced_samples = 0;
        enough.then_some(Utterance {
            sample_rate: self.config.sample_rate,
            samples,
        })
    }

    fn remember_pre_roll(&mut self, mono: &[f32]) {
        let max = self.config.samples(self.config.pre_roll_ms);
        for &sample in mono {
            if max == 0 {
                return;
            }
            self.pre_roll.push_back(sample);
            if self.pre_roll.len() > max {
                self.pre_roll.pop_front();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_config_and_rejects_bad_values() {
        assert!(SegmentConfig::new(16_000).validate().is_ok());
        assert!(SegmentConfig::new(0).validate().is_err());
        let mut config = SegmentConfig::new(16_000);
        config.speech_rms_threshold = f32::NAN;
        assert!(config.validate().is_err());
    }

    #[test]
    fn does_not_emit_silence_or_tiny_noise() {
        let mut segmenter = EnergySegmenter::new(SegmentConfig::new(16_000)).unwrap();
        for _ in 0..100 {
            assert!(segmenter.push(&vec![0.001; 320]).is_none());
        }
        assert!(segmenter.flush().is_none());
    }

    #[test]
    fn speech_then_silence_yields_one_segment() {
        let mut segmenter = EnergySegmenter::new(SegmentConfig::new(16_000)).unwrap();
        segmenter.push(&vec![0.0; 1_600]);
        for _ in 0..25 {
            assert!(segmenter.push(&vec![0.1; 320]).is_none());
        }
        let mut result = None;
        for _ in 0..30 {
            if let Some(utterance) = segmenter.push(&vec![0.0; 320]) {
                result = Some(utterance);
            }
        }
        let utterance = result.expect("one utterance");
        assert!(utterance.duration_seconds() >= 1.0);
        assert!(segmenter.flush().is_none());
    }

    #[test]
    fn very_short_speech_does_not_emit() {
        let mut segmenter = EnergySegmenter::new(SegmentConfig::new(16_000)).unwrap();
        segmenter.push(&vec![0.1; 1_600]);
        assert!(segmenter.flush().is_none());
    }

    #[test]
    fn max_segment_cap_causes_emission() {
        let mut config = SegmentConfig::new(16_000);
        config.max_segment_ms = 2_000;
        let mut segmenter = EnergySegmenter::new(config).unwrap();
        let mut output = None;
        for _ in 0..20 {
            if let Some(segment) = segmenter.push(&vec![0.1; 1_600]) {
                output = Some(segment);
            }
        }
        assert!(output.is_some());
    }
}
