use crate::dsp::{DspPipeline, DspSettings};
use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, SampleFormat, StreamConfig};
use crossbeam_channel::{bounded, Receiver, Sender};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

#[derive(Debug, Clone)]
pub struct AudioSource {
    pub id: String,
    pub name: String,
    pub device_type: DeviceType,
    pub sample_rate: u32,
    pub channels: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeviceType {
    Microphone,
    SystemAudio,
    AudioFile,
    NetworkStream,
}

pub struct AudioEngine {
    host: Host,
    active_streams: Arc<tokio::sync::Mutex<HashMap<String, StreamHandle>>>,
    audio_sender: Sender<(String, Vec<f32>)>,
    #[allow(dead_code)]
    audio_receiver: Arc<tokio::sync::Mutex<Receiver<(String, Vec<f32>)>>>,
    dsp_configs: Arc<RwLock<HashMap<String, DspSettings>>>,
}

struct StreamHandle {
    #[allow(dead_code)]
    thread_handle: thread::JoinHandle<()>,
}

impl AudioEngine {
    pub async fn new() -> Self {
        let host = cpal::default_host();
        let (audio_sender, audio_receiver) = bounded(1024);

        AudioEngine {
            host,
            active_streams: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
            audio_sender,
            audio_receiver: Arc::new(tokio::sync::Mutex::new(audio_receiver)),
            dsp_configs: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get_input_devices(&self) -> Result<Vec<AudioSource>> {
        let mut devices = Vec::new();

        if let Ok(input_devices) = self.host.input_devices() {
            for (index, device) in input_devices.enumerate() {
                if let Ok(name) = device.name() {
                    if let Ok(default_config) = device.default_input_config() {
                        devices.push(AudioSource {
                            id: format!("mic_{}", index),
                            name,
                            device_type: DeviceType::Microphone,
                            sample_rate: default_config.sample_rate().0,
                            channels: default_config.channels(),
                        });
                    }
                }
            }
        }

        Ok(devices)
    }

    pub fn get_output_devices(&self) -> Result<Vec<AudioSource>> {
        let mut devices = Vec::new();

        if let Ok(output_devices) = self.host.output_devices() {
            for (index, device) in output_devices.enumerate() {
                if let Ok(name) = device.name() {
                    if let Ok(default_config) = device.default_output_config() {
                        devices.push(AudioSource {
                            id: format!("out_{}", index),
                            name,
                            device_type: DeviceType::SystemAudio,
                            sample_rate: default_config.sample_rate().0,
                            channels: default_config.channels(),
                        });
                    }
                }
            }
        }

        Ok(devices)
    }

    pub async fn start_capture(&self, device_id: String) -> Result<()> {
        let device = self
            .host
            .input_devices()?
            .find(|d| d.name().unwrap_or_default().contains(&device_id))
            .ok_or_else(|| anyhow::anyhow!("Device not found"))?;

        let config = device.default_input_config()?;
        let sender = self.audio_sender.clone();
        let device_id_clone = device_id.clone();

        // Load DSP settings for this device
        let dsp_settings = self
            .dsp_configs
            .read()
            .get(&device_id)
            .cloned()
            .unwrap_or_default();
        let pipeline = DspPipeline::from_settings(&dsp_settings);

        let thread_handle = thread::spawn(move || {
            let device = device;
            let config = config.clone();

            let result = match config.sample_format() {
                SampleFormat::F32 => Self::build_and_run_stream::<f32>(
                    &device,
                    &config.into(),
                    sender,
                    device_id_clone,
                    pipeline.clone(),
                ),
                SampleFormat::I16 => Self::build_and_run_stream::<i16>(
                    &device,
                    &config.into(),
                    sender,
                    device_id_clone,
                    pipeline.clone(),
                ),
                SampleFormat::U16 => Self::build_and_run_stream::<u16>(
                    &device,
                    &config.into(),
                    sender,
                    device_id_clone,
                    pipeline.clone(),
                ),
                _ => Err(anyhow::anyhow!("Unsupported sample format")),
            };

            if let Err(e) = result {
                eprintln!("Audio stream error: {}", e);
            }
        });

        let handle = StreamHandle { thread_handle };
        self.active_streams.lock().await.insert(device_id, handle);

        Ok(())
    }

    pub async fn stop_capture(&self, device_id: String) -> Result<()> {
        self.active_streams.lock().await.remove(&device_id);
        Ok(())
    }

    pub fn set_device_dsp(&self, device_id: &str, settings: DspSettings) -> Result<()> {
        self.dsp_configs
            .write()
            .insert(device_id.to_string(), settings);
        Ok(())
    }

    pub fn get_device_dsp(&self, device_id: &str) -> Result<DspSettings> {
        self.dsp_configs
            .read()
            .get(device_id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("No DSP config for device {}", device_id))
    }

    pub fn get_all_dsp_settings(&self) -> Result<HashMap<String, DspSettings>> {
        Ok(self.dsp_configs.read().clone())
    }

    fn build_and_run_stream<T>(
        device: &Device,
        config: &StreamConfig,
        sender: Sender<(String, Vec<f32>)>,
        device_id: String,
        pipeline: DspPipeline,
    ) -> Result<()>
    where
        T: cpal::Sample + cpal::SizedSample + Send + 'static,
        f32: cpal::FromSample<T>,
    {
        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = device.build_input_stream(
            config,
            move |data: &[T], _: &cpal::InputCallbackInfo| {
                let samples: Vec<f32> = data
                    .iter()
                    .map(|&sample| cpal::Sample::from_sample(sample))
                    .collect();

                // Apply DSP pipeline
                let processed = pipeline.process(&samples);

                if let Err(_) = sender.send((device_id.clone(), processed)) {
                    eprintln!("Failed to send audio data");
                }
            },
            err_fn,
            None,
        )?;

        stream.play()?;

        std::thread::park();

        Ok(())
    }
}
