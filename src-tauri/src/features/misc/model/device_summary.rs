use serde::Serialize;

use super::{DeviceCapability, DeviceType};

#[derive(Debug, Serialize, Clone)]
pub struct DeviceSummary {
    pub name: String,
    pub serial: String,
    #[serde(rename = "type")]
    pub device_type: DeviceType,
    pub image: String,
    pub mode: String,
    pub driver_version: String,
    pub firmware: String,
    pub matrix_dimensions: Option<Vec<i32>>,
    pub poll_rate: Option<i32>,
    pub razer_urls: String,
    pub supported_poll_rates: Option<Vec<u16>>,
    pub vid: i32,
    pub pid: i32,
    pub has_dedicated_macro_keys: bool,
    pub has_matrix: bool,
    pub capabilities: Vec<DeviceCapability>,
}
