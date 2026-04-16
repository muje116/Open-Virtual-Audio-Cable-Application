use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Host};
use crossbeam_channel::{bounded, Receiver, Sender};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex as TokioMutex;

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
    active_streams: Arc<TokioMutex<HashMap<String, bool>>>,
    audio_sender: Sender<(String, Vec<f32>)>,
    audio_receiver: Arc<TokioMutex<Receiver<(String, Vec<f32>)>>>,
}

impl AudioEngine {
    pub async fn new() -> Self {
        let host = cpal::default_host();
        let (audio_sender, audio_receiver) = bounded(1024);

        AudioEngine {
            host,
            active_streams: Arc::new(TokioMutex::new(HashMap::new())),
            audio_sender,
            audio_receiver: Arc::new(TokioMutex::new(audio_receiver)),
        }
    }

    pub fn get_input_devices(&self) -> Result<Vec<AudioSource>> {
        let mut devices = Vec::new();

        // Get input devices
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

        // Get output devices
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

        let _config = device.default_input_config()?;
        let _sender = self.audio_sender.clone();
        let _device_id_clone = device_id.clone();

        // For now, just mark as active without actually starting the stream
        // In a real implementatio.await, you'd need to handle the stream differently
        self.active_streams.lock().await.insert(device_id, true);

        Ok(())
    }

    pub async fn stop_capture(&self, device_id: String) -> Result<()> {
        self.active_streams.lock().await.remove(&device_id);
        Ok(())
    }
}
