use crate::dsp::DspSettings;
use crate::InnerState;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDeviceInfo {
    pub id: String,
    pub name: String,
    pub device_type: String,
    pub sample_rate: u32,
    pub channels: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub input_id: String,
    pub output_id: String,
    pub volume: f32,
    pub muted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    pub routes: Vec<Route>,
    pub dsp_settings: HashMap<String, DspSettings>,
    pub created_at: String,
}

fn preset_dir() -> PathBuf {
    let base = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    let dir = base.join("vac").join("presets");
    let _ = fs::create_dir_all(&dir);
    dir
}

fn timestamp() -> String {
    chrono_now()
}

fn chrono_now() -> String {
    // Simple UTC timestamp without pulling in chrono crate
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();
    let days = secs / 86400;
    let time = secs % 86400;
    let hours = time / 3600;
    let minutes = (time % 3600) / 60;
    let seconds = time % 60;
    format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z", 1970 + (days / 365), 1, 1, hours, minutes, seconds)
}

fn convert_route(r: &crate::routing::Route) -> Route {
    Route {
        input_id: r.input_id.clone(),
        output_id: r.output_id.clone(),
        volume: r.volume * 100.0,
        muted: r.muted,
    }
}

fn convert_routes(routes: &[crate::routing::Route]) -> Vec<Route> {
    routes.iter().map(convert_route).collect()
}

fn find_route_id(inner: &InnerState, input_id: &str, output_id: &str) -> Option<String> {
    inner
        .routing_matrix
        .get_all_routes()
        .iter()
        .find(|r| r.input_id == input_id && r.output_id == output_id)
        .map(|r| r.id.clone())
}

#[tauri::command]
pub async fn get_audio_devices(state: State<'_, crate::AppState>) -> Result<Vec<AudioDeviceInfo>, String> {
    let inner = state.inner().0.lock().await;

    let mut devices = Vec::new();

    if let Ok(inputs) = inner.audio_engine.get_input_devices() {
        for d in inputs {
            devices.push(AudioDeviceInfo {
                id: d.id,
                name: d.name,
                device_type: format!("{:?}", d.device_type),
                sample_rate: d.sample_rate,
                channels: d.channels,
            });
        }
    }

    if let Ok(outputs) = inner.audio_engine.get_output_devices() {
        for d in outputs {
            devices.push(AudioDeviceInfo {
                id: d.id,
                name: d.name,
                device_type: format!("{:?}", d.device_type),
                sample_rate: d.sample_rate,
                channels: d.channels,
            });
        }
    }

    for vd in inner.device_manager.get_virtual_devices() {
        devices.push(AudioDeviceInfo {
            id: vd.id,
            name: vd.name,
            device_type: "Virtual".to_string(),
            sample_rate: vd.sample_rate,
            channels: vd.channels,
        });
    }

    Ok(devices)
}

