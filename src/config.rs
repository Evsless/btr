use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Core {
    pub expense_types: Vec<String>,
    pub income_types: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub core: Core,
}
