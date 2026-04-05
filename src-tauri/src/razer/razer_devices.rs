use zbus::{proxy, Result};

#[proxy(
    interface = "razer.devices",
    default_service = "org.razer",
    default_path = "/org/razer"
)]
pub trait RazerDevices {
    #[zbus(name = "getDevices")]
    async fn get_devices(self) -> Result<Vec<String>>;
}
