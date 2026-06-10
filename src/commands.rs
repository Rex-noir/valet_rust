use clap::Subcommand;

use anyhow::Result;

use crate::{commands::setup::SetupArgs, core::App};

mod install;
mod link;
mod setup;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Serve,
    Link(link::Args),
    Install,
    Setup(SetupArgs),
}

impl Commands {
    pub fn run(self, app: &App) -> Result<()> {
        match self {
            Commands::Serve => todo!(),
            Commands::Link(args) => link::run(app, args),
            Commands::Install => install::run(app),
            Commands::Setup(args) => setup::run(app, args),
        }
    }
}
