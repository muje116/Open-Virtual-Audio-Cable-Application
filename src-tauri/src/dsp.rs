
#[derive(Debug, Clone)]
pub struct DspProcessor {
    pub gain: f32,
    pub muted: bool,
    pub noise_gate_threshold: f32,
    pub noise_gate_enabled: bool,
}

impl Default for DspProcessor {
    fn default() -> Self {
        DspProcessor {
            gain: 1.0,
            muted: false,
            noise_gate_threshold: 0.01,
            noise_gate_enabled: false,
        }
    }
}

impl DspProcessor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process(&self, samples: &[f32]) -> Vec<f32> {
        if self.muted {
            return vec![0.0; samples.len()];
        }

        samples
            .iter()
            .map(|&sample| {
                let mut processed = sample * self.gain;

                if self.noise_gate_enabled {
                    processed = self.apply_noise_gate(processed);
                }

                processed.clamp(-1.0, 1.0)
            })
            .collect()
    }

    fn apply_noise_gate(&self, sample: f32) -> f32 {
        if sample.abs() < self.noise_gate_threshold {
            0.0
        } else {
            sample
        }
    }

    pub fn set_gain(&mut self, gain: f32) {
        self.gain = gain.clamp(0.0, 10.0);
    }

    pub fn set_mute(&mut self, muted: bool) {
        self.muted = muted;
    }

    pub fn set_noise_gate(&mut self, enabled: bool, threshold: f32) {
        self.noise_gate_enabled = enabled;
        self.noise_gate_threshold = threshold.clamp(0.0, 1.0);
    }
}

#[derive(Debug, Clone)]
pub struct Equalizer {
    pub bands: [f32; 5], // 5-band EQ
}

impl Default for Equalizer {
    fn default() -> Self {
        Equalizer {
            bands: [0.0; 5], // Flat response
        }
    }
}

impl Equalizer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process(&self, samples: &[f32]) -> Vec<f32> {
        // TODO: Implement actual EQ filtering
        // For now, just return samples unchanged
        samples.to_vec()
    }

    pub fn set_band(&mut self, index: usize, gain: f32) {
        if index < 5 {
            self.bands[index] = gain.clamp(-12.0, 12.0);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Compressor {
    pub threshold: f32,
    pub ratio: f32,
    pub attack: f32,
    pub release: f32,
    pub enabled: bool,
}

impl Default for Compressor {
    fn default() -> Self {
        Compressor {
            threshold: -20.0,
            ratio: 4.0,
            attack: 0.01,
            release: 0.1,
            enabled: false,
        }
    }
}

impl Compressor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process(&self, samples: &[f32]) -> Vec<f32> {
        if !self.enabled {
            return samples.to_vec();
        }

        // TODO: Implement actual compression
        // For now, just return samples unchanged
        samples.to_vec()
    }
}
