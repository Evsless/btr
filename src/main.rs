mod console;
mod database;
mod error;
mod utils;

use crate::console::cli::TrackerCli;

fn main() {
    match TrackerCli::new() {
        Ok(mut cli) => cli.main_function(),
        Err(e) => {
            eprintln!("! ERROR. Unable to start application: {}", e)
        }
    };
}
