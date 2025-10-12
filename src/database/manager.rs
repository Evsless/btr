// use home;
// use core::panic;
// use chrono::{Datelike, NaiveDate};
// use std::fs::{self, File, OpenOptions, create_dir};
// use std::io::{ErrorKind, Read, Write};
// use serde::{Serialize, Deserialize};

// use crate::database_config::Config;

/* __TO_BE_MODIFIED__ :: Redo the structure there. */
// const DEFAULT_CFG: &str = r#"[core]
// expense_types = ["groceries", "transport", "entertainment"]
// income_types  = ["salary", "freelance", "dividends"]
// "#;

// #[derive(Serialize, Deserialize)]
// struct ExpenseRecord {
//     field: String,
//     spent: f32,
//     date: NaiveDate
// }

// #[derive(Serialize, Deserialize)]
// pub struct ExpenseSheet {
//     period: String, /* __TO_BE_MODIFIED__ :: A simplified data type to hold month and year only? */SD;LKJL
//     savings: f32,
//     start_amount: f32,
//     currently_spent: f32, 
//     expenses: Vec<ExpenseRecord>
// }

// pub struct DataHandler {
//     root: Option<File>,
//     cfg: Config
// }

// impl ExpenseSheet {
//     pub fn new() -> Self {
//         let curr_date = chrono::Utc::now().date_naive();
//         ExpenseSheet{
//             period: format!("{}-{}", curr_date.month(), curr_date.year()),
//             savings: 0.0,
//             start_amount: 0.0,
//             currently_spent: 0.0,
//             expenses: Vec::new(),
//         }
//     }

//     pub fn get_data(expense_sheet: &str) -> Result<Self, Box<dyn std::error::Error>> {
//         let path = format!("/home/evsless/.btr/{}.json", expense_sheet);
        
//         let expense_sheet_str = fs::read_to_string(&path)?;
//         let sheet = serde_json::from_str::<Self>(&expense_sheet_str)?;

//         Ok(sheet)
//     }

//     pub fn new_record(&self, expense_type: &str, amount: &str) {
//         println!(">> DEBUG: {}", self.period);
//     }
    
// }

pub struct TrackerManager {
    tmp: String
}

impl TrackerManager {
    pub fn new() -> Self {
        Self { tmp: String::from("Hello there!") }
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
