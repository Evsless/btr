use std::{fs::{File, read_to_string}, path::PathBuf, vec};
use crate::{
    database::config::expenses::{ExpenseCategory, ExpensesConfigRaw},
    error::BtrError, utils::Utils
};

use toml;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct TrackerConfig {
    expenses: Vec<ExpenseCategory>
}

impl TrackerConfig {
    fn default_cfg() -> Vec<ExpenseCategory> {
        vec![
            ExpenseCategory {
                name: String::from("Groceries"),
                description: Some(String::from("Groceries and dining"))
            },
            ExpenseCategory {
                name: String::from("Cafe && Bar"),
                description: Some(String::from("Coffee shops, bars, and related expenses"))
            },
            ExpenseCategory {
                name: String::from("Transport"),
                description: Some(String::from("Public transport tickets, taxi expenses"))
            }
        ]
    }

    pub fn new() -> Result<Self, BtrError> {
        let utils = Utils::new();

        let cfg_file = utils.home_dir()
            .join(utils.btr_dir())
            .join("cfg.toml");

        if !cfg_file.exists() {
            File::create_new(&cfg_file)?;
        }

        let cfg_str = read_to_string(cfg_file)?;
        let config: ExpensesConfigRaw = toml::from_str(&cfg_str)
            .map_err(|e| BtrError::InvalidData(
                Some(format!("Failed to parse configuration file: {}", e)))
            )?;

        let expenses = match config.expenses_cfg {
            Some(exp_cfg_path) => {
                let expenses_path = if let Ok(stripped) = exp_cfg_path.strip_prefix("~/") {
                    utils.home_dir().join(stripped)
                } else {
                    PathBuf::from(&exp_cfg_path)
                };

                if expenses_path.try_exists()? {
                    let expenses_cfg_str = read_to_string(&expenses_path)?;
    
                    /* Use a temporary wrapper instead of a global type there. It is needed just to parse a vector. */
                    #[derive(Deserialize)]
                    struct ExpensesWrapper {
                        expenses: Vec<ExpenseCategory>
                    }
    
                    let wrapped: ExpensesWrapper = toml::from_str(&expenses_cfg_str)
                        .map_err(|e| BtrError::InvalidData(
                            Some(format!("Failed to parse expenses configuration file: {}", e))
                        ))?;
                
                    wrapped.expenses
                } else {
                    Self::default_cfg()
                }
            },
            None => {
                Self::default_cfg()
            }
        };

        Ok(Self{ expenses })
    }


    pub fn expenses(&self) -> &[ExpenseCategory] {
        &self.expenses
    }

}
