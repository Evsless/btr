use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpenseRecord {
    category: String,
    amount: f32,
}

impl ExpenseRecord {
    pub fn new(category: String, amount: f32) -> Self {
        Self { category, amount }
    }
}
