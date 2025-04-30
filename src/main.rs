use anyhow::Result;
use chrono::NaiveDate;
use rustyline::{ error::ReadlineError, DefaultEditor };
use serde::{ Deserialize, Serialize };
use uuid::Uuid;
use std::{ fs::{ self, File }, io::{ self, Write } };
const HISTORY_PATH: &str = "history.txt";

#[derive(Serialize, Deserialize)]
struct Expense {
    id: Uuid,
    date: NaiveDate,
    category: String,
    amount: u16,
}

fn print_expenses(expenses: &[Expense]) {
    if expenses.is_empty() {
        println!("No Expenses yet");
    } else {
        println!("{:<36} {:<20} {:<10} {:>10}", "Id", "ExpenseName", "Date", "Amount");
        println!("{}", "-".repeat(36 + 1 + 20 + 1 + 10 + 1 + 10));
        for e in expenses {
            println!("{:<36} {:<20} {:<10} {:>10}", e.id, e.category, e.date, e.amount);
        }
        let total: u16 = expenses
            .iter()
            .map(|r| r.amount)
            .sum();
        println!("{}", "-".repeat(36 + 1 + 20 + 1 + 10 + 1 + 10));
        println!("{:<36} {:<20} {:<10} {:>10}", "", "", "Total:", total);
    }
}

fn print_commands_help() {
    println!(
        "Commands:
        1.add <category> <YYYY-MM-DD> <amount>
        2.update <id> <category> <YYYY-MM-DD> <amount>
        3.delete <id>
        4.list
        5.save <file.json>
        6.load <file.json>
        7.total-expense
        8.clear
        9.help
        10.quit"
    );
}

fn clear_screen() -> Result<()> {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush()?;
    Ok(())
}

fn prune_history_file(max: usize) -> io::Result<()> {
    let mut lines: Vec<String> = fs
        ::read_to_string(HISTORY_PATH)?
        .lines()
        .map(str::to_string)
        .collect();

    if lines.len() > max {
        let excess = lines.len() - max;
        lines.drain(0..excess);
        fs::write(HISTORY_PATH, lines.join("\n"))?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut expenses: Vec<Expense> = Vec::new();
    let mut next_id = Uuid::new_v4();

    let mut rl = DefaultEditor::new()?;
    let _ = rl.load_history("history.txt");

    println!("Welcome to the Blue Expense Tracker");
    print_commands_help();
    loop {
        let readline = rl.readline(">> ");
        let line = match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                line
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!();
                break;
            }
            Err(err) => {
                eprintln!("Error reading line: {:?}", err);
                break;
            }
        };

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "clear" => {
                let _ = clear_screen();
            }

            "add" if parts.len() == 4 => {
                let date = NaiveDate::parse_from_str(parts[2], "%Y-%m-%d")?;

                let amount: u16 = parts[3].parse::<f64>()?.round() as u16;

                expenses.push(Expense {
                    id: next_id,
                    date,
                    category: parts[1].to_string(),
                    amount,
                });
                next_id = Uuid::new_v4();
                print_expenses(&expenses);
            }

            "update" => {
                let target = Uuid::parse_str(parts[1]).expect("parts[1] was not valid id");
                if let Some(rec) = expenses.iter_mut().find(|r| r.id == target) {
                    rec.category = parts[2].to_string();
                    rec.date = NaiveDate::parse_from_str(parts[3], "%Y-%m-%d")?;
                    rec.amount = parts[4].parse()?;
                    print_expenses(&expenses);
                } else {
                    println!("No record Found with this id {}", parts[1]);
                }
            }

            "delete" => {
                let target = Uuid::parse_str(parts[1]).expect("parts[1] was not valid id");
                expenses.retain(|r| r.id != target);
                print_expenses(&expenses);
            }

            "list" => {
                print_expenses(&expenses);
            }

            "load" if parts.len() == 2 => {
                let file = File::open(parts[1])?;
                expenses = serde_json::from_reader(file)?;
                next_id = Uuid::new_v4();
                print_expenses(&expenses);
            }

            "save" if parts.len() == 2 => {
                let file: File = File::create(parts[1])?;
                serde_json::to_writer_pretty(file, &expenses)?;
                println!("saved {} expense(s) to {}", expenses.len(), parts[1]);
            }

            "total-expense" => {
                let total_sum = expenses
                    .iter()
                    .map(|r| r.amount)
                    .reduce(|acc, amt| acc + amt)
                    .unwrap_or(0);
                println!("Total Expense : {:?}", total_sum);
            }

            "help" => {
                print_commands_help();
            }

            "quit" => {
                break;
            }

            _ => println!("command not available - type `help`"),
        }
    }
    let _ = rl.save_history("history.txt");
    let _ = prune_history_file(20);
    println!("Goodbye!");
    Ok(())
}
