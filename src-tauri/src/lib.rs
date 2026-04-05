mod commands;
mod models;
mod razer;
mod services;

struct AppState {
    pub services: services::Services,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = tauri::async_runtime::block_on(async {
        // Initialize D-Bus connection and store it in the app state
        let dbus_conn = zbus::Connection::session()
            .await
            .expect("Failed to connect to D-Bus session bus");
        let razer_service = services::RazerService::new(dbus_conn.clone()).await;

        let services = services::Services {
            razer: razer_service,
        };

        AppState { services }
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            commands::razer::get_devices,
            commands::razer::get_device_by_serial,
            commands::razer::update_poll_rate,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
