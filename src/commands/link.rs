use anyhow::Result;

#[derive(Debug, clap::Subcommand)]
pub enum LinkCommand {
    Laravel {
        name: String,
        path: Option<String>,

        #[arg(long)]
        php_version: Option<String>,
    },
    Symfony {
        name: String,
        path: Option<String>,

        #[arg(long)]
        php_version: Option<String>,
    },
    Vite {
        name: String,
        #[arg(long, default_value = ".")]
        path: Option<String>,

        #[arg(long, default_value_t = 5173)]
        port: u16,
    },
}

pub fn run(command: LinkCommand) -> Result<()> {
    println!("{command:?}");
    Ok(())
}
