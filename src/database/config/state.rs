use std::{collections::VecDeque, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TrackerState {
    pub selected_sheet: Option<PathBuf>,
    #[serde(default)]
    pub cmd_history: VecDeque<String>,
}

impl TrackerState {
    pub fn add_cmd_to_history(&mut self, command: String) {
        if self.cmd_history.back() != Some(&command) {
            self.cmd_history.push_back(command);
        }

        if self.cmd_history.len() > 100 {
            self.cmd_history.pop_front();
        }
    }
}
