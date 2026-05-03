use anyhow::{Result, anyhow};
use std::process::Command;
use system_env::{CommandType, SystemPackageManager};

use crate::app::App;

const PACKAGES: &[&str] = &["dnsmasq", "nginx"];

pub fn run(_app: &App) -> Result<()> {
    let package_manager = SystemPackageManager::detect()?;

    let config = package_manager.get_config();

    let install_command = config
        .commands
        .get(&CommandType::InstallPackage)
        .ok_or_else(|| anyhow!("Install command not available"))?;

    let elevated_command = package_manager
        .get_elevated_command()
        .ok_or(anyhow!("Elevated command not found"))?;

    for package in PACKAGES {
        let package: &str = package;

        if !is_package_installed(package) {
            println!("{package} not found, installing...");

            let mut cmd = Command::new(elevated_command);

            cmd.arg(&install_command[0]);

            for arg in &install_command[1..] {
                let arg = if arg == "$" { package } else { arg };
                cmd.arg(arg);
            }

            let status = cmd.status()?;

            if !status.success() {
                return Err(anyhow!("Failed to install {package}"));
            }
        } else {
            println!("{package} already installed");
        }
    }

    Ok(())
}

fn is_package_installed(package: &str) -> bool {
    Command::new("which")
        .arg(package)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
