use super::service::MiscService;

pub struct Feature {
    service: MiscService,
}

impl Feature {
    pub fn new(conn: zbus::Connection) -> Self {
        let service = MiscService::new(conn);
        Feature { service }
    }

    pub fn service(&self) -> &MiscService {
        &self.service
    }
}
