use super::model::DevicePower;
use super::proxy::RazerDevicePowerProxy;

pub struct Service {
    conn: zbus::Connection,
}

impl Service {
    pub fn new(conn: zbus::Connection) -> Self {
        Self { conn }
    }

    pub async fn get_power_info(&self, serial: &str) -> zbus::Result<DevicePower> {
        let path = format!("/org/razer/device/{serial}");

        let proxy = RazerDevicePowerProxy::new(&self.conn, path).await?;
        let (battery, idle_time, low_battery_threshold, is_charging) = futures::try_join!(
            proxy.get_battery(),
            proxy.get_idle_time(),
            proxy.get_low_battery_threshold(),
            proxy.is_charging()
        )?;

        Ok(DevicePower {
            battery: battery as f32,
            idle_time,
            low_battery_threshold,
            is_charging,
        })
    }

    pub async fn set_low_battery_threshold(&self, serial: &str, threshold: u8) -> zbus::Result<()> {
        let path = format!("/org/razer/device/{serial}");
        let proxy = RazerDevicePowerProxy::new(&self.conn, path).await?;
        proxy.set_low_battery_threshold(threshold).await
    }

    pub async fn set_idle_time(&self, serial: &str, idle_time: u16) -> zbus::Result<()> {
        let path = format!("/org/razer/device/{serial}");
        let proxy = RazerDevicePowerProxy::new(&self.conn, path).await?;
        proxy.set_idle_time(idle_time).await
    }
}
