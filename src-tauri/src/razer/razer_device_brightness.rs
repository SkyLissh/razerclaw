use zbus::{proxy, Result};

#[proxy(interface = "razer.device.brightness", default_service = "org.razer")]
pub trait RazerDeviceBrightness {
    // Properties
    #[zbus(name = "getBrightness")]
    async fn get_brightness(&self) -> Result<f32>;

    // Methods
    #[zbus(name = "setBrightness")]
    async fn set_brightness(&self, brightness: f32) -> Result<()>;
}
