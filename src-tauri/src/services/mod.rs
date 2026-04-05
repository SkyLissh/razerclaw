mod razer;

pub use razer::RazerService;

pub struct Services {
    pub razer: RazerService,
}
