
/**
 * Defines configuration structures and functions to load configurations from files.
 * This file ensures that all parts of the application can access configuration settings in a structured manner.
*/
use serde::Deserialize;

use anyhow::Result;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub logging: LoggingConfig,
    pub flow_event: Event,
    pub daemon_event: Event,
    pub packet_event: Event,
    pub error_event: Event,
}

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub encoding: String,
    pub format: String,
    pub datefmt: String,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    pub ignore_fields: Vec<String>,
    pub ignore_risks: Vec<String>,
    pub flow_event_name: Vec<String>,
    pub geoip: Option<GeoIP>,
    pub filename: String,
}

#[derive(Debug, Deserialize)]
pub struct GeoIP {
    pub enabled: bool,
    pub filepath: String,
    pub keys: Vec<String>,  
}

impl Config {
    pub fn from_file(path: &std::path::Path) -> Result<Self> {
        let config_str = fs::read_to_string(path)?;
        let config = toml::from_str(&config_str)?;
        Ok(config)
    }

    pub fn new(
        logging: LoggingConfig,
        flow_event: Event,
        daemon_event: Event,
        packet_event: Event,
        error_event: Event,
    ) -> Self {
        Config {
            logging,
            flow_event,
            daemon_event,
            packet_event,
            error_event,
        }
    }
}
