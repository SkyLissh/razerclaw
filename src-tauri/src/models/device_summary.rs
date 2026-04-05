use serde::Serialize;
use zbus::Result;

use crate::razer::RazerDeviceMiscProxy;

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DeviceType {
    Mouse,
    Keyboard,
    Headset,
    Other(String),
}

impl From<String> for DeviceType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "mouse" => DeviceType::Mouse,
            "keyboard" => DeviceType::Keyboard,
            "headset" => DeviceType::Headset,
            other => DeviceType::Other(other.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct DeviceSummary {
    pub name: String,
    pub serial: String,
    #[serde(rename = "type")]
    pub device_type: DeviceType,
    pub image: String,
    pub driver_version: String,
    pub firmware: String,
}

impl DeviceSummary {
    pub async fn assemble(proxy: &RazerDeviceMiscProxy<'_>) -> Result<Self> {
        Ok(Self {
            serial: proxy.get_serial().await?,
            name: proxy.get_device_name().await?,
            device_type: DeviceType::from(proxy.get_device_type().await?),
            image: proxy.get_device_image().await?,
            driver_version: proxy.get_driver_version().await?,
            firmware: proxy.get_firmware().await?,
        })
    }
}
