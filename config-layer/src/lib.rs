use serde::Deserialize;
use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse config: {0}")]
    ParseError(#[from] toml::de::Error),
}

#[derive(Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub gain: f64,
    pub resistance: f64,
    pub initial_agents_count: usize,
    pub initial_energy: f64,
    pub max_ticks: u64,
    pub mutation_threshold: f64,
    pub min_age_for_mutation: u32,
    pub mutation_reset_energy: f64,
    pub interference_factor: f64,
    pub state_file: String,
}

impl AppConfig {
    pub fn load(path: &str) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }
}
