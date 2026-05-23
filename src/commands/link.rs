use anyhow::Result;
use clap::Args as ClapArgs;

use crate::App;

#[derive(Debug, ClapArgs)]
pub struct Args {
    pub name: String,
    pub path: Option<String>,
    #[arg(long, short)]
    #[arg(long, short)]
    pub php_version: Option<String>,
}

pub fn run(app: &App, args: Args) -> Result<()> {
    let Args {
        name,
        path,
        php_version,
    } = args;
    println!("{name:?} {path:?} {php_version:?}");
    Ok(())
}
