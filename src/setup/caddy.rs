use std::process::{ExitStatus, Output};

use anyhow::{Error, Result};

use crate::core::CommandManager;

pub struct Caddy;

impl Caddy {
    pub fn setup() -> Result<()> {
        // install Caddy
        //
        //
        let cm = CommandManager::init();

        if !cm.is_installed("caddy")? {
            println!("Installing caddy using system package manager");
            Self::install_caddy()?;
        }

        println!("Enabling caddy systemd service ...");
        Self::enable_caddy_systemd_service()?;

        Ok(())
    }

    fn install_caddy() -> Result<ExitStatus, Error> {
        let cm = CommandManager::init();
        cm.install_package("caddy")
    }

    fn enable_caddy_systemd_service() -> Result<Output, Error> {
        let cm = CommandManager::init();
        cm.run("systemctl", &["enable", "--now", "caddy"])
    }
}
