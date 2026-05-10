pub mod commands;
pub mod configuration;
pub mod core;
pub mod services;

mod app;

pub use app::App;

use clap::Parser;

use commands::Commands;

const NGINX_CONFIG_STUB: &str = include_str!("./stubs/nginx.conf");
const NGINX_CONFIG_PATH: &str = "/etc/nginx/nginx.conf";

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}
