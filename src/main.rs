use anyhow::Result;
use clap::Parser;
use valet_rust::{App, Cli};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let app = App::new()?;

    cli.commands.run(&app)?;

    Ok(())
}
