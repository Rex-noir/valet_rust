use anyhow::Result;

use crate::{App, core::CommandManager};

#[derive(Debug)]
pub struct DnsMasq;

impl DnsMasq {
    pub fn setup(app: &App) -> Result<()> {
        let cm = CommandManager::init();

        println!("Setting up dnsmasq");

        if !cm.is_installed(&"dnsmasq")? {
            cm.install_package(&"dnsmasq")?;
        }

        Ok(())
    }
}
