use crate::core::App;
use crate::{NGINX_CONFIG_PATH, NGINX_CONFIG_STUB, core::CommandManager};
use anyhow::{Result, bail};
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Nginx;

impl Nginx {
    pub fn setup(app: &App) -> Result<()> {
        let cm = CommandManager::init();

        if !cm.is_installed("nginx")? {
            Self::install_with(cm)?;
        }

        if Path::new(NGINX_CONFIG_PATH).exists() {
            Self::backup_config()?;
        }

        let config = Self::build_config(cm, app)?;
        Self::write_config(&config)?;

        println!("Nginx config updated successfully.");

        Self::restart_nginx(cm)?;

        Ok(())
    }

    pub fn install() -> Result<()> {
        Self::install_with(CommandManager::init())
    }

    fn install_with(cm: &CommandManager) -> Result<()> {
        cm.install_package("nginx")?;
        Ok(())
    }

    fn backup_config() -> Result<()> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let backup_path = format!("{}.back.{}", NGINX_CONFIG_PATH, timestamp);

        fs::copy(NGINX_CONFIG_PATH, &backup_path)
            .map_err(|e| anyhow::anyhow!("Failed to backup nginx config: {}", e))?;

        println!("Backed up nginx config to {}", backup_path);
        Ok(())
    }

    fn build_config(cm: &CommandManager, app: &App) -> Result<String> {
        let pid_path = Self::resolve_pid_path();

        let config = NGINX_CONFIG_STUB
            .replace("VALET_USER", &cm.user())
            .replace("VALET_GROUP", &cm.group())
            .replace("VALET_PID", pid_path)
            .replace(
                "VALET_HOME_PATH",
                &app.app_dir().canonicalize()?.to_string_lossy(),
            );

        Ok(config)
    }

    fn resolve_pid_path() -> &'static str {
        let content = fs::read_to_string("/lib/systemd/system/nginx.service");

        match content {
            Ok(s) if s.contains("PIDFile=") => "# pid /run/nginx.pid",
            _ => "pid /run/nginx.pid",
        }
    }

    fn write_config(config: &str) -> Result<()> {
        fs::write(NGINX_CONFIG_PATH, config)
            .map_err(|e| anyhow::anyhow!("Failed to write nginx config to {}: {}", NGINX_CONFIG_PATH, e))?;
        Ok(())
    }

    fn restart_nginx(cm: &CommandManager) -> Result<()> {
        let output = cm.run("nginx", &["-t", "-q"])?;
        if !output.status.success() {
            bail!("Nginx config test failed");
        }

        let active = cm.run("systemctl", &["is-active", "--quiet", "nginx"])?;

        if active.status.success() {
            println!("Reloading nginx...");

            let reload = cm.run("systemctl", &["reload", "nginx"])?;

            if !reload.status.success() {
                bail!("Nginx reload failed");
            }
        } else {
            print!("Nginx is not running. Start it now? [y/N]: ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let input = input.trim().to_lowercase();

            if input == "y" || input == "yes" {
                let start = cm.run("systemctl", &["start", "nginx"])?;

                if !start.status.success() {
                    bail!("Failed to start nginx");
                }

                println!("Nginx started successfully.");
            } else {
                println!("Skipping nginx start.");
            }
        }

        Ok(())
    }
}
