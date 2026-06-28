use crate::core::Configuration;
use anyhow::Result;
use std::{env, fs, os::unix::fs::chown, path::PathBuf, sync::OnceLock};
use uzers::{get_current_groupname, get_current_username, get_user_by_name, os::unix::UserExt};

pub struct App {
    pub app_dir: PathBuf,
    pub config_file: PathBuf,
    pub config: Configuration,
    pub username: String,
    pub groupname: String,
    pub home_dir: PathBuf,
    pub nginx_files_path: PathBuf,
    pub uid: u32,
    pub gid: u32,
}

static INSTANCE: OnceLock<App> = OnceLock::new();

impl App {
    pub fn init() -> Result<&'static Self> {
        if let Some(app) = INSTANCE.get() {
            return Ok(app);
        }

        let username = env::var("SUDO_USER").unwrap_or_else(|_| {
            get_current_username()
                .expect("failed to determine current user")
                .into_string()
                .expect("username is not valid UTF-8")
        });
        let groupname = get_current_groupname()
            .expect("Failed  to get user groupname")
            .into_string()
            .expect("Groupname is not a valid groupname");

        let user = get_user_by_name(&username).expect("failed to look up user");
        let home_dir = user.home_dir().to_path_buf();
        let uid = user.uid();
        let gid = user.primary_group_id();
        let app_dir = home_dir.join(".config").join("valex");
        let config_path = app_dir.join("config.json5");

        fs::create_dir_all(&app_dir).expect("failed to create config directory");
        if env::var_os("SUDO_USER").is_some() {
            chown(&app_dir, Some(uid), Some(gid)).expect("failed to chown config directory");
        }

        let nginx_files_path = app_dir.join("nginx");
        fs::create_dir_all(&nginx_files_path).expect("failed to create directory for caddy files");

        let config = Configuration::load_or_default(&config_path)?;

        let app = App {
            app_dir,
            config_file: config_path,
            config,
            username,
            groupname,
            nginx_files_path,
            home_dir,
            uid,
            gid,
        };

        Ok(INSTANCE.get_or_init(move || app))
    }
}
