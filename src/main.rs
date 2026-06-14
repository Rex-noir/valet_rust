use anyhow::Result;
use clap::Parser;
use valet_rust::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    cli.commands.run()?;

    Ok(())
}
