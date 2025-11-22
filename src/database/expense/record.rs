use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpenseRecord {
    category: String,
    amount: f32,
    logged_on: NaiveDate,
}

impl ExpenseRecord {
    pub fn new(category: String, amount: f32, logged_on: NaiveDate) -> Self {
        Self {
            category,
            amount,
            logged_on,
        }
    }

    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn amount(&self) -> f32 {
        self.amount
    }

    pub fn logged_on(&self) -> NaiveDate {
        self.logged_on
    }
}
