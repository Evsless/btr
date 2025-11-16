use crate::{
    database::config::state::TrackerState,
    database::expense::{ExpenseCategory, ExpensesConfigRaw},
    error::BtrError,
    utils,
};
use std::{
    fs::{self, File, create_dir_all, read_to_string},
    path::PathBuf,
    vec,
};

use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackerConfig {
    #[serde(skip_deserializing)]
    state: TrackerState,
    expenses: Vec<ExpenseCategory>,
}

impl TrackerConfig {
    fn default_cfg() -> Vec<ExpenseCategory> {
        vec![
            ExpenseCategory {
                name: String::from("Groceries"),
                description: Some(String::from("Groceries and dining")),
            },
            ExpenseCategory {
                name: String::from("Cafe && Bar"),
                description: Some(String::from("Coffee shops, bars, and related expenses")),
            },
            ExpenseCategory {
                name: String::from("Transport"),
                description: Some(String::from("Public transport tickets, taxi expenses")),
            },
        ]
    }

    fn get_state() -> Result<TrackerState, BtrError> {
        let state_file = utils::state_dir().join("state");

        if !state_file.exists() {
            if let Some(parent) = state_file.parent() {
                create_dir_all(parent)?;
            }
            File::create_new(&state_file)?;
        }

        let state_str = read_to_string(state_file)?;

        let state: TrackerState = toml::from_str(&state_str).map_err(|e| {
            BtrError::InvalidData(Some(format!("Failed to parse a state TOML: {}", e)))
        })?;

        Ok(state)
    }

    fn save_state(&self) -> Result<(), BtrError> {
        let state_file = utils::state_dir().join("state");

        let state_str = toml::to_string(&self.state).map_err(|e| {
            BtrError::InvalidData(Some(format!("failed to serialize state: {}", e)))
        })?;

        fs::write(state_file, state_str)?;

        Ok(())
    }

    pub fn new() -> Result<Self, BtrError> {
        let cfg_file = utils::btr_dir().join("cfg.toml");

        if !cfg_file.exists() {
            File::create_new(&cfg_file)?;
        }

        let cfg_str = read_to_string(cfg_file)?;
        let config: ExpensesConfigRaw = toml::from_str(&cfg_str).map_err(|e| {
            BtrError::InvalidData(Some(format!("Failed to parse configuration file: {}", e)))
        })?;

        let expenses = match config.expenses_cfg {
            Some(exp_cfg_path) => {
                let expenses_path = if let Ok(stripped) = exp_cfg_path.strip_prefix("~/") {
                    utils::home_dir().join(stripped)
                } else {
                    PathBuf::from(&exp_cfg_path)
                };

                if expenses_path.try_exists()? {
                    let expenses_cfg_str = read_to_string(&expenses_path)?;

                    let parsed_cfg: TrackerConfig =
                        toml::from_str(&expenses_cfg_str).map_err(|e| {
                            BtrError::InvalidData(Some(format!(
                                "Failed to parse expenses configuration file: {}",
                                e
                            )))
                        })?;

                    parsed_cfg.expenses
                } else {
                    Self::default_cfg()
                }
            }
            None => Self::default_cfg(),
        };

        let state = Self::get_state()?;

        Ok(Self { state, expenses })
    }

    pub fn update_state<F>(&mut self, updater: F) -> Result<(), BtrError>
    where
        F: FnOnce(&mut TrackerState),
    {
        updater(&mut self.state);
        self.save_state()?;

        Ok(())
    }

    pub fn load_active_sheet(&self) -> Option<&PathBuf> {
        self.state.selected_sheet.as_ref()
    }

    pub fn expenses(&self) -> &[ExpenseCategory] {
        &self.expenses
    }
}
