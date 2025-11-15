use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
};

use crate::{
    database::{
        config::{expenses::ExpenseCategory, tracker::TrackerConfig},
        periods::Period,
    },
    error::BtrError,
    utils,
};

pub struct TrackerManager {
    active_sheet: Option<ExpenseSheet>,
    config: TrackerConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpenseRecord {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpenseSheet {
    pub name: String, /* TODO: Try to modify to &str */
    pub period: Period,
    pub expenses: Vec<ExpenseRecord>,
}

impl TrackerManager {
    pub fn new() -> Result<Self, BtrError> {
        let config = match TrackerConfig::new() {
            Ok(cfg) => cfg,
            Err(e) => panic!("Failed to extract tracker configuration: {}", e),
        };

        let active_sheet = if let Some(active_sheet_path) = config.load_active_sheet() {
            let active_sheet_str = fs::read_to_string(&active_sheet_path)?;

            serde_json::from_str::<ExpenseSheet>(&active_sheet_str).ok()
        } else {
            None
        };

        Ok(Self {
            active_sheet,
            config,
        })
    }

    pub fn new_sheet(
        &mut self,
        sheet_name: &str,
        period: Period,
        truncate: bool,
    ) -> Result<(), BtrError> {
        /* Check if the directory with the sheets exists. */
        let sheet_dir = utils::sheets_dir();
        if !sheet_dir.exists() {
            fs::create_dir_all(&sheet_dir)?;
        }

        /* Setup a path to a sheet. */
        let sheet_path = sheet_dir.join(format!("{}.json", sheet_name));

        let mut file = if truncate {
            File::create(sheet_path)?
        } else {
            File::create_new(sheet_path)?
        };

        let empty_sheet = ExpenseSheet {
            name: sheet_name.to_string(),
            period: period,
            expenses: Vec::new(),
        };

        let json = serde_json::to_string_pretty(&empty_sheet).map_err(|e| {
            BtrError::InvalidData(Some(format!("Failed to serialize a JSON data: {}", e)))
        })?;

        file.write_all(json.as_bytes())?;

        Ok(())
    }

    pub fn get_categories(&self) -> &[ExpenseCategory] {
        self.config.expenses()
    }

    pub fn set_active_sheet(&mut self, sheet_name: &str) -> Result<(), BtrError> {
        let sheet_path = utils::sheets_dir().join(format!("{}.json", sheet_name));

        let sheet_content = fs::read_to_string(&sheet_path)?;
        let active_sheet: ExpenseSheet = serde_json::from_str(&sheet_content).map_err(|e| {
            BtrError::InvalidData(Some(format!("Failed to deserialize a sheet data: {}", e)))
        })?;

        self.active_sheet = Some(active_sheet);

        self.config
            .update_state(|state| state.selected_sheet = Some(sheet_path))?;

        Ok(())
    }

    pub fn get_active_sheet(&self) -> &Option<ExpenseSheet> {
        &self.active_sheet
    }
}
