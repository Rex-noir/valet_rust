use anyhow::Result;
use clap::Args as ClapArgs;

#[derive(Debug, ClapArgs)]
pub struct Args {
    pub name: String,
    pub path: Option<String>,
    #[arg(long, short)]
    #[arg(long, short)]
    pub php_version: Option<String>,
}

pub fn run(args: Args) -> Result<()> {
    let Args {
        name,
        path,
        php_version,
    } = args;
    println!("{name:?} {path:?} {php_version:?}");
    Ok(())
}
