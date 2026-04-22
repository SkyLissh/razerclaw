// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    nvidia_wayland_workaround();
    razerclaw_lib::run()
}

fn nvidia_wayland_workaround() {
    // NVIDIA's Wayland support is very bad, and it causes the app to crash on launch.
    // This is a workaround to prevent that from happening.
    #[cfg(target_os = "linux")]
    {
        let is_nvidia = std::path::Path::new("/dev/nvidia0").exists();
        let is_wayland = std::env::var("WAYLAND_DISPLAY").is_ok();

        if is_nvidia && is_wayland {
            // std::env::set_var("__NV_DISABLE_EXPLICIT_SYNC", "1");
            std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
        }
    }
}
