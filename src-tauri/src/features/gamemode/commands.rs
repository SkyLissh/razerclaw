use tauri::State;

use crate::AppState;

use super::model::DeviceGamemode;

#[tauri::command]
pub async fn get_gamemode(
    state: State<'_, AppState>,
    serial: &str,
) -> Result<DeviceGamemode, String> {
    let service = &state.features.gamemode.service();

    service
        .get_gamemode(serial)
        .await
        .map_err(|e| format!("Failed to get game mode: {}", e))
}

#[tauri::command]
pub async fn set_gamemode(
    state: State<'_, AppState>,
    serial: &str,
    enabled: bool,
) -> Result<(), String> {
    let service = &state.features.gamemode.service();

    service
        .set_gamemode(serial, enabled)
        .await
        .map_err(|e| format!("Failed to set game mode: {}", e))
}
