#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use vac::audio::AudioEngine;
use vac::config::AppConfig;
use vac::devices::DeviceManager;
use vac::routing::RoutingMatrix;
use vac::{AppState, InnerState};
use std::sync::Arc;
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::Manager;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let audio_engine = AudioEngine::new().await;
    let routing_matrix = RoutingMatrix::new();
    let device_manager = DeviceManager::new();
    let app_config = AppConfig::load().unwrap_or_default();

    let state = AppState(Arc::new(Mutex::new(InnerState {
        audio_engine,
        routing_matrix,
        device_manager,
        app_config,
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
            vac::commands::get_runtime_diagnostics,
            vac::commands::get_audio_status,
            vac::commands::get_config,
            vac::commands::update_config,
        ])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }

            let config = AppConfig::load().unwrap_or_default();
            if config.start_minimized {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }

            let icon = tauri::image::Image::from_bytes(include_bytes!("../icons/32x32.png"))
                .expect("failed to load tray icon");

            let show = tauri::menu::MenuItemBuilder::with_id("show", "Show")
                .build(app)
                .expect("failed to build show menu item");
            let hide = tauri::menu::MenuItemBuilder::with_id("hide", "Hide")
                .build(app)
                .expect("failed to build hide menu item");
            let quit = tauri::menu::MenuItemBuilder::with_id("quit", "Quit")
                .build(app)
                .expect("failed to build quit menu item");
            let menu = tauri::menu::MenuBuilder::new(app)
                .item(&show)
                .item(&hide)
                .separator()
                .item(&quit)
                .build()
                .expect("failed to build tray menu");

            TrayIconBuilder::new()
                .icon(icon)
                .tooltip("Virtual Audio Cable")
                .menu(&menu)
                .on_menu_event(|app_handle, event| {
                    match event.id().as_ref() {
                        "show" => {
                            if let Some(window) = app_handle.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "hide" => {
                            if let Some(window) = app_handle.get_webview_window("main") {
                                let _ = window.hide();
                            }
                        }
                        "quit" => {
                            app_handle.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        if let Some(window) = tray.app_handle().get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)
                .expect("failed to build tray icon");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
