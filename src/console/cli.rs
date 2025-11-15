use crate::{
    console::{cmd::CommandNode, handlers},
    database::manager::TrackerManager,
    error::BtrError,
};

use std::io::{Write, stdin, stdout};

pub struct TrackerCli {
    cmd_tree: CommandNode,
    pub tracker_manager: TrackerManager,
}

impl TrackerCli {
    pub fn new() -> Result<Self, BtrError> {
        let cmd_tree = CommandNode::new("root", "Budget tracker CLI.", None)
            .add_child(
                CommandNode::new("add", "Add a new record to a budget tracker.", None)
                    .add_child(CommandNode::new(
                        "expense",
                        "Add a new expense record to an active sheet.",
                        Some(handlers::add_expense_handler),
                    ))
                    .add_child(CommandNode::new(
                        "month",
                        "Add a new month sheet to an expenses database.",
                        Some(handlers::add_sheet_handler),
                    ))
                    .add_child(CommandNode::new(
                        "year",
                        "Add a new year sheet to an expenses database.",
                        Some(handlers::add_sheet_handler),
                    )),
            )
            .add_child(CommandNode::new(
                "remove",
                "Remove a record from an expenses database.",
                None,
            ))
            .add_child(CommandNode::new(
                "modify",
                "Modify an existing record in an expenses database",
                None,
            ))
            .add_child(
                CommandNode::new("show", "Print a configuration of selected category.", None)
                    .add_child(CommandNode::new(
                        "sheets",
                        "Print a list of all available expense sheets.",
                        Some(handlers::show_sheets_handler),
                    ))
                    .add_child(CommandNode::new(
                        "categories",
                        "List all available expense categories.",
                        Some(handlers::show_categories_handler),
                    )),
            )
            .add_child(CommandNode::new(
                "select",
                "Select an active sheet which will be updated with a new expenses logs.",
                Some(handlers::select_handler),
            ));

        Ok(Self {
            cmd_tree: cmd_tree,
            tracker_manager: TrackerManager::new()?,
        })
    }

    pub fn user_input() -> Result<String, BtrError> {
        print!("> ");
        let _ = stdout().flush();

        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?;

        Ok(buffer)
    }

    pub fn main_function(&mut self) {
        loop {
            let buffer = TrackerCli::user_input().expect("MODIFY THE RETURN VALUE THERE");

            let tokens: Vec<&str> = buffer.split_ascii_whitespace().collect();

            /* Check if given request is 'help' */
            if let Some(help_pos) = tokens.iter().position(|&token| token == "help") {
                let context = &tokens[..help_pos];

                if let Some(cmd_node) = self.cmd_tree.find_command(context) {
                    cmd_node.show_help(context);
                } else {
                    eprintln!("> FAILED: Unknown command: {}", context.join(" "));
                }
            } else if let Some(cmd_node) = self.cmd_tree.find_command(&tokens) {
                if let Some(handler_fn) = cmd_node.handler {
                    match handler_fn(self, &tokens) {
                        Err(e) => {
                            eprintln!("! Operation finished with an error:\n!   {}", e);
                        }
                        _ => {}
                    }
                }
            } else {
                eprintln!("> FAILED: Unknown command: {}", tokens.join(" "));
            }
        }
    }
}
