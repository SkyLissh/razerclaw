use zbus::{proxy, Result};

#[proxy(interface = "razer.device.gamemode", default_service = "org.razer")]
pub trait RazerDeviceGamemode {
    // Properties
    #[zbus(name = "getGameMode")]
    async fn get_gamemode(&self) -> Result<bool>;

    // Methods
    #[zbus(name = "setGameMode")]
    async fn set_gamemode(&self, enabled: bool) -> Result<()>;
}
