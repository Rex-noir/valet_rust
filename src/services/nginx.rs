use crate::{App, NGINX_CONFIG_PATH, NGINX_CONFIG_STUB, core::CommandManager};
use anyhow::Result;
use std::{
    io::Write,
    path::Path,
    process::Stdio,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug)]
pub struct Nginx;

impl Nginx {
    pub fn setup(app: &App) -> Result<()> {
        let cm = CommandManager::init();

        if let Ok(false) = cm.is_installed(&"nginx") {
            Self::install_with(cm)?;
        }

        if Path::new(NGINX_CONFIG_PATH).exists() {
            Self::backup_config(cm)?;
        }

        let config = Self::build_config(cm, app)?;
        Self::write_config(cm, &config)?;

        println!("Nginx config updated successfully.");
        Ok(())
    }

    pub fn install() -> Result<()> {
        Self::install_with(CommandManager::init())
    }

    fn install_with(cm: &CommandManager) -> Result<()> {
        cm.install_package(&"nginx")?;
        Ok(())
    }

    fn backup_config(cm: &CommandManager) -> Result<()> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let backup_path = format!("{}.back.{}", NGINX_CONFIG_PATH, timestamp);

        let status = cm
            .get_elevated_command_builder(&["cp", NGINX_CONFIG_PATH, &backup_path])
            .status()?;

        if !status.success() {
            return Err(anyhow::anyhow!("Failed to backup nginx config"));
        }

        println!("Backed up nginx config to {}", backup_path);
        Ok(())
    }

    fn build_config(cm: &CommandManager, app: &App) -> Result<String> {
        let pid_path = Self::resolve_pid_path(cm);

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

    fn resolve_pid_path(cm: &CommandManager) -> &'static str {
        let output = cm.run(&"cat", Some(&["/lib/systemd/system/nginx.service"]));

        match output {
            Ok(out) if String::from_utf8_lossy(&out.stdout).contains("PIDFile=") => {
                "# pid /run/nginx.pid"
            }
            _ => "pid /run/nginx.pid",
        }
    }

    fn write_config(cm: &CommandManager, config: &str) -> Result<()> {
        let mut child = cm
            .get_elevated_command_builder(&["tee", NGINX_CONFIG_PATH])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()
            .map_err(|e| anyhow::anyhow!("Failed to spawn elevated tee: {}", e))?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(config.as_bytes())?;
        }

        let status = child.wait()?;
        if !status.success() {
            return Err(anyhow::anyhow!("Failed to write config via sudo tee"));
        }

        Ok(())
    }
}
