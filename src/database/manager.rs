use std::{fs::File, io::ErrorKind};
use crate::{database::config::TrackerConfig, };

pub struct TrackerManager {
    active_sheet: Option<ExpenseSheet>,
    config: TrackerConfig
}

pub struct ExpenseSheet {
    pub name: String
}

impl TrackerManager {
    pub fn new() -> Self {
        Self {
            active_sheet: None,
            config: TrackerConfig::new()
        }
    }

    pub fn month(&self, sheet_path: &str, truncate: bool) -> Result<(), std::io::Error> {

        let file = if truncate {
            File::create(sheet_path)?
        } else {
            File::create_new(sheet_path)?
        };

        /* __TO_BE_MODIFIED__ :: Write a default data to a sheet there. */
        Ok(())
    }
    // pub fn new() -> Self {
    //     let mut buffer = String::new();

    //     /* Setup the home directory for cross platform compatibility */
    //     let home_dir = home::home_dir().expect("Failed to get a home directory");

    //     /* Ensure that the base dir exists */
    //     let base_dir = home_dir.join(".btr");
    //     if let Err(e) = create_dir(&base_dir) {
    //         if e.kind() != ErrorKind::AlreadyExists {
    //             panic!("Failed to create .btr directory.");
    //         }
    //     }

    //     /* Check if the config file already exists */
    //     let cfg_path = base_dir.join("btr.conf");
    //     if !cfg_path.exists() {
    //         if let Ok(mut file) = OpenOptions::new().write(true).create(true).open(&cfg_path) {
    //             if let Err(e) = file.write_all(DEFAULT_CFG.as_bytes()) {
    //                 panic!("Failed to write a default configuration to a config file: {e}")
    //             }
    //         } else {
    //             panic!("Failed to create a default configuration file")
    //         }
    //     }

    //     /* Read the configuration from a file to RAM */
    //     let mut cfg_raw =
    //         File::open(cfg_path).expect("The line cannot fail as the code panicked previously.");

    //     if let Err(e) = cfg_raw.read_to_string(&mut buffer) {
    //         panic!("Failed to read a configuration file: {e}")
    //     }

    //     match toml::from_str(&buffer) {
    //         Ok(cfg) => Self {
    //             root: None,
    //             cfg: cfg,
    //         },
    //         Err(e) => {
    //             panic!("Failed to deserialize configuration file: {e}")
    //         }
    //     }
    // }
}
