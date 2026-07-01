use anyhow::Result;

use crate::core::AppContext;
use crate::setup::{Dns, Nginx};

pub fn run(app: &AppContext) -> Result<()> {
    Dns::setup(app)?;
    Nginx::setup(app)?;

    Ok(())
}
