use anyhow::Result;
use clap::Parser;
use valet_rust::{Cli, core::App};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let app = App::new()?;

    cli.commands.run(&app)?;

    Ok(())
}
