use anyhow::Result;
use directories::ProjectDirs;
use std::{fs, path::PathBuf};

use crate::core::Config;

pub struct App {
    app_dir: PathBuf,
    config: Config,
}

impl App {
    pub fn new() -> Result<Self> {
        let dirs = ProjectDirs::from("dev", "acerex", "valet_rust")
            .expect("Unable to determine project directory.");

        let app_dir = dirs.data_dir().to_path_buf();

        fs::create_dir_all(&app_dir)?;

        let config = Config::load_or_default(&app_dir)?;

        Ok(Self { app_dir, config })
    }

    pub fn app_dir(&self) -> &PathBuf {
        &self.app_dir
    }

    pub fn config_path(&self) -> PathBuf {
        self.app_dir.join("config.json")
    }

    pub fn socket_path(&self) -> PathBuf {
        self.app_dir.join("valet.sock")
    }
}
