mod utils;
mod console;
mod database;

use crate::console::cli::TrackerCli;


fn main() {
    // let utils = Utils::new();

    /* Create a CLI handler */
    let mut tracker_cli = TrackerCli::new();

    loop {
        tracker_cli.main_function();
    }
    // let mut request_handler = requests::RequestsHandler::new();
    // let mut data_handler = database::DataHandler::new();

    // let mut buffer = String::new();
    // let stdin = io::stdin();

    // loop {
    //     print!("> ");
    //     io::stdout().flush().expect("Error flushing the buffer.");

    //     buffer.clear();
    //     stdin.read_line(&mut buffer).unwrap();
    //     let tokens: Vec<&str> = buffer.split_ascii_whitespace().collect();

    //     if tokens.len() > 0 {
    //         match tokens[0] {
    //             "help" => request_handler.help(),
    //             "add" => request_handler.add(&tokens[1..]),
    //             _ => println!("> Unknown command!"),
    //         }
    //     }
    // }
}
