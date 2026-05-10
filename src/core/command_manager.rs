use anyhow::{Result, anyhow};
use std::{
    process::{Command, ExitStatus, Output},
    sync::OnceLock,
};
use system_env::SystemPackageManager;

pub struct CommandManager {
    package_manager: SystemPackageManager,
}

#[allow(dead_code)]
pub trait CommandArgs: AsRef<str> {}
impl<T: AsRef<str>> CommandArgs for T {}

static INSTANCE: OnceLock<CommandManager> = OnceLock::new();

impl CommandManager {
    pub fn init() -> &'static Self {
        INSTANCE.get_or_init(|| {
            let package_manager =
                SystemPackageManager::detect().expect("Failed to detect package manager");
            CommandManager { package_manager }
        })
    }

    pub fn get_elevated_command_builder<S: AsRef<str>>(&self, args: &[S]) -> Command {
        let elevated_program = self
            .package_manager
            .get_elevated_command()
            .expect("Elevated command not available");

        let mut cmd = Command::new(elevated_program);

        cmd.args(args.iter().map(|a| a.as_ref()));

        cmd
    }

    pub fn run_elevated<S: CommandArgs>(&self, args: &[S]) -> Result<ExitStatus> {
        let mut elevated_command = self.get_elevated_command_builder(args);
        let status = elevated_command.status()?;
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

    pub fn is_installed<S: CommandArgs>(&self, package: &S) -> Result<bool> {
        let cmd = self
            .package_manager
            .get_config()
            .commands
            .get(&system_env::CommandType::ListPackages)
            .ok_or(anyhow!("List command not availabel"))?
            .clone();

        let output = Command::new(&cmd[0]).args(&cmd[1..]).output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        let installed_list = self.package_manager.get_config().list_parser.parse(&stdout);

        Ok(installed_list.contains_key(package.as_ref()))
    }

    pub fn run<S: CommandArgs>(&self, command: &S, args: Option<&[S]>) -> Result<Output> {
        let mut cmd = Command::new(command.as_ref());
        if let Some(args) = args {
            cmd.args(args.iter().map(|a| a.as_ref()));
        }
        let output = cmd.output()?;
        Ok(output)
    }

    pub fn user(&self) -> String {
        let output = Command::new("whoami")
            .output()
            .expect("Failed to run whoami command");

        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    pub fn group(&self) -> String {
        let output = Command::new("id")
            .arg("-gn")
            .output()
            .expect("Failed to get user group");

        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }
}
