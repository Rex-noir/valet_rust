use std::fs;

use anyhow::Result;
use slug::slugify;

use crate::{
    core::App,
    drivers::{Driver, ServeContext},
};

pub struct Laravel;

impl Driver for Laravel {
    fn name(&self) -> &'static str {
        "Laravel"
    }

    fn serves(&self, path: &std::path::Path) -> bool {
        path.join("artisan").exists() && path.join("public").join("index.php").exists()
    }

    fn serve(
        &self,
        ServeContext {
            php_fpm,
            domain,
            path,
            ..
        }: ServeContext,
    ) -> Result<()> {
        println!("→ Configuring Laravel site...");

        let php_fpm = php_fpm.ok_or_else(|| anyhow::anyhow!("Please provide php-fpm path."))?;

        let mut domain = domain.unwrap_or_else(|| {
            let name = path.file_name().unwrap().to_string_lossy();
            slugify(name.as_ref())
        });

        if !domain.ends_with(".test") {
            domain.push_str(".test");
        }

        println!("✓ Using domain: {}", domain);
        println!("✓ Using PHP-FPM: {}", php_fpm);

        let public_path = path.join("public");

        let caddyfile = format!(
            r#"{domain} {{
    tls internal
    root * {}
    encode zstd gzip

    php_fastcgi {php_fpm}
    file_server
}}
"#,
            public_path.display()
        );

        let app = App::init();
        let caddy_file_path = app.nginx_files_path.join(format!("{domain}.caddyfile"));

        println!("→ Writing Caddyfile to {}", caddy_file_path.display());

        fs::write(&caddy_file_path, caddyfile)?;

        println!("✓ Caddyfile created successfully");

        Ok(())
    }
}
