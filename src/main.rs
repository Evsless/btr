mod utils;
mod error;
mod console;
mod database;

use crate::console::cli::TrackerCli;


fn main() {

    let mut tracker_cli = TrackerCli::new();

    tracker_cli.main_function();

}
