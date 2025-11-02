use chrono::{Datelike, NaiveDate, Utc};
use std::io::{ErrorKind, Write, stdin, stdout};

use crate::database::{
    manager::TrackerManager,
};


pub struct TrackerCli {
    buffer: String,
    cmd_tree: CommandNode,
    tracker_manager: TrackerManager
}


pub struct CommandNode {
    cmd: String,
    description: String,
    children: Vec<CommandNode>,
    handler: Option<fn(&mut TrackerManager, &[&str]) -> Result<(), String>>,
    extra_args: bool
}


impl TrackerCli {
    pub fn new() -> Self {
        let cmd_tree = 
            CommandNode::new("root", "Budget tracker CLI.", None)
            .add_child(
                CommandNode::new("add", "Add a new record to a budget tracker.", None)
                    .add_child(CommandNode::new(
                        "expense",
                        "Add a new expense record to an active sheet.",
                        Some(Self::add_expense_handler))
                    )
                    .add_child(CommandNode::new(
                        "month",
                        "Add a new month sheet to an expenses database.",
                        Some(Self::add_sheet_handler))
                    )
                    .add_child(CommandNode::new(
                        "year", 
                        "Add a new year sheet to an expenses database.",
                        Some(Self::add_sheet_handler))
                    )
            )
            .add_child(CommandNode::new("remove", "Remove a record from an expenses database.", None))
            .add_child(CommandNode::new("modify", "Modify an existing record in an expenses database", None));

        Self { 
            buffer: String::new(), 
            cmd_tree: cmd_tree,
            tracker_manager: TrackerManager::new() }
    }


    fn add_expense_handler(manager: &mut TrackerManager, args: &[&str]) -> Result<(), String>{
        println!("Enteren add handler!");
        Ok(())
    }

    fn user_input() -> Result<String, String> {
        print!("> ");
        let _ = stdout().flush();

        let mut buffer = String::new();
        stdin().read_line(&mut buffer)
            .map_err(|e| format!("Error reading from stdin: {}", e))?;

        Ok(buffer)
    } 

    fn create_sheet_with_prompt(
        manager: &mut TrackerManager, 
        sheet_name: &str,
        period: (NaiveDate, NaiveDate)
    ) -> Result<(), String> {
        if let Err(e) = manager.new_sheet(sheet_name, period, false) {
            if e.kind() == ErrorKind::AlreadyExists {
                loop {
                    println!("! Sheet '{}.json' already exists. Overwrite? [Y/N]", sheet_name);
                    let user_input = TrackerCli::user_input()?;

                    match user_input.trim().to_ascii_lowercase().as_str() {
                        "y" => {
                            manager.new_sheet(sheet_name, period, true)
                                .map_err(|e| format!("Failed to create sheet: {}", e))?;
                            break;
                        },
                        "n" => {
                            break;
                        },
                        _ => {
                            println!("! Unsupported input: '{}'", user_input.trim());
                        }
                    }
                }
            } else {
                return Err(format!("Failed to create sheet with error: {}", e))
            }
        } else {
            println!("> Sheet '{}.json' created succesfully.", sheet_name);
        }

        Ok(())
    }

