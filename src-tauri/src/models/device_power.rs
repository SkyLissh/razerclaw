use serde::{Deserialize, Serialize};
use zbus::Result;

use crate::razer::RazerDevicePowerProxy;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DevicePower {
    pub battery: f32,
    pub idle_time: u16,
    pub low_battery_threshold: u8,
    pub is_charging: bool,
}

impl DevicePower {
    pub async fn assemble(proxy: &RazerDevicePowerProxy<'_>) -> Result<Self> {
        Ok(Self {
            battery: proxy.get_battery().await?,
            idle_time: proxy.get_idle_time().await?,
            low_battery_threshold: proxy.get_low_battery_threshold().await?,
            is_charging: proxy.is_charging().await?,
        })
    }
}
