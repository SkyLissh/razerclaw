use serde::Serialize;
use zbus::Result;

use crate::models::{DeviceDpi, DevicePower, DeviceSummary};
use crate::razer::{
    RazerDeviceBrightnessProxy, RazerDeviceDpiProxy, RazerDeviceGamemodeProxy,
    RazerDeviceMiscProxy, RazerDevicePowerProxy,
};

pub struct DeviceDetailParts<'a> {
    pub misc: &'a RazerDeviceMiscProxy<'a>,
    pub dpi: &'a Option<RazerDeviceDpiProxy<'a>>,
    pub power: &'a Option<RazerDevicePowerProxy<'a>>,
    pub brightness: &'a Option<RazerDeviceBrightnessProxy<'a>>,
    pub gamemode: &'a Option<RazerDeviceGamemodeProxy<'a>>,
}

#[derive(Debug, Serialize, Clone)]
pub struct DeviceDetail {
    #[serde(flatten)]
    pub summary: DeviceSummary,
    pub poll_rate: Option<i32>,
    pub mode: Option<String>,
    pub supported_poll_rates: Option<Vec<u16>>,
    pub matrix_dimensions: Option<Vec<i32>>,
    pub razer_urls: Option<String>,
    pub has_dedicated_macro_keys: bool,
    pub has_matrix: bool,
    pub vid: i32,
    pub pid: i32,
    pub firmware: String,

    // DPI properties
    pub dpi: Option<DeviceDpi>,

    // Power properties
    pub power: Option<DevicePower>,

    // Brightness properties
    pub brightness: Option<f32>,

    // Gamemode properties
    pub gamemode_enabled: Option<bool>,
}

impl DeviceDetail {
    pub async fn assemble(proxy: DeviceDetailParts<'_>) -> Result<Self> {
        Ok(Self {
            summary: DeviceSummary::assemble(&proxy.misc).await?,
            poll_rate: proxy.misc.get_poll_rate().await.ok(),
            mode: proxy.misc.get_device_mode().await.ok(),
            supported_poll_rates: proxy.misc.get_supported_poll_rates().await.ok(),
            matrix_dimensions: proxy.misc.get_matrix_dimensions().await.ok(),
            razer_urls: proxy.misc.get_razer_urls().await.ok(),
            has_dedicated_macro_keys: proxy.misc.has_dedicated_macro_keys().await?,
            has_matrix: proxy.misc.has_matrix().await?,
            vid: proxy.misc.get_vid_pid().await?[0],
            pid: proxy.misc.get_vid_pid().await?[1],
            firmware: proxy.misc.get_firmware().await?,
            dpi: match proxy.dpi {
                Some(dpi_proxy) => Some(DeviceDpi::assemble(&dpi_proxy).await?),
                None => None,
            },
            power: match proxy.power {
                Some(power_proxy) => Some(DevicePower::assemble(&power_proxy).await?),
                None => None,
            },
            brightness: match proxy.brightness {
                Some(brightness_proxy) => Some(brightness_proxy.get_brightness().await?),
                None => None,
            },
            gamemode_enabled: match proxy.gamemode {
                Some(gamemode_proxy) => Some(gamemode_proxy.get_gamemode().await?),
                None => None,
            },
        })
    }
}
