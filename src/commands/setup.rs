use anyhow::{Ok, Result};
use clap::{Args, Subcommand};

use crate::setup::{Dns, Nginx, PHPFpm};

#[derive(Debug, Args)]
pub struct SetupArgs {
    #[command(subcommand)]
    pub command: Option<SetupCommand>,
}

#[derive(Debug, Subcommand)]
pub enum SetupCommand {
    Dns,
    Nginx,
    PHPFpm,
}

pub fn run(args: SetupArgs) -> Result<()> {
    match args.command {
        Some(cmd) => match cmd {
            SetupCommand::Dns => {
                Dns::setup()?;
            }
            SetupCommand::Nginx => {
                Nginx::setup()?;
            }
            SetupCommand::PHPFpm => {
                PHPFpm::setup()?;
            }
        },
        None => {
            Dns::setup()?;
            Nginx::setup()?;
            PHPFpm::setup()?;
        }
    }

    Ok(())
}
