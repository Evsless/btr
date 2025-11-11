use crate::{
    console::cmd::CommandNode,
    database::{manager::TrackerManager, periods::Period},
    error::{BtrError, BtrErrorKind},
    utils::Utils,
};
use chrono::{Datelike, Utc};
use std::io::{stdin, stdout, ErrorKind, Write};

pub struct TrackerCli {
    buffer: String,
    cmd_tree: CommandNode,
    tracker_manager: TrackerManager,
}

impl TrackerCli {
    pub fn new() -> Self {
        let cmd_tree = CommandNode::new("root", "Budget tracker CLI.", None)
            .add_child(
                CommandNode::new("add", "Add a new record to a budget tracker.", None)
                    .add_child(CommandNode::new(
                        "expense",
                        "Add a new expense record to an active sheet.",
                        Some(Self::add_expense_handler),
                    ))
                    .add_child(CommandNode::new(
                        "month",
                        "Add a new month sheet to an expenses database.",
                        Some(Self::add_sheet_handler),
                    ))
                    .add_child(CommandNode::new(
                        "year",
                        "Add a new year sheet to an expenses database.",
                        Some(Self::add_sheet_handler),
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
                        Some(Self::show_sheets_handler),
                    ))
                    .add_child(CommandNode::new(
                        "categories",
                        "List all available expense categories.",
                        Some(Self::show_categories_handler),
                    )),
            );

        Self {
            buffer: String::new(),
            cmd_tree: cmd_tree,
            tracker_manager: TrackerManager::new(),
        }
    }

    fn add_expense_handler(cli: &mut TrackerCli, args: &[&str]) -> Result<(), BtrError> {
        println!("Enteren add handler!");
        Ok(())
    }

    fn user_input() -> Result<String, BtrError> {
        print!("> ");
        let _ = stdout().flush();

        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?;

        Ok(buffer)
    }

    fn create_sheet_with_prompt(
        manager: &mut TrackerManager,
        sheet_name: &str,
        period: Period,
    ) -> Result<(), BtrError> {
        /* Period is a small data type - simple clone use is enough. */
        if let Err(e) = manager.new_sheet(sheet_name, period.clone(), false) {
            if e.kind() == BtrErrorKind::Io(ErrorKind::AlreadyExists) {
                loop {
                    println!(
                        "! Sheet '{}.json' already exists. Overwrite? [Y/N]",
                        sheet_name
                    );
                    let user_input = TrackerCli::user_input()?;

                    match user_input.trim().to_ascii_lowercase().as_str() {
                        "y" => {
                            manager.new_sheet(sheet_name, period, true)?;
                            break;
                        }
                        "n" => {
                            break;
                        }
                        _ => {
                            println!("! Unsupported input: '{}'", user_input.trim());
                        }
                    }
                }
            } else {
                return Err(e);
            }
        } else {
            println!("> Sheet '{}.json' created succesfully.", sheet_name);
        }

        Ok(())
    }

    fn add_sheet_handler(cli: &mut TrackerCli, args: &[&str]) -> Result<(), BtrError> {
        let sheet_type = if args.iter().any(|&x| x == "month") {
            "month"
        } else if args.iter().any(|&x| x == "year") {
            "year"
        } else {
            return Err(BtrError::InvalidData(Some(String::from(
                "Invalid operation",
            ))));
        };

        /* Determine a period */
        /* TODO: Should I invent some separate class to handle the period calculations? */
        let date = Utc::now().date_naive();

        let period = match sheet_type {
            "month" => Period::current_month()
                .expect("Getting a current month. No option to be outside the month range."),
            "year" => Period::current_year()
                .expect("Getting a current year. No option to hit a negative year."),
            _ => unreachable!(),
        };

        /* Determine a sheet name */
        let pos = args.iter().position(|&x| x == sheet_type).unwrap();
        let sheet_name = if args.len() > pos + 1 {
            /* Special case: custom sheet name */
            args[pos + 1..].join(" ")
        } else {
            /* Default case. Prepare a sheet name based on its type. */
            match sheet_type {
                "month" => format!("{}-{}", date.month(), date.year()),
                "year" => date.year().to_string(),
                _ => unreachable!(),
            }
        };

        Self::create_sheet_with_prompt(&mut cli.tracker_manager, &sheet_name, period)
    }

    fn show_sheets_handler(_cli: &mut TrackerCli, _args: &[&str]) -> Result<(), BtrError> {
        let utils = Utils::new();

        println!("? SHEETS:");
        for dir_entry in utils.sheets_dir().read_dir()? {
            if let Ok(sheet_path) = dir_entry {
                if let Some(sheet_name) = sheet_path.file_name().to_str() {
                    println!(">  {}", sheet_name);
                }
            }
        }
        Ok(())
    }

    fn show_categories_handler(cli: &mut TrackerCli, args: &[&str]) -> Result<(), BtrError> {
        println!("? CATEGORIES:");
        for category in cli.tracker_manager.get_categories() {
            println!(">  {}", category.name);
            if let Some(description) = &category.description {
                println!("   {}", description);
            }
        }

        Ok(())
    }

    pub fn main_function(&mut self) {
        /* Without the &self - static method. */
        loop {
            self.buffer.clear();

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
                    handler_fn(self, &tokens).unwrap();
                }
            } else {
                eprintln!("> FAILED: Unknown command: {}", tokens.join(" "));
            }
        }
    }
}
