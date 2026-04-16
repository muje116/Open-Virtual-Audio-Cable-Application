use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, SampleFormat, StreamConfig};
use crossbeam_channel::{bounded, Receiver, Sender};
use std::sync::Arc;
use std::collections::HashMap;
use std::thread;
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
    active_streams: Arc<TokioMutex<HashMap<String, StreamHandle>>>,
    audio_sender: Sender<(String, Vec<f32>)>,
    audio_receiver: Arc<TokioMutex<Receiver<(String, Vec<f32>)>>>,
}

// Wrapper to handle streams that aren't Send/Sync
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

        let config = device.default_input_config()?;
        let sender = self.audio_sender.clone();
        let device_id_clone = device_id.clone();

        // Spawn a thread to handle the audio stream
        let thread_handle = thread::spawn(move || {
            let device = device;
            let config = config.clone();
            
            let result = match config.sample_format() {
                SampleFormat::F32 => {
                    Self::build_and_run_stream::<f32>(&device, &config.into(), sender, device_id_clone)
                }
                SampleFormat::I16 => {
                    Self::build_and_run_stream::<i16>(&device, &config.into(), sender, device_id_clone)
                }
                SampleFormat::U16 => {
                    Self::build_and_run_stream::<u16>(&device, &config.into(), sender, device_id_clone)
                }
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

    fn build_and_run_stream<T>(
        device: &Device,
        config: &StreamConfig,
        sender: Sender<(String, Vec<f32>)>,
        device_id: String,
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

                if let Err(_) = sender.send((device_id.clone(), samples)) {
                    eprintln!("Failed to send audio data");
                }
            },
            err_fn,
            None,
        )?;

        stream.play()?;
        
        // Keep the stream alive
        std::thread::park();
        
        Ok(())
    }
}
