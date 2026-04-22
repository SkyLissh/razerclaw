use super::{model, proxy};

pub struct Service {
    conn: zbus::Connection,
}

impl Service {
    pub fn new(conn: zbus::Connection) -> Self {
        Self { conn }
    }

    pub async fn get_dpi(&self, serial: &str) -> zbus::Result<model::DeviceDpi> {
        let path = format!("/org/razer/device/{serial}");
        let proxy = proxy::RazerDeviceDpiProxy::new(&self.conn, path).await?;

        let (dpi, stages, max_dpi) =
            futures::try_join!(proxy.get_dpi(), proxy.get_dpi_stages(), proxy.max_dpi())?;

        Ok(model::DeviceDpi {
            dpi: dpi.into(),
            stages: stages.into(),
            max_dpi,
        })
    }

    pub async fn set_dpi(&self, serial: &str, stage: model::DpiStage) -> zbus::Result<()> {
        let path = format!("/org/razer/device/{serial}");
        let proxy = proxy::RazerDeviceDpiProxy::new(&self.conn, path).await?;

        proxy.set_dpi(stage.x, stage.y).await
    }

    pub async fn set_dpi_stages(
        &self,
        serial: &str,
        active_stage: u8,
        stages: Vec<model::DpiStage>,
    ) -> zbus::Result<()> {
        let path = format!("/org/razer/device/{serial}");
        let proxy = proxy::RazerDeviceDpiProxy::new(&self.conn, path).await?;

        let stages = stages.into_iter().map(|stage| (stage.x, stage.y)).collect();

        proxy.set_dpi_stages(active_stage, stages).await
    }
}
