use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Core {
    pub expense_types: Vec<String>,
    pub income_types: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackerConfig {
    pub core: Option<Core>,
}


impl TrackerConfig {
    pub fn new() -> Self {
        Self { core: None }
    }
}