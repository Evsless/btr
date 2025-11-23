use crate::{
    console::cli::TrackerCli,
    database::{
        expense::{self, ExpenseRecord, ExpenseSheet},
        manager::TrackerManager,
        periods::Period,
    },
    error::{BtrError, BtrErrorKind},
    utils,
};
use chrono::{Datelike, Utc};
use std::{collections::HashMap, fs, io::ErrorKind, thread::AccessError};

fn get_sheet_list() -> Result<Vec<String>, BtrError> {
    let entries = utils::sheets_dir().read_dir()?;

    let sheet_list: Vec<String> = entries
        .filter_map(|e| {
            let path = e.ok()?.path();
            if path.is_file() {
                path.file_stem()
                    .and_then(|f| f.to_str().map(|s| s.to_owned()))
            } else {
                None
            }
        })
        .collect();

    Ok(sheet_list)
}

fn print_sheet_list(active_sheet: &Option<ExpenseSheet>) -> Result<(), BtrError> {
    let sheet_list = get_sheet_list()?;

    for (idx, sheet_name) in sheet_list.iter().enumerate() {
        let is_active = active_sheet
            .as_ref()
            .map(|s| s.name == *sheet_name)
            .unwrap_or(false);

        if is_active {
            println!(">  {}. {:<20} (ACTIVE)", idx, sheet_name);
        } else {
            println!(">  {}. {}", idx, sheet_name);
        }
    }

    Ok(())
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
                    "!> Sheet '{}.json' already exists. Overwrite? [Y/N]",
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
                        println!("!> Unsupported input: '{}'", user_input.trim());
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

/* ---------------------- ADD HANDLERS ---------------------- */
pub fn add_expense_handler(cli: &mut TrackerCli, _args: &[&str]) -> Result<(), BtrError> {
    let categories = cli.tracker_manager.get_categories();

    println!("!> Select a category:");
    for (idx, category) in categories.iter().enumerate() {
        println!("> {}: {}", idx + 1, category.name);
    }

    let category_idx = loop {
        let input = TrackerCli::user_input()?;
        match input.trim().parse::<usize>() {
            Ok(idx) if idx >= 1 && idx <= categories.len() => break idx - 1,
            _ => println!(
                "! Invalid input. Select a number between 1 and {}.",
                categories.len()
            ),
        }
    };

    println!("!> Enter amount:");
    let amount = loop {
        let input = TrackerCli::user_input()?;
        match input.trim().parse::<f32>() {
            Ok(num) if num > 0.0 => break num,
            _ => println!("! Invalid input. The value must be greated then 0."),
        }
    };

    /* Use the clone() for now. However you might consider a better solution there. */
    let new_expense = ExpenseRecord::new(
        categories[category_idx].name.clone(),
        amount,
        Utc::now().date_naive(),
    );

    cli.tracker_manager.update_active_sheet(|sheet| {
        sheet.expenses_mut().push(new_expense);
    })?;

    println!("!> Expense added!");

    Ok(())
}

pub fn add_sheet_handler(cli: &mut TrackerCli, args: &[&str]) -> Result<(), BtrError> {
    /* Determine a period */
    let date = Utc::now().date_naive();

    let period = Period::current_month()?;

    /* Determine a sheet name */
    let sheet_name = if args.len() > 2 {
        /* Special case: custom sheet name */
        args[2..].join(" ")
    } else {
        /* Default case. */
        format!("{:02}-{}", date.month(), date.year())
    };

    create_sheet_with_prompt(&mut cli.tracker_manager, &sheet_name, period)
}

/* ---------------------- SHOW HANDLERS ---------------------- */
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

pub fn show_expenses_handler(cli: &mut TrackerCli, _args: &[&str]) -> Result<(), BtrError> {
    let Some(active_sheet) = cli.tracker_manager.get_active_sheet() else {
        return Err(BtrError::ActiveSheetNotSelected);
    };

    #[derive(Default)]
    struct CategoryStats {
        total: f32,
        count: usize,
    }

    let mut stats: HashMap<&str, CategoryStats> = HashMap::new();
    let mut grand_total = 0.0;

    for expense in active_sheet.expenses() {
        let category_stat = stats.entry(expense.category()).or_default();

        category_stat.total += expense.amount();
        category_stat.count += 1;

        grand_total += expense.amount();
    }

    println!("\n{:<22} {}", "EXPENSES SUMMARY FOR", active_sheet.name);
    println!(
        "{:<22} {} - {}",
        "PERIOD",
        active_sheet.period.start(),
        active_sheet.period.end()
    );
    println!("{}\n", "-".repeat(60));

    println!(
        "{:<20} {:>12} {:>8}  {:>12}",
        "Category", "Total", "Count", "Part Of Total"
    );
    println!("{}", "-".repeat(60));

    let mut sorted = stats.iter().collect::<Vec<_>>();
    sorted.sort_by(|&(_cat_a, stat_a), &(_cat_b, stat_b)| stat_b.total.total_cmp(&stat_a.total));

    for (category, stat) in sorted {
        let part_of_total = (stat.total / grand_total) * 100.0;
        println!(
            "{:<20} {:>9.2} PLN {:>8}  {:>10.1}%",
            category, stat.total, stat.count, part_of_total
        );
    }
    println!("{}", "-".repeat(60));
    println!("{:<20} {:>9.2} PLN", "TOTAL", grand_total);

    Ok(())
}

pub fn show_sheets_handler(cli: &mut TrackerCli, _args: &[&str]) -> Result<(), BtrError> {
    let active_sheet = cli.tracker_manager.get_active_sheet();

    println!("? SHEETS:");
    print_sheet_list(&active_sheet)?;

    Ok(())
}

pub fn select_handler(cli: &mut TrackerCli, args: &[&str]) -> Result<(), BtrError> {
    match args.len() {
        3 => cli.tracker_manager.set_active_sheet(Some(&args[2]))?,
        _ => {
            eprintln!("! Wrong input. Sheet name must be provided.")
        }
    }

    Ok(())
}

/* ---------------------- DELETE HANDLERS ---------------------- */
pub fn delete_sheet_handler(cli: &mut TrackerCli, _args: &[&str]) -> Result<(), BtrError> {
    let active_sheet = cli.tracker_manager.get_active_sheet();

    println!("?> Select a sheet to be deleted:");
    print_sheet_list(active_sheet)?;

    let sheet_list = get_sheet_list()?;
    let choise = loop {
        let user_input = TrackerCli::user_input()?;

        match user_input.trim().parse::<usize>() {
            Ok(idx) if idx < sheet_list.len() => break idx,
            _ => {
                println!(
                    "!> Invalid input. Enter a number in range from 0 to {}.",
                    sheet_list.len()
                )
            }
        }
    };

    /* TO DO: Handle a case when the sheet was active. */
    if let Some(sheet) = active_sheet {
        if sheet.name == sheet_list[choise] {
            cli.tracker_manager.set_active_sheet(None)?
        }
    }

    fs::remove_file(utils::sheets_dir().join(format!("{}.json", sheet_list[choise])))?;
    println!(
        "!> Sheet {} has been sucessfully removed.",
        sheet_list[choise]
    );

    Ok(())
}

pub fn delete_expense_handler(cli: &mut TrackerCli, _args: &[&str]) -> Result<(), BtrError> {
    let Some(active_sheet) = cli.tracker_manager.get_active_sheet() else {
        return Err(BtrError::ActiveSheetNotSelected);
    };

    println!("?> Select an expense to be deleted:");
    for (idx, expense) in active_sheet.expenses().iter().enumerate() {
        println!(
            "> {}. {:<20} {:<8} {}",
            idx,
            expense.category(),
            expense.amount(),
            expense.logged_on()
        );
    }

    let choise = loop {
        let user_input = TrackerCli::user_input()?;

        match user_input.trim().parse::<usize>() {
            Ok(input) => break input,
            _ => {
                println!(
                    "> Invalid input. Enter a number in range from 0 to {}",
                    active_sheet.expenses().len()
                )
            }
        }
    };

    cli.tracker_manager.update_active_sheet(|sheet| {
        sheet.expenses_mut().remove(choise);
    })?;
    println!("!> Expense has been sucesfully removed.");

    Ok(())
}
