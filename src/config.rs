use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct SimulationConfig {
    pub cars: u32,
    pub rule_breakers: u32,
    pub base_point: bool,
    pub top_speed: u32,
    pub debug: bool
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub simulation: SimulationConfig,
}

pub fn load_config() -> Config {
    let toml_str = fs::read_to_string("config.toml")
        .expect("Failed to read config.toml");

    toml::from_str::<Config>(&toml_str)
        .expect("Failed to parse TOML")
}