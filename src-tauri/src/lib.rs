pub mod audio;
pub mod commands;
pub mod config;
pub mod devices;
pub mod dsp;
pub mod routing;

use audio::AudioEngine;
use devices::DeviceManager;
use routing::RoutingMatrix;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct InnerState {
    pub audio_engine: AudioEngine,
    pub routing_matrix: RoutingMatrix,
    pub device_manager: DeviceManager,
}

#[derive(Clone)]
pub struct AppState(pub Arc<Mutex<InnerState>>);
