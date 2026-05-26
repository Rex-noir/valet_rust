use anyhow::Result;

use crate::{
    core::App,
    services::{DnsMasq, Nginx},
};

pub fn run(app: &App) -> Result<()> {
    Nginx::setup(app)?;
    DnsMasq::setup(app)?;

    Ok(())
}
