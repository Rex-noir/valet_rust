use clap::Subcommand;

use crate::app::App;
use anyhow::Result;

mod install;
mod link;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Serve,
    Link {
        name: String,
        path: Option<String>,
        #[arg(long, short)]
        php_version: Option<String>,
    },
    Install,
}

impl Commands {
    pub fn run(self, app: &App) -> Result<()> {
        match self {
            Commands::Serve => todo!(),
            Commands::Link {
                name: _name,
                path: _path,
                php_version: _php_version,
            } => todo!(),
            Commands::Install => install::run(app),
        }
    }
}
