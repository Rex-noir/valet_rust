use std::{fs, path::Path};

use anyhow::Result;

use crate::core::{App, CommandManager};

#[derive(Debug)]
pub struct DnsMasq;

const SYSTEMD_RESOLVED_PATH: &str = "/etc/systemd/resolved.conf";

impl DnsMasq {
    pub fn setup(app: &App) -> Result<()> {
        let cm = CommandManager::init();

        println!("Setting up dnsmasq");

        if !cm.is_installed(&"dnsmasq")? {
            cm.install_package(&"dnsmasq")?;
        }

        // check if systemd resolved is in use
        let result = cm.run(
            &"systemctl",
            Some(&["is-active", "systemd-resolved", "--quiet"]),
        )?;

        if result.status.success() {
            println!("Uses systemd resolved, configuring it to prevent conflict with dnsmasq.");

            Self::configure_systemd_resolved_config()?;

            cm.run(&"systemctl", Some(&["restart", "systemd-resolved"]))?;
        }

        Ok(())
    }

    fn configure_systemd_resolved_config() -> Result<()> {
        let path = Path::new(SYSTEMD_RESOLVED_PATH);

        let mut content = if path.exists() {
            fs::read_to_string(path)?
        } else {
            String::new()
        };

        // Add DNSStubListener=no if not present
        if !content.contains("DNSStubListener=no") {
            if !content.ends_with('\n') {
                content.push('\n');
            }

            content.push_str("DNSStubListener=no\n");

            fs::write(path, content)?;

            println!("Added DNSStubListener=no to resolved.conf");
        } else {
            println!("DNSStubListener=no already configured");
        }

        Ok(())
    }
}
