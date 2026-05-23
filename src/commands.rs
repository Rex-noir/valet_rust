use clap::Subcommand;

use crate::app::App;
use anyhow::Result;

mod install;
mod link;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Serve,
    Link(link::Args),
    Install,
}

impl Commands {
    pub fn run(self, app: &App) -> Result<()> {
        match self {
            Commands::Serve => todo!(),
            Commands::Link(args) => link::run(app, args),
            Commands::Install => install::run(app),
        }
    }
}
