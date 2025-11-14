use home;
use std::{path::PathBuf, sync::OnceLock};

static HOME_DIR: OnceLock<PathBuf> = OnceLock::new();

pub fn home_dir() -> &'static PathBuf {
    HOME_DIR.get_or_init(|| {
        home::home_dir()
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
