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

    pub fn help(&self) {
        println!(">> Help: budget tracker application
            \r>> add - add a new item to a database.
            \r>> rm  - remove an item from a database.
            \r>> mod - modify an item from a database.
            \r>> show - list an item from a database.")
    }
}
