use anyhow::Result;

use crate::core::fs::SudoFs;
use crate::core::App;
use crate::setup::{Dns, Nginx};

pub fn run() -> Result<()> {
    App::init_with_fs(&SudoFs)?;

    Dns::setup()?;
    Nginx::setup()?;

    Ok(())
}
