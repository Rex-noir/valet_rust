use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub default_php: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_php: "8.5".to_string(),
        }
    }
}

impl Config {
    pub fn load_or_default<P: AsRef<Path>>(app_dir: P) -> Result<Self> {
        let app_dir = app_dir.as_ref();
        let config_path = app_dir.join("config.json");

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config = serde_json::from_str(&content).unwrap_or_else(|_| Config::default());
            return Ok(config);
        }

        let default = Config::default();
        let content = serde_json::to_string_pretty(&default)?;
        fs::write(&config_path, content)?;

        Ok(default)
    }
}
