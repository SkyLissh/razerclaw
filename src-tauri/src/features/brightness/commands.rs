use tauri::State;

use crate::AppState;

use super::model::DeviceBrightness;

#[tauri::command]
pub async fn get_brightness(
    state: State<'_, AppState>,
    serial: &str,
) -> Result<DeviceBrightness, String> {
    let service = &state.features.brightness.service();

    service
        .get_brightness(serial)
        .await
        .map_err(|e| format!("Failed to get brightness: {}", e))
}

#[tauri::command]
pub async fn set_brightness(
    state: State<'_, AppState>,
    serial: &str,
    brightness: f32,
) -> Result<(), String> {
    let service = &state.features.brightness.service();

    service
        .set_brightness(serial, brightness)
        .await
        .map_err(|e| format!("Failed to set brightness: {}", e))
}
