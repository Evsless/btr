use std::io::{stdin, stdout, Write};
use crate::database::manager::TrackerManager;



pub struct TrackerCli {
    buffer: String,
    cmd_tree: CommandNode,
    db_manager: TrackerManager
}


pub struct CommandNode {
    cmd: String,
    description: String,
    children: Vec<CommandNode>,
    handler: Option<fn(&[&str]) -> Result<(), String>>

}


impl TrackerCli {
    pub fn new() -> Self {
        let cmd_tree = 
            CommandNode::new("root", "Budget tracker CLI.")
            .add_child(
                CommandNode::new("add", "Add a new record to a budget tracker.")
                    .add_child(CommandNode::new("expense", "Add a new expense record to an active sheet."))
                    .add_child(CommandNode::new("month", "Add a new month sheet to an expenses database."))
                    .add_child(CommandNode::new("year", "Add a new year sheet to an expenses database."))
            )
            .add_child(CommandNode::new("remove", "Remove a record from an expenses database."))
            .add_child(CommandNode::new("modify", "Modify an existing record in an expenses database"));

        Self { 
            buffer: String::new(), 
            cmd_tree: cmd_tree,
            db_manager: TrackerManager::new() }
    }

    pub fn main_function(&mut self) { /* Without the &self - static method. */
        print!("> ");

        if let Err(e) = stdout().flush() {
            eprintln!("> Failed to flush the stdout: {}", e);
        }

        self.buffer.clear();
        stdin().read_line(&mut self.buffer).unwrap();

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
    }
}


impl CommandNode {
    pub fn new(cmd: &str, description: &str) -> Self {
        Self {
            cmd: cmd.to_string(),
            description: description.to_string(),
            children: Vec::new(),
            handler: None
        }
    }

    pub fn add_child(mut self, child: CommandNode) -> Self {
        self.children.push(child);
        self
    }

    pub fn find_command(&self, cmd: &[&str]) -> Option<&CommandNode> {
        /* Base case: the last CommandNode was reached */
        if cmd.is_empty() {
            return Some(self);
        };

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