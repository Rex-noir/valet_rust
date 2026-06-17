use anyhow::{Ok, Result};
use clap::{Args, Subcommand};
use uzers::get_effective_uid;

use crate::setup::{Caddy, Dns};

#[derive(Debug, Args)]
pub struct SetupArgs {
    #[command(subcommand)]
    pub command: Option<SetupCommand>,
}

#[derive(Debug, Subcommand)]
pub enum SetupCommand {
    Dns,
    Caddy,
}

pub fn run(args: SetupArgs) -> Result<()> {
    match args.command {
        Some(cmd) => match cmd {
            SetupCommand::Dns => {
                println!("Setting up DNS...");
                Dns::setup()?;
            }
            SetupCommand::Caddy => {
                println!("Setting up Caddy...");
                Caddy::setup()?;
            }
        },
        None => {
            if get_effective_uid() == 0 {
                Dns::setup()?;
                Caddy::setup()?;
            } else {
                anyhow::bail!("This operation requires root privileges.");
            }
        }
    }

    Ok(())
}
