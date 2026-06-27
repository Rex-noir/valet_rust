use anyhow::Result;

use crate::setup::{Dns, Nginx};

pub fn run() -> Result<()> {
    Dns::setup()?;
    Nginx::setup()?;

    Ok(())
}
