use clap::Subcommand;

use anyhow::Result;

use crate::commands::setup::SetupArgs;

mod install;
mod serve;
mod setup;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Serve(serve::ServeArgs),
    Install,
    Setup(SetupArgs),
}

impl Commands {
    pub fn run(self) -> Result<()> {
        match self {
            Commands::Serve(args) => serve::run(args),
            Commands::Install => install::run(),
            Commands::Setup(args) => setup::run(args),
        }
    }
}