#[tauri::command]
pub async fn start_audio_capture(
    device_id: String,
    state: State<'_, crate::AppState>,
) -> Result<(), String> {
    let inner = state.inner().0.lock().await;
    inner
        .audio_engine
        .start_capture(device_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_audio_capture(
    device_id: String,
    state: State<'_, crate::AppState>,
) -> Result<(), String> {
    let inner = state.inner().0.lock().await;
    inner
        .audio_engine
        .stop_capture(device_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_virtual_device(
    name: String,
    channels: u16,
    state: State<'_, crate::AppState>,
) -> Result<String, String> {
    let mut inner = state.inner().0.lock().await;
    let device = inner.device_manager.create_virtual_device(name, channels);
    Ok(device.id)
}

#[tauri::command]
pub async fn delete_virtual_device(
    device_id: String,
    state: State<'_, crate::AppState>,
) -> Result<(), String> {
    let mut inner = state.inner().0.lock().await;
    inner
        .device_manager
        .delete_virtual_device(&device_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_route(
    input_id: String,
    output_id: String,
    volume: f32,
    muted: bool,
    state: State<'_, crate::AppState>,
) -> Result<(), String> {
    let mut inner = state.inner().0.lock().await;
    let input_id_copy = input_id.clone();
    let output_id_copy = output_id.clone();
    inner.routing_matrix.add_route(input_id, output_id);
    // Update volume/muted on the newly created route
    let routes = inner.routing_matrix.get_all_routes();
    if let Some(route) = routes
        .iter()
        .find(|r| r.input_id == input_id_copy && r.output_id == output_id_copy)
    {
        let mut r = route.clone();
        r.volume = volume / 100.0;
        r.muted = muted;
        inner.routing_matrix.update_route(r).map_err(|e| e.to_string())?;
    }
    let all_routes = inner.routing_matrix.get_all_routes();
    inner
        .audio_engine
        .sync_routes(all_routes)
        .map_err(|e| e.to_string())?;
    inner
        .audio_engine
        .start_capture(input_id_copy)
        .await
        .map_err(|e| {
            eprintln!("start_capture warning for route add: {}", e);
            e.to_string()
        })
        .ok();
    Ok(())
}

#[tauri::command]
pub async fn remove_route(
    input_id: String,
    output_id: String,
    state: State<'_, crate::AppState>,
) -> Result<(), String> {
    let mut inner = state.inner().0.lock().await;
    if let Some(route_id) = find_route_id(&inner, &input_id, &output_id) {
        inner.routing_matrix.remove_route(&route_id);
    }
    let all_routes = inner.routing_matrix.get_all_routes();
    inner
        .audio_engine
        .sync_routes(all_routes)
        .map_err(|e| e.to_string())?;

    let has_input_routes = inner
        .routing_matrix
        .get_all_routes()
        .iter()
        .any(|r| r.input_id == input_id);
    if !has_input_routes {
        let _ = inner.audio_engine.stop_capture(input_id).await;
    }
    Ok(())
}

#[tauri::command]
pub async fn get_routes(state: State<'_, crate::AppState>) -> Result<Vec<Route>, String> {
    let inner = state.inner().0.lock().await;
    let routes = inner.routing_matrix.get_all_routes();
    Ok(convert_routes(&routes))
}

#[tauri::command]
pub async fn set_volume(
    input_id: String,
    output_id: String,
    volume: f32,
    state: State<'_, crate::AppState>,
) -> Result<(), String> {
    let mut inner = state.inner().0.lock().await;
    if let Some(route_id) = find_route_id(&inner, &input_id, &output_id) {
        if let Some(route) = inner.routing_matrix.get_route(&route_id) {
            let mut r = route.clone();
            r.volume = volume / 100.0;
            inner.routing_matrix.update_route(r).map_err(|e| e.to_string())?;
        }
    }
    let all_routes = inner.routing_matrix.get_all_routes();
    inner
        .audio_engine
        .sync_routes(all_routes)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn set_mute(
    input_id: String,
    output_id: String,
    muted: bool,
    state: State<'_, crate::AppState>,
) -> Result<(), String> {
    let mut inner = state.inner().0.lock().await;
    if let Some(route_id) = find_route_id(&inner, &input_id, &output_id) {
        if let Some(route) = inner.routing_matrix.get_route(&route_id) {
            let mut r = route.clone();
            r.muted = muted;
            inner.routing_matrix.update_route(r).map_err(|e| e.to_string())?;
        }
    }
    let all_routes = inner.routing_matrix.get_all_routes();
    inner
        .audio_engine
        .sync_routes(all_routes)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn set_device_dsp(
    device_id: String,
    settings: DspSettings,
    state: State<'_, crate::AppState>,
) -> Result<(), String> {
    let inner = state.inner().0.lock().await;
    inner
        .audio_engine
        .set_device_dsp(&device_id, settings)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_device_dsp(
    device_id: String,
    state: State<'_, crate::AppState>,
) -> Result<DspSettings, String> {
    let inner = state.inner().0.lock().await;
    inner
        .audio_engine
        .get_device_dsp(&device_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_preset(
    name: String,
    state: State<'_, crate::AppState>,
) -> Result<(), String> {
    let inner = state.inner().0.lock().await;

    let routes = convert_routes(&inner.routing_matrix.get_all_routes());

    let dsp_settings: HashMap<String, DspSettings> = inner
        .audio_engine
        .get_all_dsp_settings()
        .map_err(|e| e.to_string())?;

    let preset = Preset {
        name: name.clone(),
        routes,
        dsp_settings,
        created_at: timestamp(),
    };

    let path = preset_dir().join(format!("{}.json", sanitize_filename(&name)));
    let json = serde_json::to_string_pretty(&preset).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn load_preset(
    name: String,
    state: State<'_, crate::AppState>,
) -> Result<Preset, String> {
    let path = preset_dir().join(format!("{}.json", sanitize_filename(&name)));
    let json = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let preset: Preset = serde_json::from_str(&json).map_err(|e| e.to_string())?;

    // Restore into state
    let mut inner = state.inner().0.lock().await;
    inner.routing_matrix.clear();
    for r in &preset.routes {
        let mut route = inner.routing_matrix.add_route(r.input_id.clone(), r.output_id.clone());
        route.volume = r.volume / 100.0;
        route.muted = r.muted;
        inner
            .routing_matrix
            .update_route(route)
            .map_err(|e| e.to_string())?;
    }
    let all_routes = inner.routing_matrix.get_all_routes();
    inner
        .audio_engine
        .sync_routes(all_routes)
        .map_err(|e| e.to_string())?;

    for (device_id, settings) in &preset.dsp_settings {
        inner
            .audio_engine
            .set_device_dsp(device_id, settings.clone())
            .map_err(|e| e.to_string())?;
    }

    Ok(preset)
}

#[tauri::command]
pub async fn get_presets() -> Result<Vec<String>, String> {
    let dir = preset_dir();
    let mut presets = Vec::new();
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".json") {
                    presets.push(name.trim_end_matches(".json").to_string());
                }
            }
        }
    }
    presets.sort();
    Ok(presets)
}

#[tauri::command]
pub async fn delete_preset(name: String) -> Result<(), String> {
    let path = preset_dir().join(format!("{}.json", sanitize_filename(&name)));
    if path.exists() {
        fs::remove_file(&path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn export_preset(name: String) -> Result<String, String> {
    let path = preset_dir().join(format!("{}.json", sanitize_filename(&name)));
    fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn import_preset(preset_json: String) -> Result<String, String> {
    let preset: Preset = serde_json::from_str(&preset_json).map_err(|e| e.to_string())?;
    let path = preset_dir().join(format!("{}.json", sanitize_filename(&preset.name)));
    let json = serde_json::to_string_pretty(&preset).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(preset.name)
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect()
}
