use std::{
    env, fs,
    os::unix::fs::chown,
    process::{Command, ExitStatus},
};

use anyhow::{Error, Ok, Result, bail};
use which::which;

use crate::core::{App, CommandManager};

pub struct Caddy;

impl Caddy {
    pub fn setup() -> Result<()> {
        println!("Installing caddy using system package manager");
        Self::install_caddy()?;

        println!("Setup caddy configuration");
        Self::setup_caddy_configuration()?;

        println!("Disable default caddy systemd service");
        Self::disable_caddy_system_service()?;

        println!("Create valet rust caddy systemd service");
        Self::create_valet_rust_caddy_service()?;

        println!("Enable valet rust caddy systemd service");
        Self::enable_valet_rust_systemd_service()?;

        Ok(())
    }

    fn install_caddy() -> Result<ExitStatus, Error> {
        let cm = CommandManager::init();
        cm.install_package("caddy")
    }

    fn setup_caddy_configuration() -> Result<()> {
        let stub_config = include_str!("../stubs/Caddyfile");
        let app = App::init();

        let caddy_files_dir = app.config_path.join("caddy_files");
        fs::create_dir_all(&caddy_files_dir)?;

        let import_path = caddy_files_dir.join("*");

        let config = stub_config.replace(
            "{{CADDY_CONFIGS_DIR}}",
            import_path
                .to_str()
                .expect("Application config path is not valid UTF-8"),
        );

        let main_caddy_path = app.config_path.join("Caddyfile");

        fs::write(&main_caddy_path, config)?;

        if env::var_os("SUDO_USER").is_some() {
            chown(&caddy_files_dir, Some(app.uid), Some(app.gid))
                .expect("failed to chown caddy config dirs");

            chown(&main_caddy_path, Some(app.uid), Some(app.gid))
                .expect("failed to chown main caddy file")
        }

        Ok(())
    }

    fn disable_caddy_system_service() -> Result<()> {
        let status = Command::new("systemctl")
            .args(["disable", "--now", "caddy"])
            .status()?;

        if !status.success() {
            anyhow::bail!("failed to disable caddy system service");
        }

        Ok(())
    }

    fn create_valet_rust_caddy_service() -> Result<()> {
        let app = App::init();
        let caddy_bin = which("caddy")?;
        let service_config = include_str!("../stubs/caddy.service")
            .replace(
                "{{CADDY_BIN}}",
                caddy_bin.to_str().expect("invalid caddy path"),
            )
            .replace(
                "{{CONFIG_DIR}}",
                app.config_path.to_str().expect("invalid config path"),
            );

        let user_systemd_dir = app.home_dir.join(".config/systemd/user");
        fs::create_dir_all(&user_systemd_dir)?;

        let service_path = user_systemd_dir.join("valet-rust-caddy.service");
        fs::write(&service_path, service_config)?;

        if env::var_os("SUDO_USER").is_some() {
            chown(&user_systemd_dir, Some(app.uid), Some(app.gid))?;
            chown(&service_path, Some(app.uid), Some(app.gid))?;
        }

        Ok(())
    }

    fn enable_valet_rust_systemd_service() -> Result<()> {
        let status = Command::new("systemctl")
            .args(["enable", "--user", "--now", "valet-rust-caddy.service"])
            .status()?;

        if !status.success() {
            bail!("Failed to enable valet rust caddy systemd service");
        }

        Ok(())
    }
}
