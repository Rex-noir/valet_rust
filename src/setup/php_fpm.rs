use std::{io::Write, process::Command};

use anyhow::{Context, Ok, Result, bail};

use crate::core::App;

pub struct PHPFpm;

impl PHPFpm {
    pub(crate) fn setup() -> Result<()> {
        let app = App::init()?;

        let fpm_config = include_str!("../stubs/valex-fpm.conf")
            .replace("{{VALEX_USER}}", &app.username)
            .replace("{{VALEX_USERGROUP}}", &app.groupname)
            .replace("{{VALEX_FPM_SOCKET_OWNER}}", &app.username)
            .replace("{{VALEX_FPM_SOCKET_GROUP}}", &app.groupname);

        for installation in app.config.php.values() {
            let config =
                fpm_config.replace("{{VALEX_FPM_SOCKET_PATH}}", &installation.fpm_socket_path);

            Command::new("sudo")
                .args(["tee", &installation.fpm_config_path])
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::null())
                .spawn()
                .context("failed to start tee")?
                .stdin
                .as_mut()
                .context("failed to open stdin")?
                .write_all(config.as_bytes())?;
        }

        // Restart all fpm services
        let status = Command::new("sudo")
            .arg("systemctl")
            .args(["restart", "php*-fpm.service"])
            .status()?;

        if !status.success() {
            bail!("Failed to restart fpm services");
        }

        Ok(())
    }
}
