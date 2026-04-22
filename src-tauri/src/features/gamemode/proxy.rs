use zbus::{proxy, Result};

#[proxy(interface = "razer.device.led.gamemode", default_service = "org.razer")]
pub trait RazerDeviceGamemode {
    #[zbus(name = "getGameMode")]
    async fn get_gamemode(&self) -> Result<bool>;

    #[zbus(name = "setGameMode")]
    async fn set_gamemode(&self, enabled: bool) -> Result<()>;
}
