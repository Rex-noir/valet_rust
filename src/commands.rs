use clap::Subcommand;

use anyhow::Result;

use crate::commands::setup::SetupArgs;

mod install;
mod link;
mod setup;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Serve,
    #[command(subcommand)]
    Link(link::LinkCommand),
    Install,
    Setup(SetupArgs),
}

impl Commands {
    pub fn run(self) -> Result<()> {
        match self {
            Commands::Serve => todo!(),
            Commands::Link(args) => link::run(args),
            Commands::Install => install::run(),
            Commands::Setup(args) => setup::run(args),
        }
    }
}
