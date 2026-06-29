use std::os::unix::fs::chown;
use std::path::Path;

use anyhow::Result;

pub trait FsProvider {
    fn create_dir_all(&self, path: &Path) -> Result<()>;
    fn write(&self, path: &Path, contents: &str) -> Result<()>;
    fn read_to_string(&self, path: &Path) -> Result<String>;
    fn exists(&self, path: &Path) -> bool;
    fn chown(&self, path: &Path, uid: Option<u32>, gid: Option<u32>) -> Result<()>;
}

pub struct StdFs;

impl FsProvider for StdFs {
    fn create_dir_all(&self, path: &Path) -> Result<()> {
        Ok(std::fs::create_dir_all(path)?)
    }

    fn write(&self, path: &Path, contents: &str) -> Result<()> {
        Ok(std::fs::write(path, contents)?)
    }

    fn read_to_string(&self, path: &Path) -> Result<String> {
        Ok(std::fs::read_to_string(path)?)
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn chown(&self, path: &Path, uid: Option<u32>, gid: Option<u32>) -> Result<()> {
        Ok(chown(path, uid, gid)?)
    }
}

pub struct SudoFs;

impl FsProvider for SudoFs {
    fn create_dir_all(&self, path: &Path) -> Result<()> {
        crate::util::sudo_create_dir_all(&path.to_string_lossy())
    }

    fn write(&self, path: &Path, contents: &str) -> Result<()> {
        crate::util::sudo_write(&path.to_string_lossy(), contents)
    }

    fn read_to_string(&self, path: &Path) -> Result<String> {
        Ok(std::fs::read_to_string(path)?)
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn chown(&self, path: &Path, uid: Option<u32>, gid: Option<u32>) -> Result<()> {
        crate::util::sudo_chown(&path.to_string_lossy(), uid, gid)
    }
}
