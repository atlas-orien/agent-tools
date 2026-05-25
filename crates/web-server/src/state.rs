use crate::settings::Settings;

#[derive(Debug, Clone, Default)]
pub struct AppState;

impl AppState {
    pub fn new(_settings: &Settings) -> Self {
        Self
    }
}
