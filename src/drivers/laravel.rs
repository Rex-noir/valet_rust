use anyhow::Result;

use crate::drivers::{Driver, ServeContext};

pub struct Laravel;

impl Driver for Laravel {
    fn name(&self) -> &'static str {
        "Laravel"
    }

    fn serves(&self, path: &std::path::Path) -> bool {
        path.join("artisan").exists()
    }

    fn serve(&self, ctx: ServeContext) -> Result<()> {
        println!("Using laravel driver");

        Ok(())
    }
}
