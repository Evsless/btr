use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpensesConfigRaw {
    pub expenses_cfg: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpenseCategory {
    pub name: String,
    pub description: Option<String>,
}
