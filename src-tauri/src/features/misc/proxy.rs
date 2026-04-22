use zbus::{proxy, Result};

#[proxy(interface = "razer.device.misc", default_service = "org.razer")]
pub trait RazerDeviceMisc {
    // Property getters

    #[zbus(name = "getDeviceImage")]
    async fn get_device_image(&self) -> Result<String>;

    #[zbus(name = "getDeviceMode")]
    async fn get_device_mode(&self) -> Result<String>;

    #[zbus(name = "getDeviceName")]
    async fn get_device_name(&self) -> Result<String>;

    #[zbus(name = "getDeviceType")]
    async fn get_device_type(&self) -> Result<String>;

    #[zbus(name = "getDriverVersion")]
    async fn get_driver_version(&self) -> Result<String>;

    #[zbus(name = "getFirmware")]
    async fn get_firmware(&self) -> Result<String>;

    #[zbus(name = "getMatrixDimensions")]
    async fn get_matrix_dimensions(&self) -> Result<Vec<i32>>;

    #[zbus(name = "getPollRate")]
    async fn get_poll_rate(&self) -> Result<i32>;

    #[zbus(name = "getRazerUrls")]
    async fn get_razer_urls(&self) -> Result<String>;

    #[zbus(name = "getSerial")]
    async fn get_serial(&self) -> Result<String>;

    #[zbus(name = "getSupportedPollRates")]
    async fn get_supported_poll_rates(&self) -> Result<Vec<u16>>;

    #[zbus(name = "getVidPid")]
    async fn get_vid_pid(&self) -> Result<Vec<i32>>;

    #[zbus(name = "hasDedicatedMacroKeys")]
    async fn has_dedicated_macro_keys(&self) -> Result<bool>;

    #[zbus(name = "hasMatrix")]
    async fn has_matrix(&self) -> Result<bool>;

    // Control methods

    #[zbus(name = "resumeDevice")]
    async fn resume_device(&self) -> zbus::Result<()>;

    #[zbus(name = "setDeviceMode")]
    async fn set_device_mode(&self, mode: u8, param: u8) -> zbus::Result<()>;

    #[zbus(name = "setHyperPollingLED")]
    async fn set_hyper_polling_led(&self, enabled: u8) -> zbus::Result<()>;

    #[zbus(name = "setPollRate")]
    async fn set_poll_rate(&self, rate: u16) -> zbus::Result<()>;

    #[zbus(name = "suspendDevice")]
    async fn suspend_device(&self) -> zbus::Result<()>;
}

#[proxy(
    interface = "razer.devices",
    default_service = "org.razer",
    default_path = "/org/razer"
)]
pub trait RazerDevices {
    #[zbus(name = "getDevices")]
    async fn get_devices(self) -> zbus::Result<Vec<String>>;
}
