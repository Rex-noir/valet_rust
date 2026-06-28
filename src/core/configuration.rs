use std::{collections::HashMap, fs, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PhpInstallation {
    pub fpm_config_path: String,
    pub fpm_socket_path: String,
    pub fpm_binary_path: Option<String>,
    pub cli_binary_path: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Configuration {
    pub php: HashMap<String, PhpInstallation>,
}

impl Configuration {
    pub fn load_or_default(path: &Path) -> Result<Self> {
        if path.exists() {
            let text = fs::read_to_string(path)?;
            Ok(serde_json5::from_str(&text)?)
        } else {
            let config = Self::default();
            config.save(path)?;
            Ok(config)
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let text = serde_json5::to_string(self)?;
        fs::write(path, text)?;
        Ok(())
    }
}
