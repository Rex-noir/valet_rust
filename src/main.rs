use anyhow::Result;
use clap::Parser;
use valex::{
    Cli,
    core::{AppContext, SystemUserProvider},
};

fn main() -> Result<()> {
    let app = AppContext::build(&SystemUserProvider::new())?;
    let cli = Cli::parse();

    cli.commands.run(&app)?;

    Ok(())
}
