use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::{Result, bail};

pub fn sudo_create_dir_all(dir: &str) -> Result<()> {
    let status = Command::new("sudo")
        .args(["mkdir", "-p", dir])
        .status()
        .map_err(|e| anyhow::anyhow!("Failed to spawn `sudo mkdir -p {}`: {}", dir, e))?;

    if !status.success() {
        bail!("Failed to create {} via sudo mkdir -p", dir);
    }

    Ok(())
}

pub fn sudo_write(path: &str, content: &str) -> Result<()> {
    let mut child = Command::new("sudo")
        .args(["tee", path])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to spawn `sudo tee {}`: {}", path, e))?;

    child
        .stdin
        .take()
        .ok_or_else(|| anyhow::anyhow!("Failed to open stdin for `sudo tee {}`", path))?
        .write_all(content.as_bytes())
        .map_err(|e| anyhow::anyhow!("Failed to write to `sudo tee {}` stdin: {}", path, e))?;

    let output = child
        .wait_with_output()
        .map_err(|e| anyhow::anyhow!("Failed waiting on `sudo tee {}`: {}", path, e))?;

    if !output.status.success() {
        bail!(
            "Failed to write {} via sudo tee: {}",
            path,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

pub fn sudo_chown(path: &str, uid: Option<u32>, gid: Option<u32>) -> Result<()> {
    let spec = match (uid, gid) {
        (Some(u), Some(g)) => format!("{}:{}", u, g),
        (Some(u), None) => format!("{}", u),
        (None, Some(g)) => format!(":{}", g),
        (None, None) => return Ok(()),
    };

    let status = Command::new("sudo")
        .args(["chown", &spec, path])
        .status()
        .map_err(|e| anyhow::anyhow!("Failed to spawn `sudo chown {}`: {}", path, e))?;

    if !status.success() {
        bail!("Failed to chown {} via sudo chown", path);
    }

    Ok(())
}
