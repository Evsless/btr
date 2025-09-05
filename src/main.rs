use std::{io::{self, Write}};

mod requests;
mod data;


fn main() {
    let mut data_handler = data::DataHandler::new();
    let request_handler = requests::RequestsHandler::new();
    
    if let Err(e) = data_handler.data_init() {
        println!("Error when creating DB occured: {}", e)
    }

    let mut buffer = String::new();
    let stdin = io::stdin();

    loop {
        print!("> "); 
        io::stdout().flush().expect("Error flushing the buffer.");

        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        let tokens: Vec<&str> = buffer.split_ascii_whitespace().collect();

        match tokens.as_slice() {
            ["help"] => request_handler.help(),
            [] => continue,
            _ => println!("> Unknown command!")
        }

    }
}
