use zbus::{proxy, Result};

#[proxy(interface = "razer.device.power", default_service = "org.razer")]
pub trait RazerDevicePower {
    // Properties
    #[zbus(name = "getBattery")]
    async fn get_battery(&self) -> Result<f32>;

    #[zbus(name = "getIdleTime")]
    async fn get_idle_time(&self) -> Result<u16>;

    #[zbus(name = "getLowBatteryThreshold")]
    async fn get_low_battery_threshold(&self) -> Result<u8>;

    #[zbus(name = "isCharging")]
    async fn is_charging(&self) -> Result<bool>;

    // Methods
    #[zbus(name = "setLowBatteryThreshold")]
    async fn set_low_battery_threshold(&self, threshold: u8) -> Result<()>;

    #[zbus(name = "setIdleTime")]
    async fn set_idle_time(&self, idle_time: u16) -> Result<()>;
}
