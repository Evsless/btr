use std::io::{self, Write};

mod config;
mod data;
mod requests;

fn main() {
    let mut request_handler = requests::RequestsHandler::new();
    let mut data_handler = data::DataHandler::new();

    let mut buffer = String::new();
    let stdin = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush().expect("Error flushing the buffer.");

        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        let tokens: Vec<&str> = buffer.split_ascii_whitespace().collect();

        if tokens.len() > 0 {
            match tokens[0] {
                "help" => request_handler.help(),
                "add" => request_handler.add(&tokens[1..]),
                _ => println!("> Unknown command!"),
            }
        }
    }
}
