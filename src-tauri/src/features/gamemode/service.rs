use super::{model::DeviceGamemode, proxy::RazerDeviceGamemodeProxy};

pub struct Service {
    conn: zbus::Connection,
}

impl Service {
    pub fn new(conn: zbus::Connection) -> Self {
        Self { conn }
    }

    pub async fn get_gamemode(&self, serial: &str) -> zbus::Result<DeviceGamemode> {
        let path = format!("/org/razer/device/{serial}");
        let proxy = RazerDeviceGamemodeProxy::new(&self.conn, path).await?;

        Ok(DeviceGamemode {
            enabled: proxy.get_gamemode().await?,
        })
    }

    pub async fn set_gamemode(&self, serial: &str, enabled: bool) -> zbus::Result<()> {
        let path = format!("/org/razer/device/{serial}");
        let proxy = RazerDeviceGamemodeProxy::new(&self.conn, path).await?;

        proxy.set_gamemode(enabled).await
    }
}
