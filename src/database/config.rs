use crate::utils::Utils;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Core {
    pub expense_cat: Vec<String>,
    pub income_cat: Vec<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TrackerConfig {
    pub core: Option<Core>,
}


impl TrackerConfig {
    pub fn new() -> Self {
        let utils = Utils::new();

        let cfg_path = utils.home_dir()
            .join(utils.btr_dir())
            .join("config");

        if !cfg_path.exists() {
            return Self {core: None}
        }

        // TODO: Parse the config file
        Self {core: None} 
    }

}

