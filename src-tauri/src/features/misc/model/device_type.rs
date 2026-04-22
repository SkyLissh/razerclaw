use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DeviceType {
    Mouse,
    Keyboard,
    Headset,
    Other,
}

impl From<String> for DeviceType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "mouse" => DeviceType::Mouse,
            "keyboard" => DeviceType::Keyboard,
            "headset" => DeviceType::Headset,
            _ => DeviceType::Other,
        }
    }
}
