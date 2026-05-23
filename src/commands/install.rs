use anyhow::Result;

use crate::{
    App,
    services::{DnsMasq, Nginx},
};

pub fn run(app: &App) -> Result<()> {
    Nginx::setup(app)?;
    DnsMasq::setup(app)?;

    Ok(())
}
