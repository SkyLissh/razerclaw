use super::model::{DeviceCapability, DeviceSummary};
use super::proxy::RazerDeviceMiscProxy;

pub async fn summary_from_proxy<'a>(
    conn: &zbus::Connection,
    path: &str,
) -> zbus::Result<DeviceSummary> {
    let misc = RazerDeviceMiscProxy::new(conn, path).await?;

    let (
        name,
        serial,
        device_type,
        image,
        driver_version,
        firmware,
        mode,
        vid_pid,
        razer_urls,
        has_matrix,
        has_dedicated_macro_keys,
        mut capabilities,
    ) = futures::try_join!(
        misc.get_device_name(),
        misc.get_serial(),
        misc.get_device_type(),
        misc.get_device_image(),
        misc.get_driver_version(),
        misc.get_firmware(),
        misc.get_device_mode(),
        misc.get_vid_pid(),
        misc.get_razer_urls(),
        misc.has_matrix(),
        misc.has_dedicated_macro_keys(),
        capabilities_from_interfaces(conn, path),
    )?;

    let (matrix_dimensions, poll_rate, supported_poll_rates) = futures::join!(
        misc.get_matrix_dimensions(),
        misc.get_poll_rate(),
        misc.get_supported_poll_rates(),
    );

    let [vid, pid] = vid_pid.as_slice() else {
        return Err(zbus::Error::Failure("Invalid VID/PID format".into()));
    };

    if poll_rate.is_ok() {
        capabilities.push(DeviceCapability::PollRate);
    }

    Ok(DeviceSummary {
        name,
        serial,
        device_type: device_type.into(),
        image,
        driver_version,
        firmware,
        mode,
        matrix_dimensions: matrix_dimensions.ok(),
        poll_rate: poll_rate.ok(),
        supported_poll_rates: supported_poll_rates.ok(),
        has_dedicated_macro_keys: has_dedicated_macro_keys,
        has_matrix: has_matrix,
        razer_urls: razer_urls,
        vid: vid.to_owned(),
        pid: pid.to_owned(),
        capabilities,
    })
}

fn capability_from_interface(name: &str) -> Option<DeviceCapability> {
    match name {
        "razer.device.dpi" => Some(DeviceCapability::Dpi),
        "razer.device.lighting.brightness" => Some(DeviceCapability::Brightness),
        "razer.device.power" => Some(DeviceCapability::Power),
        "razer.device.led.gamemode" => Some(DeviceCapability::Gamemode),
        _ => None,
    }
}

async fn capabilities_from_interfaces(
    conn: &zbus::Connection,
    path: &str,
) -> zbus::Result<Vec<DeviceCapability>> {
    let proxy = zbus::fdo::IntrospectableProxy::builder(conn)
        .destination("org.razer")?
        .path(path)?
        .build()
        .await?;

    let xml = proxy.introspect().await?;
    let node = zbus_xml::Node::from_reader(xml.as_bytes())
        .map_err(|e| zbus::Error::Failure(e.to_string()))?;

    let capabilities = node
        .interfaces()
        .iter()
        .filter_map(|interface| capability_from_interface(interface.name().as_str()))
        .collect();

    Ok(capabilities)
}
