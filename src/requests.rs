use std::fs::OpenOptions;

use chrono::{Datelike, Utc};

enum Request {
    Add,
    Remove,
    Modify,
    Show,
    None,
}

pub struct RequestsHandler {
    active_request: Request
}

impl RequestsHandler {
    pub fn new() -> Self {
        Self { active_request: Request::None }
    }

    pub fn help(&mut self) {
        match self.active_request {
            Request::Add => {
                println!(">> add: add a new element to database
                    \r>> month - create a new sheet with expenses.
                    \r>> year - create a new sheet with year expenses.")
            },
            Request::None => {
                println!(">> Help: budget tracker application
                    \r>> add - add a new item to a database.
                    \r>> rm  - remove an item from a database.
                    \r>> mod - modify an item from a database.
                    \r>> show - list an item from a database.");
            }
            _ => println!("To be done")
        }

        self.active_request = Request::None
    }

    pub fn add(&mut self, args: &[&str]) {
        self.active_request = Request::Add;

        if args.len() > 0 {
            match args[0] {
                "help" => self.help(),
                "month" => {
                    self.month(&args[1..]);
                    println!("Code to handle month creation");
                }
                "year" => {
                    println!("Code to handler year-s creation");
                }
                _ => println!("> ERROR: Unsupported command: {}. Check 'add help'.", args[0])
            }
        } else {
            println!("> ERROR: 'add' expects at least one argument. Check help");
        }
    }

    fn month(&self, args: &[&str]) {
        let mut sheet_name = String::new();
        
        if args.len() > 0 {
            sheet_name = args[0].to_string();
        } else {
            let now_utc = Utc::now().date_naive();
            sheet_name = format!("{:02}_{}.json", now_utc.month(), now_utc.year());

            let sheet = OpenOptions::new()
                .write(true)
                .create(true)
                .open(format!("/home/evsless/.btr/{}", sheet_name));

            println!("{}", format!("~/.btr/{}", sheet_name));

            match sheet {
               Ok(file) => {
                println!("* Created a new expenses sheet.")
                /* Write a root node to an expense sheet there. */
               },
               Err(e)  => println!("* ERROR: Problem occured when creating expense sheet: {}", e),
            }
        }
    }
}
