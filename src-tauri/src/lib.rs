mod features;

use features::{brightness, dpi, gamemode, misc, power};

struct Features {
    pub misc: misc::Feature,
    pub dpi: dpi::Feature,
    pub power: power::Feature,
    pub brightness: brightness::Feature,
    pub gamemode: gamemode::Feature,
}

struct AppState {
    pub features: Features,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = tauri::async_runtime::block_on(async {
        // Initialize D-Bus connection and store it in the app state
        let dbus_conn = zbus::Connection::session()
            .await
            .expect("Failed to connect to D-Bus session bus");

        let features = Features {
            misc: misc::Feature::new(dbus_conn.clone()),
            dpi: dpi::Feature::new(dbus_conn.clone()),
            power: power::Feature::new(dbus_conn.clone()),
            brightness: brightness::Feature::new(dbus_conn.clone()),
            gamemode: gamemode::Feature::new(dbus_conn.clone()),
        };

        AppState { features }
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            // Misc commands
            misc::commands::get_devices,
            misc::commands::get_device_by_serial,
            misc::commands::set_poll_rate,
            misc::commands::suspend_device,
            misc::commands::resume_device,
            // DPI commands
            dpi::commands::set_dpi,
            dpi::commands::set_dpi_stages,
            dpi::commands::get_dpi,
            // Power commands
            power::commands::get_power,
            power::commands::set_low_battery_threshold,
            power::commands::set_idle_time,
            // Brightness commands
            brightness::commands::get_brightness,
            brightness::commands::set_brightness,
            // Game mode commands
            gamemode::commands::get_gamemode,
            gamemode::commands::set_gamemode,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
