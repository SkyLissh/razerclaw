use zbus::Connection;

use super::{
    mapper,
    model::DeviceSummary,
    proxy::{RazerDeviceMiscProxy, RazerDevicesProxy},
};

pub struct MiscService {
    conn: Connection,
}

impl MiscService {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    pub async fn get_devices(&self) -> zbus::Result<Vec<DeviceSummary>> {
        let proxy = RazerDevicesProxy::new(&self.conn).await?;
        let serials = proxy.get_devices().await?;

        let futures = serials.into_iter().map(|serial| {
            let conn = self.conn.clone();

            async move {
                let path = format!("/org/razer/device/{serial}");

                mapper::summary_from_proxy(&conn, &path).await
            }
        });

        futures::future::join_all(futures)
            .await
            .into_iter()
            .collect()
    }

    pub async fn get_device(&self, serial: &str) -> zbus::Result<DeviceSummary> {
        let path = format!("/org/razer/device/{serial}");

        mapper::summary_from_proxy(&self.conn, &path).await
    }

    pub async fn set_poll_rate(&self, serial: &str, rate: u16) -> zbus::Result<()> {
        let path = format!("/org/razer/device/{serial}");
        let misc = RazerDeviceMiscProxy::new(&self.conn, path).await?;

        misc.set_poll_rate(rate).await
    }

    pub async fn suspend_device(&self, serial: &str) -> zbus::Result<()> {
        let path = format!("/org/razer/device/{serial}");
        let misc = RazerDeviceMiscProxy::new(&self.conn, path).await?;

        misc.suspend_device().await
    }

    pub async fn resume_device(&self, serial: &str) -> zbus::Result<()> {
        let path = format!("/org/razer/device/{serial}");
        let misc = RazerDeviceMiscProxy::new(&self.conn, path).await?;

        misc.resume_device().await
    }
}