    fn add_sheet_handler(manager: &mut TrackerManager, args: &[&str]) -> Result<(), String> {
        let sheet_type = if args.iter().any(|&x| x == "month") {
            "month"
        } else if args.iter().any(|&x| x == "year") {
            "year"
        } else {
            return Err("ERROR: Unhandled exception. Non-reachable condition.".to_string());
        };

        /* Determine a period */
        /* TODO: Should I invent some separate class to handle the period calculations? */
        let date = Utc::now().date_naive();
        let year = date.year();
        let month = date.month();

        let period= match sheet_type {
            "month" => {
                let period_start = NaiveDate::from_ymd_opt(year, month, 1)
                    .expect("I don't know how to write an architecture for that. Go back there.");
        
                let period_end = if month == 12 {
                    NaiveDate::from_ymd_opt(year + 1, month, 1).unwrap()
                } else {
                    NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
                }.pred_opt().expect("I don't know how to write an architecture for that. Go back later.");
    
                (period_start, period_end)
            },
            "year" => {
                let period_start = NaiveDate::from_ymd_opt(year, 1, 1)
                    .expect("I don't know how to write an architecture for that. Go back there.");
        
                let period_end = if month == 12 {
                    NaiveDate::from_ymd_opt(year + 1, month, 1).unwrap()
                } else {
                    NaiveDate::from_ymd_opt(year, 12, 31).unwrap()
                }.pred_opt().expect("I don't know how to write an architecture for that. Go back later.");
    
                (period_start, period_end)
            },
            _ => unreachable!()
        };

        /* Determine a sheet name */
        let pos = args.iter().position(|&x| x == sheet_type)
            .unwrap();
        let sheet_name = if args.len() > pos + 1 {
            /* Special case: custom sheet name */
            args[pos + 1..].join(" ")
        } else {
            /* Default case. Prepare a sheet name based on its type. */
            match sheet_type {
                "month" => format!("{}-{}", date.month(), date.year()),
                "year" => date.year().to_string(),
                _ => unreachable!()
            }
        };

        Self::create_sheet_with_prompt(manager, &sheet_name, period)
    }
    
    pub fn main_function(&mut self) { /* Without the &self - static method. */
        loop {
            self.buffer.clear();
            self.buffer = TrackerCli::user_input()
                .expect("MODIFY THE RETURN VALUE THERE");

            let tokens: Vec<&str> = self.buffer.split_ascii_whitespace().collect();

            /* Check if given request is 'help' */
            if let Some(help_pos) = tokens.iter().position(|&token| token == "help") {
                let context = &tokens[..help_pos];

                if let Some(cmd_node) = self.cmd_tree.find_command(context) {
                    cmd_node.show_help(context);
                } else {
                    eprintln!("> FAILED: Unknown command: {}", context.join(" "));
                }
            }

            /* Process the regular command */
            if let Some(cmd_node) = self.cmd_tree.find_command(&tokens) {
                if let Some(handler_fn) = cmd_node.handler {
                    handler_fn(&mut self.tracker_manager, &tokens).unwrap();
                }
            } else {
                eprintln!("> FAILED: Unknown command: {}", tokens.join(" "));
            }
        }
    }
}


impl CommandNode {
    pub fn new(cmd: &str, description: &str, handler: Option<fn(&mut TrackerManager, &[&str]) -> Result<(), String>>) -> Self {
        Self {
            cmd: cmd.to_string(),
            description: description.to_string(),
            children: Vec::new(),
            handler: handler,
            extra_args: handler.is_some()
        }
    }

    pub fn add_child(mut self, child: CommandNode) -> Self {
        self.children.push(child);
        self
    }

    pub fn find_command(&self, cmd: &[&str]) -> Option<&CommandNode> {
        /* Base case: the last CommandNode was reached. */
        if cmd.is_empty() {
            return Some(self);
        }

        /* Last node in a chain. Rest of arguments are optional and should not be handled. */
        if self.extra_args {
            return Some(self);
        }

        /* Recursive case: iterate until reaching the last subcommand */
        for child in &self.children {
            if child.cmd == cmd[0] {
                return child.find_command(&cmd[1..]);
            }
        }

        None
    }

    pub fn show_help(&self, context: &[&str]) {
        if context.is_empty() {
            println!("? A budget tracker CLI application.\n\
                ?  Available commands:");
        } else {
            println!("? HELP: {}", context.join(" "));
        }

        println!("?   {}", self.description);
        
        if !self.children.is_empty() {
            println!("? SUBCOMMANDS: ");
            for child in &self.children {
                println!("?   {} - {}", child.cmd, child.description);
            }
        }
    }
}