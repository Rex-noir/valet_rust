use anyhow::{Ok, Result, anyhow};
use std::{
    process::{Command, ExitStatus},
    sync::OnceLock,
};
use system_env::SystemPackageManager;

pub struct CommandRunner {
    package_manager: SystemPackageManager,
}

#[allow(dead_code)]
pub trait CommandArgs: AsRef<str> {}
impl<T: AsRef<str>> CommandArgs for T {}

static INSTANCE: OnceLock<CommandRunner> = OnceLock::new();

impl CommandRunner {
    pub fn init() -> &'static Self {
        INSTANCE.get_or_init(|| {
            let package_manager =
                SystemPackageManager::detect().expect("Failed to detect package manager");
            CommandRunner { package_manager }
        })
    }

    fn run_elevated<S: CommandArgs>(&self, args: &[S]) -> Result<ExitStatus> {
        let elevated_command = self
            .package_manager
            .get_elevated_command()
            .ok_or(anyhow!("Elevated command not available"))?;

        let status = Command::new(elevated_command)
            .args(args.iter().map(|a| a.as_ref()))
            .status()?;

        Ok(status)
    }
    pub fn install_package<S: CommandArgs>(&self, package: &S) -> Result<ExitStatus> {
        let mut cmd = self
            .package_manager
            .get_config()
            .commands
            .get(&system_env::CommandType::InstallPackage)
            .ok_or(anyhow!("Install command not available"))?
            .clone();

        if let Some(pos) = cmd.iter().position(|x| x == "$") {
            cmd.splice(pos..=pos, std::iter::once(package.as_ref().to_string()));
        }

        let cmd_refs: Vec<&str> = cmd.iter().map(|s| s.as_str()).collect();
        self.run_elevated(&cmd_refs)
    }
}
