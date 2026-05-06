use std::{
    process::{Command, ExitStatus},
    sync::OnceLock,
};

use anyhow::{Ok, Result, anyhow};
use system_env::SystemPackageManager;

pub struct CommandRunner {
    package_manager: SystemPackageManager,
}

#[allow(dead_code)]
trait StrArg: AsRef<str> {}
impl<T: AsRef<str>> StrArg for T {}

static INSTANCE: OnceLock<CommandRunner> = OnceLock::new();

impl CommandRunner {
    pub fn init() -> &'static Self {
        INSTANCE.get_or_init(|| {
            let package_manager =
                SystemPackageManager::detect().expect("Failed to detect package manager");
            CommandRunner { package_manager }
        })
    }

    fn run_elevated<S: StrArg>(&self, args: &[S]) -> Result<ExitStatus> {
        let elevated_command = self
            .package_manager
            .get_elevated_command()
            .ok_or(anyhow!("Elevated command not available"))?;

        let mut install_command = self
            .package_manager
            .get_config()
            .commands
            .get(&system_env::CommandType::InstallPackage)
            .ok_or(anyhow!("Install command not available"))?
            .clone();

        if let Some(pos) = install_command.iter().position(|x| x == "$") {
            install_command.splice(pos..=pos, args.iter().map(|a| a.as_ref().to_string()));
        }

        let status = Command::new(elevated_command)
            .args(install_command)
            .status()?;

        Ok(status)
    }

    pub fn install_package() {}
}
