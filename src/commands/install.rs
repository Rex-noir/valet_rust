use anyhow::Result;

use crate::{App, services::Nginx};

pub fn run(app: &App) -> Result<()> {
    Nginx::setup(app)?;

    Ok(())
}
