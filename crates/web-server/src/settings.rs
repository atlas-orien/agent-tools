use serde::Deserialize;
use toolcraft_config::load_settings;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub http: HttpConfig,
}

#[derive(Debug, Deserialize)]
pub struct HttpConfig {
    pub port: u16,
}

impl Settings {
    pub fn load(config_path: &str) -> Result<Self, toolcraft_config::error::Error> {
        load_settings(config_path)
    }
}
