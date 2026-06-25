use anyhow::Result;
use clap::Parser;
use valex::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    cli.commands.run()?;

    Ok(())
}
