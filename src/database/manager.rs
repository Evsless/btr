use std::{fs::File, io::Write};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{
    database::{
        config::TrackerConfig, 
        periods::Period
    }, 
    utils::Utils
};


pub struct TrackerManager {
    active_sheet: Option<ExpenseSheet>,
    config: TrackerConfig
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpenseRecord {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpenseSheet {
    pub name: String, /* TODO: Try to modify to &str */
    pub period: Period,
    pub expenses: Vec<ExpenseRecord> 
}

impl TrackerManager {
    pub fn new() -> Self {
        Self {
            active_sheet: None,
            config: TrackerConfig::new()
        }
    }

    pub fn new_sheet(&mut self, sheet_name: &str, period: Period, truncate: bool) -> Result<(), std::io::Error> {
        let utils = Utils::new();

        /* Setup a path to a sheet based on a configuration */
        let sheet_path = utils.home_dir()
            .join(utils.btr_dir())
            .join(format!("{}.json", sheet_name));

        let mut file = if truncate {
            File::create(sheet_path)?
        } else {
            File::create_new(sheet_path)?
        };

        let empty_sheet = ExpenseSheet{
            name: sheet_name.to_string(),
            period: period,
            expenses: Vec::new()
        };

        let json = serde_json::to_string_pretty(&empty_sheet)
            .expect("I think I have to modify the return value there. Probably returning a string would be a good thing");

        file.write_all(json.as_bytes())?;

        Ok(())
    }

}
