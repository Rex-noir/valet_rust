use std::{
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::{Context, Ok, Result, bail};

use crate::core::{App, CommandManager};

pub struct Nginx;

impl Nginx {
    pub(crate) fn setup() -> Result<()> {
        println!("Setting nginx");

        let cm = CommandManager::init();
        cm.install_package("nginx")?;

        Self::write_nginx_config()?;
        Self::restart_nginx()?;

        Ok(())
    }

    fn load_nginx_config() -> String {
        let app = App::init();

        let nginx_path = app.nginx_files_path.join("*.conf").display().to_string();

        include_str!("../stubs/nginx.conf")
            .replace("{{VALEX_USER}}", &app.username)
            .replace("{{VALEX_NGINX_CONFIGS_PATH}}", &nginx_path)
    }

    fn write_nginx_config() -> Result<()> {
        let config = Self::load_nginx_config();

        let mut child = Command::new("sudo")
            .args(["tee", "/etc/nginx/nginx.conf"])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()?;

        child.stdin.as_mut().unwrap().write_all(config.as_bytes())?;

        let status = child.wait()?;
        anyhow::ensure!(status.success(), "failed to write nginx.conf");

        Self::configure_selinux()?;

        Ok(())
    }

    fn restart_nginx() -> Result<()> {
        let status = Command::new("sudo")
            .arg("systemctl")
            .arg("restart")
            .arg("nginx")
            .status()?;

        if !status.success() {
            bail!("Nginx service restart failed");
        }

        Ok(())
    }

    fn configure_selinux() -> Result<()> {
        let selinux_exists = Path::new("/sys/fs/selinux").exists();

        if selinux_exists {
            let status = Command::new("sudo")
                .args(["setsebool", "-P", "httpd_read_user_content", "1"])
                .status()
                .context("failed to execute setsebool")?;

            anyhow::ensure!(status.success(), "failed to enable httpd_read_user_content");
        }

        Ok(())
    }
}
