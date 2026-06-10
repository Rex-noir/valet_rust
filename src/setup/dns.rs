use anyhow::{Ok, Result, bail};

use crate::core::{App, CommandManager};

#[derive(Debug)]
pub struct Dns;

impl Dns {
    pub fn setup(app: &App) -> Result<()> {
        let cm = CommandManager::init();

        // check systemd version

        let version = cm
            .run(&"systemctl", Some(&["--version"]))
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .and_then(|s| Self::parse_systemd_version(&s));

        if let Some(v) = version {
            println!("Systemd version detected : {}", v);
            if v < 258 {
                eprintln!("Not supported systemd version less than 258.")
            }

            Self::ensure_delegate_directory_exists()?;

            println!("Setting up dns delegate config ...");

            Self::setup_dns_delegate_config()?;

            println!("Restarting systemd-resolved");

            Self::restart_systemd_resolved()?;
        } else {
            println!("Systemd version can't be determined! Aborting!!")
        }

        Ok(())
    }

    fn parse_systemd_version(stdout: &str) -> Option<u32> {
        let first_line = stdout.lines().next()?;

        let version_str = first_line.split_whitespace().nth(1)?;

        version_str.parse::<u32>().ok()
    }

    fn ensure_delegate_directory_exists() -> Result<()> {
        let cm = CommandManager::init();

        let status = cm.run_elevated(&["mkdir", "-p", "/etc/systemd/dns-delegate.d"])?;

        if !status.success() {
            eprintln!("Failed to ensure ensure_delegate_directory_exists");
        }

        Ok(())
    }

    fn setup_dns_delegate_config() -> Result<()> {
        let cm = CommandManager::init();

        let path = "/etc/systemd/dns-delegate.d/valet-rust.dns-delegate";
        let content = r#"[Delegate]
DNS=127.0.0.1
Domains=~test"#;

        let cmd = format!("printf '%s' '{}' | sudo tee {} > /dev/null", content, path);
        let status = cm.run_elevated(&["sh", "-c", &cmd])?;

        if !status.success() {
            bail!("Failed to write DNS delegate config");
        }

        println!("DNS delegate config written to {}", path);

        Ok(())
    }

    fn restart_systemd_resolved() -> Result<()> {
        let cm = CommandManager::init();
        let status = cm.run_elevated(&["systemctl", "restart", "systemd-resolved"])?;
        if !status.success() {
            bail!("Error restarting systemd-resolved");
        }
        Ok(())
    }
}
