use clap::Parser;

use crate::{app::App, commands::Commands};
use anyhow::{Ok, Result};

mod app;
mod commands;
mod configuration;
mod core;
mod services;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let app = App::new()?;

    cli.commands.run(&app)?;

    Ok(())
}
