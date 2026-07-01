use anyhow::{Ok, Result};
use clap::{Args, Subcommand};

use crate::core::AppContext;
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

pub fn run(args: SetupArgs, app: &AppContext) -> Result<()> {
    match args.command {
        Some(cmd) => match cmd {
            SetupCommand::Dns => {
                Dns::setup(app)?;
            }
            SetupCommand::Nginx => {
                Nginx::setup(app)?;
            }
            SetupCommand::PHPFpm => {
                PHPFpm::setup(app)?;
            }
        },
        None => {
            Dns::setup(app)?;
            Nginx::setup(app)?;
            PHPFpm::setup(app)?;
        }
    }

    Ok(())
}
