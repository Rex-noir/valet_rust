mod laravel;

use std::path::PathBuf;

use anyhow::Result;
use laravel::Laravel;

use crate::core::AppContext;

pub struct ServeContext {
    pub domain: Option<String>,
    pub path: PathBuf,
    pub php_version: Option<String>,
}

pub trait Driver {
    fn name(&self) -> &'static str;
    fn serves(&self, path: &std::path::Path) -> bool;
    fn serve(&self, ctx: ServeContext, app: &AppContext) -> Result<()>;
}

pub fn drivers() -> &'static [&'static dyn Driver] {
    &[&Laravel]
}
