//! PCM helpers. Real capture/playback must be implemented in platform adapters.

use std::time::Duration;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AudioError {
    ZeroSampleRate,
    ZeroChannels,
    PartialFrame,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PcmFormat {
    pub sample_rate_hz: u32,
    pub channels: u16,
}

impl PcmFormat {
    pub fn new(sample_rate_hz: u32, channels: u16) -> Result<Self, AudioError> {
        if sample_rate_hz == 0 {
            return Err(AudioError::ZeroSampleRate);
        }
        if channels == 0 {
            return Err(AudioError::ZeroChannels);
        }
        Ok(Self { sample_rate_hz, channels })
    }

    pub fn duration(self, interleaved_samples: usize) -> Result<Duration, AudioError> {
        if interleaved_samples % usize::from(self.channels) != 0 {
            return Err(AudioError::PartialFrame);
        }
        let frames = interleaved_samples / usize::from(self.channels);
        Ok(Duration::from_secs_f64(frames as f64 / f64::from(self.sample_rate_hz)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_ms_stereo_at_48khz() {
        let pcm = PcmFormat::new(48_000, 2).unwrap();
        assert_eq!(pcm.duration(960).unwrap(), Duration::from_millis(10));
    }

    #[test]
    fn invalid_formats_rejected() {
        assert_eq!(PcmFormat::new(0, 1), Err(AudioError::ZeroSampleRate));
        assert_eq!(PcmFormat::new(48_000, 0), Err(AudioError::ZeroChannels));
        assert_eq!(PcmFormat::new(48_000, 2).unwrap().duration(3), Err(AudioError::PartialFrame));
    }
}
