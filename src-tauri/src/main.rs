#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use vac::audio::AudioEngine;
use vac::devices::DeviceManager;
use vac::routing::RoutingMatrix;
use vac::{AppState, InnerState};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let audio_engine = AudioEngine::new().await;
    let routing_matrix = RoutingMatrix::new();
    let device_manager = DeviceManager::new();

    let state = AppState(Arc::new(Mutex::new(InnerState {
        audio_engine,
        routing_matrix,
        device_manager,
    })));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            vac::commands::get_audio_devices,
            vac::commands::start_audio_capture,
            vac::commands::stop_audio_capture,
            vac::commands::create_virtual_device,
            vac::commands::delete_virtual_device,
            vac::commands::set_route,
            vac::commands::remove_route,
            vac::commands::get_routes,
            vac::commands::set_volume,
            vac::commands::set_mute,
            vac::commands::set_device_dsp,
            vac::commands::get_device_dsp,
            vac::commands::save_preset,
            vac::commands::load_preset,
            vac::commands::get_presets,
            vac::commands::delete_preset,
            vac::commands::export_preset,
            vac::commands::import_preset,
        ])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }

            let _ = app;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
