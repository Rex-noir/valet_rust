use anyhow::{Result, anyhow};
use std::{
    process::{Command, ExitStatus, Output},
    sync::OnceLock,
};
use system_env::SystemPackageManager;

pub struct CommandManager {
    package_manager: SystemPackageManager,
}

static INSTANCE: OnceLock<CommandManager> = OnceLock::new();

// The crate I used couldn't detect cachyos
fn detect_package_manager() -> SystemPackageManager {
    SystemPackageManager::detect().unwrap_or(SystemPackageManager::Pacman)
}

impl CommandManager {
    pub fn init() -> &'static Self {
        INSTANCE.get_or_init(|| CommandManager {
            package_manager: detect_package_manager(),
        })
    }

    pub fn install_package(&self, package: &str) -> Result<ExitStatus> {
        let mut cmd = self
            .package_manager
            .get_config()
            .commands
            .get(&system_env::CommandType::InstallPackage)
            .ok_or(anyhow!("Install command not available"))?
            .clone();

        if let Some(pos) = cmd.iter().position(|x| x == "$") {
            cmd.splice(pos..=pos, std::iter::once(package.to_string()));
        }

        let elevated_program = self
            .package_manager
            .get_elevated_command()
            .expect("Elevated command not available");

        let status = Command::new(elevated_program)
            .args(cmd.iter().map(|s| s.as_str()))
            .arg("-y")
            .status()?;

        Ok(status)
    }

    pub fn is_installed(&self, package: &str) -> Result<bool> {
        let mut cmd = self
            .package_manager
            .get_config()
            .commands
            .get(&system_env::CommandType::ListPackages)
            .ok_or(anyhow!("List command not available"))?
            .clone();

        cmd.push(package.to_string());

        let output = Command::new(&cmd[0]).args(&cmd[1..]).output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        let installed_list = self.package_manager.get_config().list_parser.parse(&stdout);

        Ok(installed_list.contains_key(package))
    }

    pub fn run(&self, command: &str, args: &[&str]) -> Result<Output> {
        let output = Command::new(command).args(args).output()?;
        Ok(output)
    }

    /// Returns the current user's name using the $USER env var,
    /// falling back to parsing /etc/passwd by UID from /proc/self/status.
    pub fn user(&self) -> String {
        std::env::var("USER").unwrap_or_else(|_| {
            let uid = read_id_from_proc("Uid").unwrap_or(65534);
            resolve_name_from_file("/etc/passwd", uid).unwrap_or_else(|| "nobody".to_string())
        })
    }

    /// Returns the current user's primary group name by reading
    /// /proc/self/status for the GID and resolving it via /etc/group.
    pub fn group(&self) -> String {
        let gid = read_id_from_proc("Gid").unwrap_or(65534);
        resolve_name_from_file("/etc/group", gid).unwrap_or_else(|| "nogroup".to_string())
    }
}

/// Read a UID or GID from /proc/self/status.
/// The file contains lines like "Uid:\t1000\t1000\t1000\t1000" — we take the
/// real (first) value.
fn read_id_from_proc(key: &str) -> Option<u32> {
    let content = std::fs::read_to_string("/proc/self/status").ok()?;
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix(key) {
            let rest = rest.strip_prefix(':')?;
            let first_field = rest.split_whitespace().next()?;
            return first_field.parse::<u32>().ok();
        }
    }
    None
}

/// Resolve a name from a colon-delimited passwd/group file.
/// Format: name:x:id:...
fn resolve_name_from_file(path: &str, target_id: u32) -> Option<String> {
    let content = std::fs::read_to_string(path).ok()?;
    for line in content.lines() {
        let fields: Vec<&str> = line.split(':').collect();
        if fields.len() >= 3 {
            if let Ok(id) = fields[2].parse::<u32>() {
                if id == target_id {
                    return Some(fields[0].to_string());
                }
            }
        }
    }
    None
}
