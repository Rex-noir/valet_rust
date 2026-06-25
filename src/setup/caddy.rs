use crate::core::{App, CommandManager};
use anyhow::{Error, Ok, Result, anyhow, bail};
use std::{
    path::PathBuf,
    process::{Command, ExitStatus},
};

pub struct Caddy;

impl Caddy {
    pub fn setup() -> Result<()> {
        println!("Installing caddy using system package manager");
        Self::install_caddy()?;
        println!("Setup caddy configuration");
        Self::setup_caddy_configuration()?;
        println!("reload default caddy systemd service");
        Self::reload_caddy_system_service()?;
        Ok(())
    }

    fn install_caddy() -> Result<ExitStatus, Error> {
        let cm = CommandManager::init();
        cm.install_package("caddy")
    }

    fn reload_caddy_system_service() -> Result<()> {
        let status = Command::new("sudo")
            .arg("systemctl")
            .args(["restart", "--now", "caddy"])
            .status()?;
        if !status.success() {
            bail!("failed to disable caddy system service");
        }
        Ok(())
    }

    fn setup_caddy_configuration() -> Result<()> {
        let caddy_configuration_path = PathBuf::from("/etc/caddy/Caddyfile");
        assert!(caddy_configuration_path.exists());

        let app = App::init();

        Command::new("mkdir")
            .arg("-p")
            .arg(&app.caddy_files_path)
            .status()?
            .success()
            .then_some(())
            .ok_or_else(|| anyhow!("failed to create caddy files directory"))?;

        let import_line = format!(
            "import {}",
            app.caddy_files_path.join("*.caddyfile").display()
        );

        let status = Command::new("grep")
            .arg("-qxF")
            .arg(&import_line)
            .arg("/etc/caddy/Caddyfile")
            .status()?;

        if !status.success() {
            Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "printf '%s\n' '{}' | sudo tee -a /etc/caddy/Caddyfile >/dev/null",
                    import_line
                ))
                .status()?
                .success()
                .then_some(())
                .ok_or_else(|| anyhow::anyhow!("failed to append import line"))?;
        }

        Ok(())
    }
}
