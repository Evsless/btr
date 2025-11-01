use chrono::{Datelike, Utc};
use std::io::{stdin, stdout, ErrorKind, Write};

use crate::utils::Utils;
use crate::database::manager::TrackerManager;


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
                        Some(Self::add_month_handler))
                    )
                    .add_child(CommandNode::new(
                        "year", 
                        "Add a new year sheet to an expenses database.",
                        Some(Self::add_year_handler))
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
        sheet_path: &str, 
        create_fn: fn(&mut TrackerManager, &str, bool) -> Result<(), std::io::Error>
    ) -> Result<(), String> {
        if let Err(e) = create_fn(manager, sheet_path, false) {
            if e.kind() == ErrorKind::AlreadyExists {
                loop {
                    println!("! Sheet '{}' already exists. Overwrite? [Y/N]", sheet_path);
                    let user_input = TrackerCli::user_input()?;

                    match user_input.trim().to_ascii_lowercase().as_str() {
                        "y" => {
                            create_fn(manager, sheet_path, true)
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
            println!("> Sheet '{}' created succesfully.", sheet_path);
        }

        Ok(())
    }

    fn add_month_handler(manager: &mut TrackerManager, args: &[&str]) -> Result<(), String> {
        if let Some(pos) = args.iter().position(|&x| x == "month") {

            let sheet_name = if args.len() > pos + 1 {
                /* Special case. Sheet name provided by the user. */
                args[pos + 1..].join(" ")
            } else {
                /* Default case. The sheet name is '<month>_<year>' */
                let date = Utc::now().naive_utc();
                format!("{}_{}", date.month(), date.year())
            };
 
            let utils = Utils::new();
            let sheet_path = format!("{}/{}.json", utils.home_dir().display(), sheet_name);
            
            return Self::create_sheet_with_prompt(manager, &sheet_path, |m, p, t| m.month(p, t));
        }
    
        Err("ERROR: Unhandled exception. Non-reachable condition.".to_string())
    }

    fn add_year_handler(manager: &mut TrackerManager, args: &[&str]) -> Result<(), String> {
        if let Some(pos) = args.iter().position(|&x| x == "year" ) {
            let sheet_name = if args.len() > pos + 1 {
                args[pos + 1..].join(" ")
            } else {
                let date = Utc::now().naive_utc();
                format!("y_{}", date.year())
            };

            let utils = Utils::new();
            let sheet_path = format!("{}/{}.json", utils.home_dir().display(), sheet_name);
                
            /* Create a sheet with 'overwrite' handler. */
            return Self::create_sheet_with_prompt(manager, &sheet_path, |m, p, t|m.year(p, t));
        }
        Err("ERROR: Unhandled exception. Non-reachable condition.".to_string())
    }

    pub fn main_function(&mut self) { /* Without the &self - static method. */
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