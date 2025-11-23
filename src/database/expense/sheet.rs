use crate::{
    database::{
        expense::ExpenseRecord,
        periods::{self, Period},
    },
    error::BtrError,
    utils,
};
use serde::{Deserialize, Serialize};
use std::fs::write;

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpenseSheet {
    pub name: String, /* TODO: Try to modify to &str */
    pub period: Period,
    expenses: Vec<ExpenseRecord>,
}

impl ExpenseSheet {
    pub fn new(name: String, period: Period, expenses: Vec<ExpenseRecord>) -> Self {
        Self {
            name,
            period,
            expenses,
        }
    }

    pub fn save_sheet(&self) -> Result<(), BtrError> {
        let sheet_path = utils::sheets_dir().join(format!("{}.json", &self.name));

        let sheet_str = serde_json::to_string_pretty(&self).map_err(|e| {
            BtrError::InvalidData(Some(format!("Failed to serialize the data: {}", e)))
        })?;

        write(sheet_path, &sheet_str)?;

        Ok(())
    }

    pub fn update<F>(&mut self, updater: F) -> Result<(), BtrError>
    where
        F: FnOnce(&mut ExpenseSheet),
    {
        updater(self);

        self.save_sheet()?;

        Ok(())
    }

    pub fn expenses(&self) -> &[ExpenseRecord] {
        &self.expenses
    }

    pub fn expenses_mut(&mut self) -> &mut Vec<ExpenseRecord> {
        &mut self.expenses
    }
}
