use crate::dsp::{DspPipeline, DspSettings};
use crate::routing::Route as RoutingRoute;
use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, Sample, SampleFormat, SizedSample, StreamConfig};
use crossbeam_channel::{bounded, Receiver, Sender, TryRecvError};
use parking_lot::{Mutex, RwLock};
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[cfg(target_os = "windows")]
use windows::Win32::Media::Audio::{
    eConsole, eRender, IAudioCaptureClient, IAudioClient, IMMDeviceEnumerator,
    AUDCLNT_BUFFERFLAGS_SILENT, AUDCLNT_SHAREMODE_SHARED, AUDCLNT_STREAMFLAGS_LOOPBACK,
    MMDeviceEnumerator,
};
#[cfg(target_os = "windows")]
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoTaskMemFree, CLSCTX_ALL, COINIT_MULTITHREADED,
};

const LOOPBACK_DEFAULT_ID: &str = "loopback_default";

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

struct CaptureHandle {
    #[allow(dead_code)]
    thread_handle: thread::JoinHandle<()>,
}

struct OutputHandle {
    sender: Sender<Vec<f32>>,
}

#[derive(Clone)]
struct RouteRuntime {
    input_id: String,
    output_id: String,
    volume: f32,
    muted: bool,
}

pub struct AudioEngine {
    host: Host,
    active_captures: Arc<Mutex<HashMap<String, CaptureHandle>>>,
    active_outputs: Arc<Mutex<HashMap<String, OutputHandle>>>,
    routes: Arc<RwLock<Vec<RouteRuntime>>>,
    audio_sender: Sender<(String, Vec<f32>)>,
    dsp_configs: Arc<RwLock<HashMap<String, DspSettings>>>,
}

impl AudioEngine {
    pub async fn new() -> Self {
        let host = cpal::default_host();
        let (audio_sender, audio_receiver) = bounded::<(String, Vec<f32>)>(1024);

        let engine = AudioEngine {
            host,
            active_captures: Arc::new(Mutex::new(HashMap::new())),
            active_outputs: Arc::new(Mutex::new(HashMap::new())),
            routes: Arc::new(RwLock::new(Vec::new())),
            audio_sender,
            dsp_configs: Arc::new(RwLock::new(HashMap::new())),
        };

        engine.start_router_worker(audio_receiver);
        engine
    }

