use std::{
    env::{self, home_dir},
    fs,
    os::unix::fs::chown,
    path::PathBuf,
    sync::OnceLock,
};

use uzers::{get_user_by_name, os::unix::UserExt};

pub struct App {
    pub config_path: PathBuf,
}

static INSTANCE: OnceLock<App> = OnceLock::new();

impl App {
    pub fn init() -> &'static Self {
        INSTANCE.get_or_init(|| {
            let (home, uid, gid) = if let Ok(sudo_user) = env::var("SUDO_USER") {
                let user = get_user_by_name(&sudo_user).expect("failed to find SUDO_USER");

                (
                    user.home_dir().to_path_buf(),
                    Some(user.uid()),
                    Some(user.primary_group_id()),
                )
            } else {
                (
                    home_dir().expect("failed to determine home directory"),
                    None,
                    None,
                )
            };

            let config_path = home.join(".config").join("valet_rust");

            fs::create_dir_all(&config_path).expect("failed to create config directory");

            if let (Some(uid), Some(gid)) = (uid, gid) {
                chown(&config_path, Some(uid), Some(gid))
                    .expect("failed to chown config directory");
            }

            App { config_path }
        })
    }
}
