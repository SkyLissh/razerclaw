use super::service::Service;

pub struct Feature {
    service: Service,
}

impl Feature {
    pub fn new(conn: zbus::Connection) -> Self {
        Self {
            service: Service::new(conn),
        }
    }

    pub fn service(&self) -> &Service {
        &self.service
    }
}
