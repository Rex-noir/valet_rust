use anyhow::Result;
use std::process::Command;

use crate::{app::App, core::command_runner::CommandRunner};

const PACKAGES: &[&str] = &["dnsmasq", "nginx"];

pub fn run(_app: &App) -> Result<()> {
    let command_runner = CommandRunner::init();
    for package in PACKAGES {
        if !is_package_installed(package) {
            command_runner.install_package(package)?;
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
