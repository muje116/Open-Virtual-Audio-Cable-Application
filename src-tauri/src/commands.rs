use crate::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDeviceInfo {
    pub id: String,
    pub name: String,
    pub device_type: String,
    pub sample_rate: u32,
    pub channels: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    pub input_id: String,
    pub output_id: String,
    pub volume: f32,
    pub muted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    pub routes: Vec<Route>,
    pub created_at: String,
}

#[tauri::command]
pub async fn get_audio_devices(state: State<'_, AppState>) -> Result<Vec<AudioDeviceInfo>, String> {
    let engine = state.0.lock().await;
    
    let input_devices: Vec<AudioDeviceInfo> = engine
        .get_input_devices()
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(|d| AudioDeviceInfo {
            id: d.id,
            name: d.name,
            device_type: format!("{:?}", d.device_type),
            sample_rate: d.sample_rate,
            channels: d.channels,
        })
        .collect();

    let output_devices: Vec<AudioDeviceInfo> = engine
        .get_output_devices()
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(|d| AudioDeviceInfo {
            id: d.id,
            name: d.name,
            device_type: format!("{:?}", d.device_type),
            sample_rate: d.sample_rate,
            channels: d.channels,
        })
        .collect();

    Ok([input_devices, output_devices].concat())
}

#[tauri::command]
pub async fn start_audio_capture(
    device_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let engine = state.0.lock().await;
    engine
        .start_capture(device_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_audio_capture(
    device_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let engine = state.0.lock().await;
    engine
        .stop_capture(device_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_virtual_device(name: String) -> Result<String, String> {
    // TODO: Implement virtual device creation
    // This will require platform-specific driver code
    Ok(format!("Created virtual device: {}", name))
}

#[tauri::command]
pub async fn delete_virtual_device(device_id: String) -> Result<(), String> {
    // TODO: Implement virtual device deletion
    Ok(())
}

#[tauri::command]
pub async fn set_route(route: Route) -> Result<(), String> {
    // TODO: Implement routing logic
    Ok(())
}

#[tauri::command]
pub async fn get_routes() -> Result<Vec<Route>, String> {
    // TODO: Return current routes
    Ok(Vec::new())
}

#[tauri::command]
pub async fn set_volume(route_id: String, volume: f32) -> Result<(), String> {
    // TODO: Implement volume control
    Ok(())
}

#[tauri::command]
pub async fn set_mute(route_id: String, muted: bool) -> Result<(), String> {
    // TODO: Implement mute control
    Ok(())
}

#[tauri::command]
pub async fn save_preset(preset: Preset) -> Result<(), String> {
    // TODO: Save preset to disk
    Ok(())
}

#[tauri::command]
pub async fn load_preset(name: String) -> Result<Preset, String> {
    // TODO: Load preset from disk
    Err("Not implemented".to_string())
}

#[tauri::command]
pub async fn get_presets() -> Result<Vec<String>, String> {
    // TODO: Return list of preset names
    Ok(Vec::new())
}

#[tauri::command]
pub async fn delete_preset(name: String) -> Result<(), String> {
    // TODO: Delete preset
    Ok(())
}
