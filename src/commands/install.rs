use anyhow::Result;

use crate::{
    core::App,
    setup::{Dns, Nginx},
};

pub fn run(app: &App) -> Result<()> {
    Nginx::setup(app)?;
    Dns::setup(app)?;

    Ok(())
}
