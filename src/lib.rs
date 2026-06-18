pub mod commands;
pub mod core;
pub mod drivers;
pub mod setup;

use clap::Parser;

use commands::Commands;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}
