use anyhow::Result;

use crate::setup::{Caddy, Dns};

pub fn run() -> Result<()> {
    Dns::setup()?;
    Caddy::setup()?;

    Ok(())
}
