pub mod commands;
pub mod core;
pub mod drivers;
pub mod setup;
pub mod util;

use clap::Parser;

use commands::Commands;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}
