use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TrackerState {
    pub selected_sheet: Option<PathBuf>,
}
