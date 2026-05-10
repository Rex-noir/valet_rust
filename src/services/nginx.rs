use std::{
    io::Write,
    path::Path,
    process::Stdio,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::Result;

use crate::{App, NGINX_CONFIG_PATH, NGINX_CONFIG_STUB, core::CommandManager};

#[derive(Debug)]
pub struct Nginx;

impl Nginx {
    pub fn setup(app: &App) -> Result<()> {
        let command_manager = CommandManager::init();

        if let Ok(value) = command_manager.is_installed(&"nginx")
            && !value
        {
            Self::install()?;
        }

        if Path::new(NGINX_CONFIG_PATH).exists() {
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

            let backup_path = format!("{}.back.{}", NGINX_CONFIG_PATH, timestamp);

            let status = command_manager
                .get_elevated_command_builder(&["cp", NGINX_CONFIG_PATH, &backup_path])
                .status()?;

            if !status.success() {
                return Err(anyhow::anyhow!("Failed to backup nginx config"));
            }
            println!("Backed up nginx config to {}", backup_path);
        }

        let nginx_service_configuration =
            command_manager.run(&"cat", Some(&["/lib/systemd/system/nginx.service"]))?;

        let output_str = String::from_utf8_lossy(&nginx_service_configuration.stdout);

        let mut pid_path = "pid /run/nginx.pid";

        if output_str.contains("PIDFile=") {
            pid_path = "# pid /run/nginx.pid";
        }

        let config = NGINX_CONFIG_STUB
            .replace("VALET_USER", &command_manager.user())
            .replace("VALET_GROUP", &command_manager.group())
            .replace("VALET_PID", pid_path)
            .replace(
                "VALET_HOME_PATH",
                &app.app_dir().canonicalize()?.to_string_lossy(),
            );

        let mut child = command_manager
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

        println!("Nginx config updated sucessfully.");

        Ok(())
    }

    pub fn install() -> Result<()> {
        let command_manager = CommandManager::init();

        command_manager.install_package(&"nginx")?;

        Ok(())
    }
}
