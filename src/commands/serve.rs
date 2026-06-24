use std::{env, path::PathBuf};

use anyhow::{Result, anyhow};

use crate::drivers::{ServeContext, drivers};

#[derive(Debug, clap::Args)]
pub struct ServeArgs {
    #[arg()]
    pub domain: Option<String>,

    #[arg(default_value = ".")]
    pub path: String,

    #[arg(long)]
    pub php_path: Option<String>,

    #[arg(long)]
    pub node_path: Option<String>,
}

pub fn run(args: ServeArgs) -> Result<()> {
    println!("{args:?}");
    let path: PathBuf = if args.path.trim().is_empty() || args.path == "." {
        env::current_dir()?
    } else {
        PathBuf::from(&args.path)
    };

    let ctx = ServeContext {
        domain: args.domain,
        path,
        php_path: args.php_path,
        node_path: args.node_path,
    };

    let driver = drivers()
        .iter()
        .find(|d| d.serves(&ctx.path))
        .ok_or_else(|| anyhow!("No matching driver found"))?;

    driver.serve(ctx)?;

    Ok(())
}
