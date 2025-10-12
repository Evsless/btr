use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Core {
    pub expense_types: Vec<String>,
    pub income_types: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub core: Core,
}
