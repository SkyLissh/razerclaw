use zbus::{Connection, Result};

use crate::models::{DeviceDetail, DeviceDetailParts, DeviceSummary};
use crate::razer::{RazerDeviceMiscProxy, RazerDevicesProxy, RazerProxyBuilder};

pub struct RazerService {
    conn: Connection,
}

impl RazerService {
    pub async fn new(conn: Connection) -> Self {
        Self { conn }
    }

    pub async fn get_devices(&self) -> Result<Vec<DeviceSummary>> {
        let daemon_proxy = RazerDevicesProxy::new(&self.conn).await?;
        let serials = daemon_proxy.get_devices().await?;

        let futures = serials.into_iter().map(|serial| {
            let conn = self.conn.clone();

            async move {
                let misc = RazerProxyBuilder::misc(&conn, &serial).await?;

                DeviceSummary::assemble(&misc).await
            }
        });

        futures::future::join_all(futures)
            .await
            .into_iter()
            .collect()
    }

    pub async fn get_device_by_serial(&self, serial: &str) -> Result<DeviceDetail> {
        let misc = RazerProxyBuilder::misc(&self.conn, serial).await?;

        let (dpi, brightness, power, gamemode) = futures::future::join4(
            RazerProxyBuilder::dpi(&self.conn, serial),
            RazerProxyBuilder::brightness(&self.conn, serial),
            RazerProxyBuilder::power(&self.conn, serial),
            RazerProxyBuilder::gamemode(&self.conn, serial),
        )
        .await;

        DeviceDetail::assemble(DeviceDetailParts {
            misc: &misc,
            dpi: &dpi?,
            brightness: &brightness?,
            power: &power?,
            gamemode: &gamemode?,
        })
        .await
    }

    pub async fn update_poll_rate(&self, serial: &str, poll_rate: u16) -> Result<()> {
        let path = format!("/org/razer/device/{}", serial);
        let misc = RazerDeviceMiscProxy::builder(&self.conn)
            .path(path)?
            .build()
            .await?;

        misc.set_poll_rate(poll_rate).await
    }
}
