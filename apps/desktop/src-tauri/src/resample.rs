//! Streaming linear interpolation for the prototype audio path.
//! Production audio quality should use a bandlimited resampler (rubato / soxr).

pub const MODEL_RATE: u32 = 24_000;
pub const MODEL_FRAME: usize = 4_800;

pub struct Resampler {
    input_rate: u32,
    output_rate: u32,
    previous: Option<f32>,
    last_index: u64,
    next_at: f64,
}

impl Resampler {
    pub fn new(input_rate: u32, output_rate: u32) -> Self {
        Self { input_rate, output_rate, previous: None, last_index: 0, next_at: 0.0 }
    }

    pub fn convert(&mut self, input: &[f32]) -> Vec<f32> {
        if self.input_rate == 0 || self.output_rate == 0 {
            return Vec::new();
        }
        let spacing = self.input_rate as f64 / self.output_rate as f64;
        let mut result = Vec::with_capacity(
            (input.len() as f64 * self.output_rate as f64 / self.input_rate as f64) as usize + 2,
        );
        for &sample in input {
            match self.previous {
                None => {
                    self.previous = Some(sample);
                    result.push(sample);
                    self.next_at = spacing;
                }
                Some(previous) => {
                    self.last_index += 1;
                    let end = self.last_index as f64;
                    while self.next_at <= end {
                        let mix = (self.next_at - (end - 1.0)) as f32;
                        result.push(previous + (sample - previous) * mix.clamp(0.0, 1.0));
                        self.next_at += spacing;
                    }
                    self.previous = Some(sample);
                }
            }
        }
        result
    }
}

pub fn pcm16_bytes(samples: &[f32]) -> Vec<u8> {
    let mut result = Vec::with_capacity(samples.len() * 2);
    for &sample in samples {
        let int = (sample.clamp(-1.0, 1.0) * 32767.0) as i16;
        result.extend_from_slice(&int.to_le_bytes());
    }
    result
}

pub fn from_pcm16(bytes: &[u8]) -> Vec<f32> {
    bytes.chunks_exact(2).map(|b| {
        i16::from_le_bytes([b[0], b[1]]) as f32 / 32768.0
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn downsample_48k_to_24k() {
        let mut resampler = Resampler::new(48_000, 24_000);
        let a = resampler.convert(&[0.0, 1.0, 2.0, 3.0]);
        let b = resampler.convert(&[4.0, 5.0, 6.0]);
        assert_eq!(a, vec![0.0, 2.0]);
        assert_eq!(b, vec![4.0, 6.0]);
    }

    #[test]
    fn upsample_24k_to_48k() {
        let mut r = Resampler::new(24_000, 48_000);
        assert_eq!(r.convert(&[0.0, 1.0, 0.0]), vec![0.0, 0.5, 1.0, 0.5, 0.0]);
    }

    #[test]
    fn round_trip_pcm16() {
        let values = vec![-0.5, 0.0, 0.5];
        let decoded = from_pcm16(&pcm16_bytes(&values));
        assert_eq!(decoded.len(), values.len());
        assert!(decoded.iter().zip(values).all(|(a, b)| (a - b).abs() < 0.001));
    }
}
