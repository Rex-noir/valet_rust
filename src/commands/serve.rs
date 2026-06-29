use std::{env, path::PathBuf};

use anyhow::{Result, anyhow};

use crate::core::fs::StdFs;
use crate::core::App;
use crate::drivers::{ServeContext, drivers};

#[derive(Debug, clap::Args)]
pub struct ServeArgs {
    #[arg(default_value = ".")]
    pub path: String,

    #[arg()]
    pub domain: Option<String>,

    #[arg(long)]
    pub php_version: Option<String>,
}

pub fn run(args: ServeArgs) -> Result<()> {
    App::init_with_fs(&StdFs)?;

    println!("{args:?}");
    let path: PathBuf = if args.path.trim().is_empty() || args.path == "." {
        env::current_dir()?
    } else {
        PathBuf::from(&args.path)
    };

    let ctx = ServeContext {
        domain: args.domain,
        path,
        php_version: args.php_version,
    };

    let driver = drivers()
        .iter()
        .find(|d| d.serves(&ctx.path))
        .ok_or_else(|| anyhow!("No matching driver found"))?;

    print!("Using {} driver.", driver.name());

    driver.serve(ctx)?;

    Ok(())
}
