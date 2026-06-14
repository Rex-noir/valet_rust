use anyhow::{Ok, Result};
use clap::{Args, Subcommand};

use crate::setup::{Caddy, Dns};

#[derive(Debug, Args)]
pub struct SetupArgs {
    #[command(subcommand)]
    pub command: SetupCommands,
}
#[derive(Debug, Subcommand)]
pub enum SetupCommands {
    Dns,
    Caddy,
    All,
}

pub fn run(args: SetupArgs) -> Result<()> {
    match args.command {
        SetupCommands::Dns => {
            println!("Setting up DNS...");
            Dns::setup()?;
            Ok(())
        }
        SetupCommands::Caddy => {
            println!("Setting up Caddy...");
            Caddy::setup()?;
            Ok(())
        }
        SetupCommands::All => {
            println!("Setting up all ");
            Ok(())
        }
    }
}
