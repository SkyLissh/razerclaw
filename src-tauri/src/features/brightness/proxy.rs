use zbus::{proxy, Result};

#[proxy(
    interface = "razer.device.lighting.brightness",
    default_service = "org.razer"
)]
pub trait RazerDeviceBrightness {
    #[zbus(name = "getBrightness")]
    async fn get_brightness(&self) -> Result<f32>;

    #[zbus(name = "setBrightness")]
    async fn set_brightness(&self, brightness: f32) -> Result<()>;
}
