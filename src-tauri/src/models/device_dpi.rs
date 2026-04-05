use serde::{Deserialize, Serialize};
use zbus::Result;

use crate::razer::RazerDeviceDpiProxy;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceDpiStages {
    pub active: u8,
    pub stages: Vec<DeviceDpiStage>,
}

impl From<(u8, Vec<(u16, u16)>)> for DeviceDpiStages {
    fn from((active, stages): (u8, Vec<(u16, u16)>)) -> Self {
        Self {
            active,
            stages: stages
                .into_iter()
                .map(|(x, y)| DeviceDpiStage { x, y })
                .collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceDpiStage {
    pub x: u16,
    pub y: u16,
}

impl From<Vec<i32>> for DeviceDpiStage {
    fn from(dpi: Vec<i32>) -> Self {
        Self {
            x: dpi[0] as u16,
            y: dpi[1] as u16,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceDpi {
    pub dpi: DeviceDpiStage,
    pub stages: DeviceDpiStages,
    pub max_dpi: i32,
}

impl DeviceDpi {
    pub async fn assemble(proxy: &RazerDeviceDpiProxy<'_>) -> Result<Self> {
        Ok(Self {
            dpi: proxy.get_dpi().await?.into(),
            stages: proxy.get_dpi_stages().await?.into(),
            max_dpi: proxy.max_dpi().await?,
        })
    }
}
