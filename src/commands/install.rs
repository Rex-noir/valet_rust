use anyhow::Result;
use std::process::Command;

use crate::{app::App, core::CommandRunner};

pub fn run(_app: &App) -> Result<()> {
    let command_runner = CommandRunner::init();

    if !is_package_installed("nginx") {
        command_runner.install_package(&"nginx")?;
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
