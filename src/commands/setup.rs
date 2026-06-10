use anyhow::{Ok, Result};
use clap::{Args, Subcommand};

use crate::{
    core::App,
    setup::{Dns, Nginx},
};

#[derive(Debug, Args)]
pub struct SetupArgs {
    #[command(subcommand)]
    pub command: SetupCommands,
}
#[derive(Debug, Subcommand)]
pub enum SetupCommands {
    Dns,
    Nginx,
    All,
}

pub fn run(app: &App, args: SetupArgs) -> Result<()> {
    match args.command {
        SetupCommands::Dns => {
            println!("Setting up DNS...");
            Dns::setup(app)?;
            Ok(())
        }
        SetupCommands::Nginx => {
            println!("Setting up nginx");
            Nginx::setup(app)?;
            Ok(())
        }
        SetupCommands::All => {
            println!("Setting up all ");
            Ok(())
        }
    }
}
