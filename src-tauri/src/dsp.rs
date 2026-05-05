use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equalizer {
    pub bands: [f32; 5],
}

impl Default for Equalizer {
    fn default() -> Self {
        Equalizer {
            bands: [0.0; 5],
        }
    }
}

impl Equalizer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process(&self, samples: &[f32]) -> Vec<f32> {
        let sample_rate = 48000.0;
        let frequencies = [60.0, 250.0, 1000.0, 4000.0, 16000.0];

        samples.iter().map(|&sample| {
            let mut output = sample;
            for (i, &freq) in frequencies.iter().enumerate() {
                let gain_db = self.bands[i];
                if gain_db.abs() > 0.01 {
                    output = self.apply_biquad_filter(output, freq, gain_db, sample_rate);
                }
            }
            output.clamp(-1.0, 1.0)
        }).collect()
    }

    fn apply_biquad_filter(&self, sample: f32, freq: f32, gain_db: f32, sample_rate: f32) -> f32 {
        let omega = 2.0 * std::f32::consts::PI * freq / sample_rate;
        let gain_linear = 10.0_f32.powf(gain_db / 20.0);
        let alpha = (omega / (1.0 + gain_linear)).tanh();
        sample * (1.0 - alpha) + sample * gain_linear * alpha
    }

    pub fn set_band(&mut self, index: usize, gain: f32) {
        if index < 5 {
            self.bands[index] = gain.clamp(-12.0, 12.0);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

        let threshold_linear = 10.0_f32.powf(self.threshold / 20.0);
        let ratio = self.ratio;
        let attack_factor = self.attack.min(1.0);
        let release_factor = self.release.min(1.0);

        let mut envelope = 0.0;
        samples.iter().map(|&sample| {
            let input_level = sample.abs();

            if input_level > envelope {
                envelope = envelope * (1.0 - attack_factor) + input_level * attack_factor;
            } else {
                envelope = envelope * (1.0 - release_factor) + input_level * release_factor;
            }

            let gain = if envelope > threshold_linear {
                let excess_db = 20.0 * (envelope / threshold_linear).log10();
                let reduction_db = excess_db * (1.0 - 1.0 / ratio);
                10.0_f32.powf(-reduction_db / 20.0)
            } else {
                1.0
            };

            (sample * gain).clamp(-1.0, 1.0)
        }).collect()
    }

    pub fn set_threshold(&mut self, threshold: f32) {
        self.threshold = threshold.clamp(-60.0, 0.0);
    }

    pub fn set_ratio(&mut self, ratio: f32) {
        self.ratio = ratio.clamp(1.0, 20.0);
    }

    pub fn set_attack(&mut self, attack: f32) {
        self.attack = attack.clamp(0.001, 1.0);
    }

    pub fn set_release(&mut self, release: f32) {
        self.release = release.clamp(0.01, 2.0);
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DspSettings {
    pub gain: f32,
    pub noise_gate_enabled: bool,
    pub noise_gate_threshold: f32,
    pub eq_bands: [f32; 5],
    pub compressor_enabled: bool,
    pub compressor_threshold: f32,
    pub compressor_ratio: f32,
    pub compressor_attack: f32,
    pub compressor_release: f32,
}

impl Default for DspSettings {
    fn default() -> Self {
        DspSettings {
            gain: 1.0,
            noise_gate_enabled: false,
            noise_gate_threshold: -60.0,
            eq_bands: [0.0; 5],
            compressor_enabled: false,
            compressor_threshold: -20.0,
            compressor_ratio: 4.0,
            compressor_attack: 0.01,
            compressor_release: 0.1,
        }
    }
}

impl DspSettings {
    pub fn to_processor(&self) -> DspProcessor {
        DspProcessor {
            gain: self.gain,
            muted: false,
            noise_gate_threshold: 10.0_f32.powf(self.noise_gate_threshold / 20.0),
            noise_gate_enabled: self.noise_gate_enabled,
        }
    }

    pub fn to_equalizer(&self) -> Equalizer {
        Equalizer {
            bands: self.eq_bands,
        }
    }

    pub fn to_compressor(&self) -> Compressor {
        Compressor {
            threshold: self.compressor_threshold,
            ratio: self.compressor_ratio,
            attack: self.compressor_attack,
            release: self.compressor_release,
            enabled: self.compressor_enabled,
        }
    }
}

#[derive(Clone)]
pub struct DspPipeline {
    pub processor: DspProcessor,
    pub equalizer: Equalizer,
    pub compressor: Compressor,
}

impl DspPipeline {
    pub fn from_settings(settings: &DspSettings) -> Self {
        DspPipeline {
            processor: settings.to_processor(),
            equalizer: settings.to_equalizer(),
            compressor: settings.to_compressor(),
        }
    }

    pub fn process(&self, samples: &[f32]) -> Vec<f32> {
        let samples = self.processor.process(samples);
        let samples = self.equalizer.process(&samples);
        self.compressor.process(&samples)
    }
}
