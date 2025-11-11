use home;
use std::path::PathBuf;

const DEFAULT_BTR_DIR: &str = ".btr";
const DEFAULT_SHEETS_DIR: &str = "sheets";

pub struct Utils {
    home_dir: PathBuf,
    btr_dir: PathBuf,
    sheets_dir: PathBuf,
}

impl Utils {
    fn find_home_dir() -> PathBuf {
        match home::home_dir() {
            Some(mut path) => {
                if !path.as_mut_os_str().is_empty() {
                    path
                } else {
                    panic!("> FATAL ERROR: The path to home directory is empty on your system")
                }
            }
            None => {
                panic!("> FATAL ERROR: Unable to get a home direc")
            }
        }
    }

    pub fn new() -> Self {
        Self {
            home_dir: Self::find_home_dir(),
            btr_dir: PathBuf::from(DEFAULT_BTR_DIR),
            sheets_dir: PathBuf::from(DEFAULT_SHEETS_DIR),
        }
    }

    pub fn home_dir(&self) -> &PathBuf {
        &self.home_dir
    }

    pub fn btr_dir(&self) -> &PathBuf {
        &self.btr_dir
    }

    pub fn sheets_dir(&self) -> PathBuf {
        self.home_dir
            .join(&self.btr_dir)
            .join(&self.sheets_dir)
    }
}
