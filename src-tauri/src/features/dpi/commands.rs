use tauri::State;

use crate::AppState;

use super::model;

#[tauri::command]
pub async fn get_dpi(state: State<'_, AppState>, serial: &str) -> Result<model::DeviceDpi, String> {
    let service = &state.features.dpi.service();

    service
        .get_dpi(serial)
        .await
        .map_err(|e| format!("Failed to get DPI: {}", e))
}

#[tauri::command]
pub async fn set_dpi(state: State<'_, AppState>, serial: &str, dpi: u16) -> Result<(), String> {
    let service = &state.features.dpi.service();

    service
        .set_dpi(serial, dpi.into())
        .await
        .map_err(|e| format!("Failed to set DPI: {}", e))
}

#[tauri::command]
pub async fn set_dpi_stages(
    state: State<'_, AppState>,
    serial: &str,
    active_stage: u8,
    stages: Vec<u16>,
) -> Result<(), String> {
    let service = &state.features.dpi.service();

    service
        .set_dpi_stages(
            serial,
            active_stage,
            stages.into_iter().map(|stage| stage.into()).collect(),
        )
        .await
        .map_err(|e| format!("Failed to set DPI stages: {}", e))
}
