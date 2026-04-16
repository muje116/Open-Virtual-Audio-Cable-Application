// Library exports for testing
pub mod audio;
pub mod commands;
pub mod config;
pub mod devices;
pub mod dsp;
pub mod routing;

use audio::AudioEngine;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState(pub Arc<Mutex<AudioEngine>>);
