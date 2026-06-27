use std::{env, fs, os::unix::fs::chown, path::PathBuf, sync::OnceLock};

use uzers::{get_current_username, get_user_by_name, os::unix::UserExt};

pub struct App {
    pub config_path: PathBuf,
    pub username: String,
    pub home_dir: PathBuf,
    pub nginx_files_path: PathBuf,
    pub uid: u32,
    pub gid: u32,
}

static INSTANCE: OnceLock<App> = OnceLock::new();

impl App {
    pub fn init() -> &'static Self {
        INSTANCE.get_or_init(|| {
            let username = env::var("SUDO_USER").unwrap_or_else(|_| {
                get_current_username()
                    .expect("failed to determine current user")
                    .into_string()
                    .expect("username is not valid UTF-8")
            });

            let user = get_user_by_name(&username).expect("failed to look up user");

            let home_dir = user.home_dir().to_path_buf();
            let uid = user.uid();
            let gid = user.primary_group_id();

            let config_path = home_dir.join(".config").join("valex");

            fs::create_dir_all(&config_path).expect("failed to create config directory");

            if env::var_os("SUDO_USER").is_some() {
                chown(&config_path, Some(uid), Some(gid))
                    .expect("failed to chown config directory");
            }

            let caddy_files_path = config_path.join("nginx");
            fs::create_dir_all(&caddy_files_path)
                .expect("Failed to create directory for caddy files");

            App {
                config_path,
                username,
                nginx_files_path: caddy_files_path,
                home_dir,
                uid,
                gid,
            }
        })
    }
}
