use crate::{
    console::{cmd::CommandNode, handlers},
    database::manager::TrackerManager,
    error::BtrError,
};

use std::io::{Write, stdout};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};

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
                        "sheet",
                        "Add a new expense sheet to an tracker database.",
                        Some(handlers::add_sheet_handler),
                    )),
            )
            .add_child(
                CommandNode::new(
                    "delete",
                    "Removes provided element from the tracker database.",
                    None,
                )
                .add_child(CommandNode::new(
                    "sheet",
                    "Removes selected expense sheet from the tracker database",
                    Some(handlers::delete_sheet_handler),
                ))
                .add_child(CommandNode::new(
                    "expense",
                    "Removes selected expense record from the active sheet",
                    Some(handlers::delete_expense_handler),
                )),
            )
            .add_child(CommandNode::new(
                "modify",
                "Modify an existing record in an expenses database",
                None,
            ))
            .add_child(
                CommandNode::new("show", "Print a configuration of selected category.", None)
                    .add_child(CommandNode::new(
                        "expenses",
                        "Print expenses from the active month.",
                        Some(handlers::show_expenses_handler),
                    ))
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

    pub fn user_input(&self) -> Result<String, BtrError> {
        enable_raw_mode()?;

        print!("> ");
        let _ = stdout().flush();

        let mut buffer = String::new();
        let cmd_history = self.tracker_manager.get_cmd_history();
        let mut history_idx: Option<usize> = None;

        loop {
            if event::poll(std::time::Duration::from_millis(500))? {
                if let Event::Key(KeyEvent {
                    code, modifiers, ..
                }) = event::read()?
                {
                    match code {
                        KeyCode::Enter => {
                            print!("\r\n");
                            let _ = stdout().flush();
                            break;
                        }

                        KeyCode::Char(c) => {
                            buffer.push(c);
                            print!("{}", c);
                            let _ = stdout().flush();
                        }

                        KeyCode::Backspace => {
                            if !buffer.is_empty() {
                                buffer.pop();
                                print!("\x08 \x08"); /* Shift back by 1 character, clear 
                                the characted and switch back again */
                                let _ = stdout().flush();
                            }
                        }

                        KeyCode::Up => {
                            if !cmd_history.is_empty() {
                                let idx = match history_idx {
                                    None => cmd_history.len() - 1,
                                    Some(0) => 0,
                                    Some(i) => i - 1,
                                };
                                history_idx = Some(idx);
                                buffer = cmd_history[idx].clone();

                                print!("\r> {}\x1B[K", buffer);
                                let _ = stdout().flush();
                            }
                        }

                        KeyCode::Down => {
                            if let Some(idx) = history_idx {
                                if idx + 1 < cmd_history.len() {
                                    history_idx = Some(idx + 1);
                                    buffer = cmd_history[idx + 1].clone();
                                } else {
                                    history_idx = None;
                                    buffer.clear();
                                }
                            }

                            print!("\r> {}\x1B[K", buffer);
                            let _ = stdout().flush();
                        }
                        /* Temporary solution */
                        KeyCode::Esc => {
                            disable_raw_mode()?;
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                }
            }
        }

        disable_raw_mode()?;

        Ok(buffer)
    }

    pub fn main_function(&mut self) {
        loop {
            let buffer = match self.user_input() {
                Ok(input) => input,
                Err(e) => {
                    eprintln!("!> Input error: {}", e);
                    break;
                }
            };

            if !buffer.trim().is_empty() {
                let _ = self.tracker_manager.config_mut().update_state(|state| {
                    state.add_cmd_to_history(buffer.clone());
                });
            }

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
