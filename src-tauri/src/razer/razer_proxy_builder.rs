use zbus::{fdo::IntrospectableProxy, proxy, Connection, Result};

use crate::razer::{
    RazerDeviceBrightnessProxy, RazerDeviceDpiProxy, RazerDeviceGamemodeProxy,
    RazerDeviceMiscProxy, RazerDevicePowerProxy,
};

pub struct RazerProxyBuilder;

impl RazerProxyBuilder {
    async fn has_interface<P>(conn: &Connection, path: &str) -> Result<bool>
    where
        P: proxy::Defaults,
    {
        let Some(interface) = P::INTERFACE else {
            return Ok(false);
        };

        let proxy = IntrospectableProxy::builder(conn)
            .destination("org.razer")?
            .path(path)?
            .build()
            .await?;

        let xml = proxy.introspect().await?;
        let node = zbus_xml::Node::from_reader(xml.as_bytes())
            .map_err(|e| zbus::Error::Failure(e.to_string()))?;

        Ok(node
            .interfaces()
            .iter()
            .any(|i| i.name() == interface.to_owned()))
    }

    pub async fn misc<'a>(conn: &Connection, serial: &str) -> Result<RazerDeviceMiscProxy<'a>> {
        let path = format!("/org/razer/device/{}", serial);

        RazerDeviceMiscProxy::builder(conn)
            .path(path)?
            .build()
            .await
    }

    pub async fn dpi<'a>(
        conn: &Connection,
        serial: &str,
    ) -> Result<Option<RazerDeviceDpiProxy<'a>>> {
        let path = format!("/org/razer/device/{}", serial);

        if !Self::has_interface::<RazerDeviceDpiProxy>(conn, &path).await? {
            return Ok(None);
        }

        let proxy = RazerDeviceDpiProxy::builder(conn)
            .path(path)?
            .build()
            .await?;

        Ok(Some(proxy))
    }

    pub async fn brightness<'a>(
        conn: &Connection,
        serial: &str,
    ) -> Result<Option<RazerDeviceBrightnessProxy<'a>>> {
        let path = format!("/org/razer/device/{}", serial);

        if !Self::has_interface::<RazerDeviceBrightnessProxy>(conn, &path).await? {
            return Ok(None);
        }

        let proxy = RazerDeviceBrightnessProxy::builder(conn)
            .path(path)?
            .build()
            .await?;

        Ok(Some(proxy))
    }

    pub async fn power<'a>(
        conn: &Connection,
        serial: &str,
    ) -> Result<Option<RazerDevicePowerProxy<'a>>> {
        let path = format!("/org/razer/device/{}", serial);

        if !Self::has_interface::<RazerDevicePowerProxy>(conn, &path).await? {
            return Ok(None);
        }

        let proxy = RazerDevicePowerProxy::builder(conn)
            .path(path)?
            .build()
            .await?;

        Ok(Some(proxy))
    }

    pub async fn gamemode<'a>(
        conn: &Connection,
        serial: &str,
    ) -> Result<Option<RazerDeviceGamemodeProxy<'a>>> {
        let path = format!("/org/razer/device/{}", serial);

        if !Self::has_interface::<RazerDeviceGamemodeProxy>(conn, &path).await? {
            return Ok(None);
        }
        let proxy = RazerDeviceGamemodeProxy::builder(conn)
            .path(path)?
            .build()
            .await?;

        Ok(Some(proxy))
    }
}
