use crate::{
    console::cli::TrackerCli,
    database::{manager::TrackerManager, periods::Period},
    error::{BtrError, BtrErrorKind},
    utils,
};
use chrono::{Datelike, Utc};
use std::io::ErrorKind;

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

pub fn add_expense_handler(_cli: &mut TrackerCli, _args: &[&str]) -> Result<(), BtrError> {
    println!("Enteren add handler!");
    Ok(())
}

pub fn show_sheets_handler(cli: &mut TrackerCli, _args: &[&str]) -> Result<(), BtrError> {
    let active_sheet = cli.tracker_manager.get_active_sheet();

    println!("? SHEETS:");
    for dir_entry in utils::sheets_dir().read_dir()? {
        if let Ok(sheet_path) = dir_entry {
            if let Some(sheet_name) = sheet_path.file_name().to_str() {
                let is_active = active_sheet
                    .as_ref()
                    .map(|s| format!("{}.json", s.name) == sheet_name)
                    .unwrap_or(false);

                if is_active {
                    println!(">  {} (ACTIVE)", sheet_name);
                } else {
                    println!(">  {}", sheet_name);
                }
            }
        }
    }
    Ok(())
}

pub fn add_sheet_handler(cli: &mut TrackerCli, args: &[&str]) -> Result<(), BtrError> {
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

    create_sheet_with_prompt(&mut cli.tracker_manager, &sheet_name, period)
}

pub fn show_categories_handler(cli: &mut TrackerCli, _args: &[&str]) -> Result<(), BtrError> {
    println!("? CATEGORIES:");
    for category in cli.tracker_manager.get_categories() {
        println!(">  {}", category.name);
        if let Some(description) = &category.description {
            println!("   {}", description);
        }
    }

    Ok(())
}

pub fn select_handler(cli: &mut TrackerCli, args: &[&str]) -> Result<(), BtrError> {
    match args.len() {
        2 => cli.tracker_manager.set_active_sheet(&args[1])?,
        _ => {
            eprintln!("! Wrong input. Sheet name must be provided.")
        }
    }

    Ok(())
}
