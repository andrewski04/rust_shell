use serde_derive::Deserialize;
use std::fmt;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub version: String,
    pub name: String,
    pub style: String,
}

impl Config {
    // returns instance of config as set in config.toml
    pub fn build(config_path: &str) -> Result<Config, &'static str> {
        // read config content, throw err on fail
        let config_content = match fs::read_to_string(config_path) {
            Ok(s) => s,
            Err(_) => return Err("Failed to read config. Check the config path."),
        };

        // parse config, throw error on fail
        let config: Config = match toml::from_str(&config_content) {
            Ok(c) => c,
            Err(_) => return Err("Failed to parse config."),
        };

        Ok(config)
    }
}

// Implementing Display for fancy printing and stuff
impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name: {}\nVersion: {}", self.name, self.version)
    }
}
