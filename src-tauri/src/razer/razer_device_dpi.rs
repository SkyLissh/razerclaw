use zbus::{proxy, Result};

#[proxy(interface = "razer.device.dpi", default_service = "org.razer")]
pub trait RazerDeviceDpi {
    #[zbus(name = "getDPI")]
    async fn get_dpi(&self) -> Result<Vec<i32>>;

    #[zbus(name = "getDPIStages")]
    async fn get_dpi_stages(&self) -> Result<(u8, Vec<(u16, u16)>)>;

    #[zbus(name = "maxDPI")]
    async fn max_dpi(&self) -> Result<i32>;

    #[zbus(name = "setDPI")]
    async fn set_dpi(&self, x_dpi: u16, y_dpi: u16) -> Result<()>;

    #[zbus(name = "setDPIStages")]
    async fn set_dpi_stages(&self, active_stage: u8, stages: Vec<(u16, u16)>) -> Result<()>;
}
