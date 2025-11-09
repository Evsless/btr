use std::{fs::File, io::Write};
use serde::{Deserialize, Serialize};

use crate::{
    database::{
        config::tracker::TrackerConfig, 
        periods::Period
    }, error::BtrError, utils::Utils
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
        let config = match TrackerConfig::new() {
            Ok(cfg) => cfg,
            Err(e) => panic!("Failed to extract tracker configuration: {}", e),
        };

        Self {
            active_sheet: None,
            config: config
        }
    }

    pub fn new_sheet(&mut self, sheet_name: &str, period: Period, truncate: bool) -> Result<(), BtrError> {
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
            .map_err(|e| BtrError::InvalidData(Some(format!("Failed to serialize a JSON data: {}", e))))?;

        file.write_all(json.as_bytes())?;

        Ok(())
    }

}
