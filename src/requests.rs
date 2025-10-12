use std::{io::{ErrorKind, Write}};
use std::fs::{
    self,
    OpenOptions
};


use chrono::{Datelike, NaiveDate, Utc};
use crate::{config::{self, Config}, data::ExpenseSheet};

enum Request {
    Add,
    Remove,
    Modify,
    Show,
    None,
}

pub struct RequestsHandler {
    active_request: Request,
}

impl RequestsHandler {
    pub fn new() -> Self {
        Self {
            active_request: Request::None,
        }
    }

    pub fn help(&mut self) {
        match self.active_request {
            Request::Add => {
                println!(
                    ">> add: add a new element to database
                    \r>> month - create a new sheet with expenses.
                    \r>> expense - create a new expense row in an active month.
                    \r>> year - create a new sheet with year expenses."
                )
            }
            Request::None => {
                println!(
                    ">> Help: budget tracker application
                    \r>> add - add a new item to a database.
                    \r>> rm  - remove an item from a database.
                    \r>> mod - modify an item from a database.
                    \r>> show - list an item from a database."
                );
            }
            _ => println!("To be done"),
        }

        self.active_request = Request::None
    }

    pub fn add(&mut self, args: &[&str]) {
        self.active_request = Request::Add;

        if args.len() > 0 {
            match args[0] {
                "help" => self.help(),
                "expense" => {
                    self.expense(&args[1..]);
                }
                "month" => {
                    self.month(&args[1..]);
                }
                "year" => {
                    println!("Code to handler year-s creation");
                }
                _ => println!(
                    "> ERROR: Unsupported command: {}. Check 'add help'.",
                    args[0]
                ),
            }
        } else {
            println!("> ERROR: 'add' expects at least one argument. Check help.");
        }
    }

    /* __TO_BE_MODIFIED__ :: Check how to handle the command help (if possible) */
    fn expense(&self, args: &[&str]) {

        if args.len() < 2 {
            eprintln!("> ERROR: 'expense' expects at least two arguments. Check help.")
        }
        
        /* __TO_BE_MODIFIED__ :: Use a home there instead of hardcoded /home/evsless. */
        if let Ok(cfg_str) = fs::read_to_string("/home/evsless/.btr/btr.conf") {
            if let Ok(cfg) = toml::from_str::<Config>(&cfg_str) {
                println!("Parsed config: {:?}", cfg);

                let expense_type = args[0];
                let amount = args[1];

                if cfg.core.expense_types.iter().any(|t| t == expense_type) {
                    let expense_sheet = ExpenseSheet::get_data("09_2025");

                    match  expense_sheet {
                        Ok(curr_expenses) => {
                            /* Create a new expense field, write and save a new file */
                            curr_expenses.new_record(&expense_type, &amount);
                        },
                        Err(e) => {
                            eprintln!("> ERROR: Error reading the expense sheet: {}", e)
                        }
                    }
                }
            }
        } else {
            eprintln!("> ERROR: Failed to read a configuration file to string.");
        }
    }

    fn month(&self, args: &[&str]) {
        let mut sheet_name = String::new();

        if args.len() > 0 {
            sheet_name = args[0].to_string();
        } else {
            let now_utc = Utc::now().date_naive();
            sheet_name = format!("{:02}_{}.json", now_utc.month(), now_utc.year());
        }

        /* Create an expense sheet if doesn't exist yet */
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(format!("/home/evsless/.btr/{}", sheet_name))
        {
            Ok(mut file) => {
                /* Create a default sheet */
                let default_data_t = ExpenseSheet::new();
                
                /* Write the default data ot a sheet */
                let default_data_s = serde_json::to_string(&default_data_t)
                    .expect("Update the error handling there.");

                file.write_all(default_data_s.as_bytes())
                    .expect("Update the error handling there");
            }
            Err(e) => {
                if e.kind() != ErrorKind::AlreadyExists {
                    eprintln!("! Error when creating a new expense sheet: {e}.");
                } else {
                    println!("> INFO: expense sheet already exists.");
                }
            }
        }
    }
}
