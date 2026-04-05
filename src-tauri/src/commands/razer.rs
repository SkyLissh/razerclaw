use tauri::State;

use crate::models::{DeviceDetail, DeviceSummary};
use crate::AppState;

#[tauri::command]
pub async fn get_devices(state: State<'_, AppState>) -> Result<Vec<DeviceSummary>, String> {
    let razer_service = &state.services.razer;

    razer_service
        .get_devices()
        .await
        .map_err(|e| format!("Failed to get devices: {}", e))
}

#[tauri::command]
pub async fn get_device_by_serial(
    state: State<'_, AppState>,
    serial: &str,
) -> Result<DeviceDetail, String> {
    let razer_service = &state.services.razer;

    razer_service
        .get_device_by_serial(serial)
        .await
        .map_err(|e| format!("Failed to get device by serial: {}", e))
}

#[tauri::command]
pub async fn update_poll_rate(
    state: State<'_, AppState>,
    serial: &str,
    poll_rate: u16,
) -> Result<(), String> {
    let razer_service = &state.services.razer;

    razer_service
        .update_poll_rate(serial, poll_rate)
        .await
        .map_err(|e| format!("Failed to update poll rate: {}", e))
}
