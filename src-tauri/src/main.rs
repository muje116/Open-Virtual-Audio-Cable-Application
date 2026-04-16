// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use vac::AppState;
use vac::audio::AudioEngine;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let audio_engine = Arc::new(Mutex::new(AudioEngine::new().await));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState(audio_engine))
        .invoke_handler(tauri::generate_handler![
            vac::commands::get_audio_devices,
            vac::commands::start_audio_capture,
            vac::commands::stop_audio_capture,
            vac::commands::create_virtual_device,
            vac::commands::delete_virtual_device,
            vac::commands::set_route,
            vac::commands::get_routes,
            vac::commands::set_volume,
            vac::commands::set_mute,
            vac::commands::save_preset,
            vac::commands::load_preset,
            vac::commands::get_presets,
            vac::commands::delete_preset,
        ])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }
            
            // Setup system tray (simplified for now)
            // Full tray integration requires Tauri v2 tray plugin
            let _ = app;
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
