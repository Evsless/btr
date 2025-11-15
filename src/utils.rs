use dirs;
use std::{path::PathBuf, sync::OnceLock};

static USER_CONFIG_DIR: OnceLock<PathBuf> = OnceLock::new();
static APP_STATE_DIR: OnceLock<PathBuf> = OnceLock::new();

pub fn home_dir() -> &'static PathBuf {
    USER_CONFIG_DIR.get_or_init(|| {
        dirs::home_dir()
            .filter(|p| !p.as_os_str().is_empty())
            .expect("Unable to get home directory.")
    })
}

pub fn btr_dir() -> PathBuf {
    home_dir().join(".btr")
}

pub fn sheets_dir() -> PathBuf {
    btr_dir().join("sheets")
}

pub fn state_dir() -> PathBuf {
    let system_state_dir = APP_STATE_DIR.get_or_init(|| {
        dirs::data_local_dir()
            .filter(|p| !p.as_os_str().is_empty())
            .expect("Unable to get app data directory.")
    });

    system_state_dir.join("btr")
}
