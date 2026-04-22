use tauri::State;

use super::model::DeviceSummary;
use crate::AppState;

#[tauri::command]
pub async fn get_devices(state: State<'_, AppState>) -> Result<Vec<DeviceSummary>, String> {
    let feature = &state.features.misc;

    feature
        .service()
        .get_devices()
        .await
        .map_err(|e| format!("Failed to get devices: {}", e))
}

#[tauri::command]
pub async fn get_device_by_serial(
    state: State<'_, AppState>,
    serial: &str,
) -> Result<DeviceSummary, String> {
    let feature = &state.features.misc;

    feature
        .service()
        .get_device(serial)
        .await
        .map_err(|e| format!("Failed to get device by serial: {}", e))
}

#[tauri::command]
pub async fn set_poll_rate(
    state: State<'_, AppState>,
    serial: &str,
    poll_rate: u16,
) -> Result<(), String> {
    let feature = &state.features.misc;

    feature
        .service()
        .set_poll_rate(serial, poll_rate)
        .await
        .map_err(|e| format!("Failed to update poll rate: {}", e))
}

#[tauri::command]
pub async fn suspend_device(state: State<'_, AppState>, serial: &str) -> Result<(), String> {
    let feature = &state.features.misc;

    feature
        .service()
        .suspend_device(serial)
        .await
        .map_err(|e| format!("Failed to suspend device: {}", e))
}

#[tauri::command]
pub async fn resume_device(state: State<'_, AppState>, serial: &str) -> Result<(), String> {
    let feature = &state.features.misc;

    feature
        .service()
        .resume_device(serial)
        .await
        .map_err(|e| format!("Failed to resume device: {}", e))
}
