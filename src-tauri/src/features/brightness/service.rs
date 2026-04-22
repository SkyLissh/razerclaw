use super::{model::DeviceBrightness, proxy::RazerDeviceBrightnessProxy};

pub struct Service {
    conn: zbus::Connection,
}

impl Service {
    pub fn new(conn: zbus::Connection) -> Self {
        Self { conn }
    }

    pub async fn get_brightness(&self, serial: &str) -> zbus::Result<DeviceBrightness> {
        let path = format!("/org/razer/device/{serial}");
        let proxy = RazerDeviceBrightnessProxy::new(&self.conn, path).await?;

        Ok(DeviceBrightness {
            brightness: proxy.get_brightness().await?,
        })
    }

    pub async fn set_brightness(&self, serial: &str, brightness: f32) -> zbus::Result<()> {
        let path = format!("/org/razer/device/{serial}");
        let proxy = RazerDeviceBrightnessProxy::new(&self.conn, path).await?;

        proxy.set_brightness(brightness).await
    }
}
