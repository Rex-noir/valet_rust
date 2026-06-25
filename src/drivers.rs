mod laravel;

use std::path::PathBuf;

use anyhow::Result;
use laravel::Laravel;

pub struct ServeContext {
    pub domain: Option<String>,
    pub path: PathBuf,
    pub php_path: Option<String>,
    pub node_path: Option<String>,
    pub php_fpm: Option<String>,
}

pub trait Driver {
    fn name(&self) -> &'static str;
    fn serves(&self, path: &std::path::Path) -> bool;
    fn serve(&self, ctx: ServeContext) -> Result<()>;
}

pub fn drivers() -> &'static [&'static dyn Driver] {
    &[&Laravel]
}
