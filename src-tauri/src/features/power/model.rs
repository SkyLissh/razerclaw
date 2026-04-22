use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DevicePower {
    pub battery: f32,
    pub idle_time: u16,
    pub low_battery_threshold: u8,
    pub is_charging: bool,
}