    pub fn get_input_devices(&self) -> Result<Vec<AudioSource>> {
        let mut devices = Vec::new();

        if let Ok(input_devices) = self.host.input_devices() {
            for (index, device) in input_devices.enumerate() {
                if let (Ok(name), Ok(default_config)) =
                    (device.name(), device.default_input_config())
                {
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

        #[cfg(target_os = "windows")]
        {
            devices.push(AudioSource {
                id: LOOPBACK_DEFAULT_ID.to_string(),
                name: "System Speaker Loopback (Default Output)".to_string(),
                device_type: DeviceType::SystemAudio,
                sample_rate: 48000,
                channels: 2,
            });
        }

        Ok(devices)
    }

    pub fn get_output_devices(&self) -> Result<Vec<AudioSource>> {
        let mut devices = Vec::new();

        if let Ok(output_devices) = self.host.output_devices() {
            for (index, device) in output_devices.enumerate() {
                if let (Ok(name), Ok(default_config)) =
                    (device.name(), device.default_output_config())
                {
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

        Ok(devices)
    }

    pub fn sync_routes(&self, routes: Vec<RoutingRoute>) -> Result<()> {
        let mapped = routes
            .into_iter()
            .map(|r| RouteRuntime {
                input_id: r.input_id,
                output_id: r.output_id,
                volume: r.volume.clamp(0.0, 1.0),
                muted: r.muted,
            })
            .collect::<Vec<_>>();
        *self.routes.write() = mapped;
        let output_ids = self
            .routes
            .read()
            .iter()
            .map(|r| r.output_id.clone())
            .collect::<Vec<_>>();
        for output_id in output_ids {
            self.ensure_output_stream_for_id(&output_id);
        }
        Ok(())
    }

    pub async fn start_capture(&self, device_id: String) -> Result<()> {
        if self.active_captures.lock().contains_key(&device_id) {
            return Ok(());
        }

        let dsp = self
            .dsp_configs
            .read()
            .get(&device_id)
            .cloned()
            .unwrap_or_default();
        let pipeline = DspPipeline::from_settings(&dsp);

        #[cfg(target_os = "windows")]
        if device_id == LOOPBACK_DEFAULT_ID {
            let sender = self.audio_sender.clone();
            let dev_id = device_id.clone();
            let thread_handle = thread::spawn(move || {
                if let Err(e) = Self::run_wasapi_loopback_capture(sender, dev_id, pipeline) {
                    eprintln!("loopback capture failed: {}", e);
                }
            });

            self.active_captures
                .lock()
                .insert(device_id, CaptureHandle { thread_handle });
            return Ok(());
        }

        let device = self.resolve_input_device_by_id(&device_id)?;
        let config = device.default_input_config()?;
        let sender = self.audio_sender.clone();

        let dev_id = device_id.clone();
        let thread_handle = thread::spawn(move || {
            let run_result = match config.sample_format() {
                SampleFormat::F32 => {
                    Self::run_capture_stream::<f32>(&device, &config.into(), sender, dev_id, pipeline)
                }
                SampleFormat::I16 => {
                    Self::run_capture_stream::<i16>(&device, &config.into(), sender, dev_id, pipeline)
                }
                SampleFormat::U16 => {
                    Self::run_capture_stream::<u16>(&device, &config.into(), sender, dev_id, pipeline)
                }
                _ => Err(anyhow::anyhow!("Unsupported input sample format")),
            };

            if let Err(e) = run_result {
                eprintln!("capture stream failed: {}", e);
            }
        });

        self.active_captures
            .lock()
            .insert(device_id, CaptureHandle { thread_handle });
        Ok(())
    }

    pub async fn stop_capture(&self, device_id: String) -> Result<()> {
        self.active_captures.lock().remove(&device_id);
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

    fn resolve_input_device_by_id(&self, device_id: &str) -> Result<Device> {
        if let Some(index_str) = device_id.strip_prefix("mic_") {
            let index = index_str.parse::<usize>()?;
            if let Ok(mut iter) = self.host.input_devices() {
                return iter
                    .nth(index)
                    .ok_or_else(|| anyhow::anyhow!("Input device index {} not found", index));
            }
        }

        self.host
            .input_devices()?
            .find(|d| d.name().map(|n| n.contains(device_id)).unwrap_or(false))
            .ok_or_else(|| anyhow::anyhow!("Input device not found for id {}", device_id))
    }

    fn ensure_output_stream_for_id(&self, output_id: &str) {
        if !output_id.starts_with("out_") {
            return;
        }

        let outputs_arc = self.active_outputs.clone();
        let output_id_s = output_id.to_string();

        thread::spawn(move || {
            let already_exists = outputs_arc.lock().contains_key(&output_id_s);
            if already_exists {
                return;
            }

            let device = {
                let mut found: Option<Device> = None;
                if let Some(index_str) = output_id_s.strip_prefix("out_") {
                    if let Ok(index) = index_str.parse::<usize>() {
                        if let Ok(mut iter) = cpal::default_host().output_devices() {
                            found = iter.nth(index);
                        }
                    }
                }
                match found {
                    Some(d) => d,
                    None => return,
                }
            };

            let supported = match device.default_output_config() {
                Ok(c) => c,
                Err(_) => return,
            };

            let (tx, rx) = bounded::<Vec<f32>>(512);
            let stream_result = match supported.sample_format() {
                SampleFormat::F32 => Self::build_output_stream::<f32>(&device, &supported.into(), rx),
                SampleFormat::I16 => Self::build_output_stream::<i16>(&device, &supported.into(), rx),
                SampleFormat::U16 => Self::build_output_stream::<u16>(&device, &supported.into(), rx),
                _ => Err(anyhow::anyhow!("Unsupported output sample format")),
            };

            let stream = match stream_result {
                Ok(s) => s,
                Err(_) => return,
            };
            if stream.play().is_err() {
                return;
            }
            let mut guard = outputs_arc.lock();
            guard.insert(
                output_id_s,
                OutputHandle {
                    sender: tx,
                },
            );

            let _stream = stream;
            loop {
                thread::sleep(Duration::from_secs(3600));
            }
        });
    }

    fn start_router_worker(&self, receiver: Receiver<(String, Vec<f32>)>) {
        let routes = self.routes.clone();
        let outputs = self.active_outputs.clone();

        thread::spawn(move || {
            while let Ok((input_id, samples)) = receiver.recv() {
                let current_routes = routes.read().clone();
                for route in current_routes.iter().filter(|r| r.input_id == input_id) {
                    if route.muted {
                        continue;
                    }

                    let out_id = route.output_id.clone();
                    let sender_opt = outputs.lock().get(&out_id).map(|h| h.sender.clone());

                    let sender = if let Some(s) = sender_opt { s } else { continue };

                    let mixed = if (route.volume - 1.0).abs() < f32::EPSILON {
                        samples.clone()
                    } else {
                        samples.iter().map(|s| s * route.volume).collect::<Vec<f32>>()
                    };

                    let _ = sender.try_send(mixed);
                }
            }
        });
    }

    fn run_capture_stream<T>(
        device: &Device,
        config: &StreamConfig,
        sender: Sender<(String, Vec<f32>)>,
        device_id: String,
        pipeline: DspPipeline,
    ) -> Result<()>
    where
        T: Sample + SizedSample + Send + 'static,
        f32: cpal::FromSample<T>,
    {
        let err_fn = |err| eprintln!("capture stream error: {}", err);
        let stream = device.build_input_stream(
            config,
            move |data: &[T], _| {
                let samples: Vec<f32> = data.iter().map(|&s| f32::from_sample(s)).collect();
                let processed = pipeline.process(&samples);
                let _ = sender.try_send((device_id.clone(), processed));
            },
            err_fn,
            None,
        )?;
        stream.play()?;
        let _stream = stream;
        loop {
            thread::sleep(Duration::from_secs(3600));
        }
    }

    fn build_output_stream<T>(
        device: &Device,
        config: &StreamConfig,
        receiver: Receiver<Vec<f32>>,
    ) -> Result<cpal::Stream>
    where
        T: Sample + SizedSample + Send + 'static,
        T: cpal::FromSample<f32>,
    {
        let err_fn = |err| eprintln!("output stream error: {}", err);
        let mut pending: Vec<f32> = Vec::new();

        let stream = device.build_output_stream(
            config,
            move |out: &mut [T], _| {
                let mut i = 0usize;
                while i < out.len() {
                    if pending.is_empty() {
                        match receiver.try_recv() {
                            Ok(buf) => pending = buf,
                            Err(TryRecvError::Empty) | Err(TryRecvError::Disconnected) => {
                                for sample in &mut out[i..] {
                                    *sample = T::from_sample(0.0f32);
                                }
                                return;
                            }
                        }
                    }

                    let take = (out.len() - i).min(pending.len());
                    for j in 0..take {
                        out[i + j] = T::from_sample(pending[j]);
                    }
                    pending.drain(0..take);
                    i += take;
                }
            },
            err_fn,
            None,
        )?;

        Ok(stream)
    }

    #[cfg(target_os = "windows")]
    fn run_wasapi_loopback_capture(
        sender: Sender<(String, Vec<f32>)>,
        device_id: String,
        pipeline: DspPipeline,
    ) -> Result<()> {
        unsafe {
            CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;

            let enumerator: IMMDeviceEnumerator =
                CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
            let device = enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?;

            let audio_client: IAudioClient = device.Activate(CLSCTX_ALL, None)?;

            let mix_format = audio_client.GetMixFormat()?;

            let stream_flags = AUDCLNT_STREAMFLAGS_LOOPBACK;
            let duration_100ns = 10_000_000i64 / 20; // 50ms
            audio_client.Initialize(
                AUDCLNT_SHAREMODE_SHARED,
                stream_flags,
                duration_100ns,
                0,
                mix_format,
                None,
            )?;

            let capture_client: IAudioCaptureClient = audio_client.GetService()?;
            audio_client.Start()?;

            let wf = *mix_format;
            let channels = wf.nChannels as usize;
            let bits_per_sample = wf.wBitsPerSample;
            let is_float = wf.wFormatTag == 3 || (wf.wFormatTag == 0xFFFE && bits_per_sample == 32);

            loop {
                let mut packet_size = capture_client.GetNextPacketSize()?;

                while packet_size > 0 {
                    let mut data_ptr = std::ptr::null_mut();
                    let mut frames = 0u32;
                    let mut flags = 0u32;

                    capture_client.GetBuffer(
                        &mut data_ptr,
                        &mut frames,
                        &mut flags,
                        None,
                        None,
                    )?;

                    let total_samples = frames as usize * channels;
                    let mut samples = vec![0.0f32; total_samples];

                    if flags & (AUDCLNT_BUFFERFLAGS_SILENT.0 as u32) == 0 && !data_ptr.is_null() {
                        if is_float {
                            let in_slice =
                                std::slice::from_raw_parts(data_ptr as *const f32, total_samples);
                            samples.copy_from_slice(in_slice);
                        } else if bits_per_sample == 16 {
                            let in_slice =
                                std::slice::from_raw_parts(data_ptr as *const i16, total_samples);
                            for (i, s) in in_slice.iter().enumerate() {
                                samples[i] = *s as f32 / i16::MAX as f32;
                            }
                        }
                    }

                    capture_client.ReleaseBuffer(frames)?;

                    if !samples.is_empty() {
                        let processed = pipeline.process(&samples);
                        let _ = sender.try_send((device_id.clone(), processed));
                    }

                    packet_size = capture_client.GetNextPacketSize()?;
                }

                thread::sleep(Duration::from_millis(5));
            }

            #[allow(unreachable_code)]
            {
                CoTaskMemFree(Some(mix_format as _));
                Ok(())
            }
        }
    }
}
