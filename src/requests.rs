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
}
