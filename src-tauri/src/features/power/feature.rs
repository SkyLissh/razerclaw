use super::service;

pub struct Feature {
    service: service::Service,
}

impl Feature {
    pub fn new(dbus_conn: zbus::Connection) -> Self {
        let service = service::Service::new(dbus_conn);
        Self { service }
    }

    pub fn service(&self) -> &service::Service {
        &self.service
    }
}
