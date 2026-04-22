use tauri::State;

use crate::AppState;

use super::model::DevicePower;

#[tauri::command]
pub async fn get_power(state: State<'_, AppState>, serial: &str) -> Result<DevicePower, String> {
    let service = &state.features.power.service();

    service
        .get_power_info(serial)
        .await
        .map_err(|e| format!("Failed to get power info: {}", e))
}

#[tauri::command]
pub async fn set_low_battery_threshold(
    state: State<'_, AppState>,
    serial: &str,
    threshold: u8,
) -> Result<(), String> {
    let service = &state.features.power.service();

    service
        .set_low_battery_threshold(serial, threshold)
        .await
        .map_err(|e| format!("Failed to set low battery threshold: {}", e))
}

#[tauri::command]
pub async fn set_idle_time(
    state: State<'_, AppState>,
    serial: &str,
    idle_time: u16,
) -> Result<(), String> {
    let service = &state.features.power.service();

    service
        .set_idle_time(serial, idle_time)
        .await
        .map_err(|e| format!("Failed to set idle time: {}", e))
}
