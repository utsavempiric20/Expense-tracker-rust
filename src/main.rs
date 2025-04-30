use anyhow::{ Ok, Result };
use chrono::NaiveDate;
use serde::{ Deserialize, Serialize };
use uuid::Uuid;
use std::{ fs::File, io::{ self, Write } };

#[derive(Serialize, Deserialize, Debug)]
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
        for e in expenses {
            println!("{:<36} {:<20} {:<10} {:>10}", e.id, e.category, e.date, e.amount);
        }
    }
}

fn clear_screen() -> Result<()> {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush()?;
    Ok(())
}

fn main() -> Result<()> {
    let mut expenses: Vec<Expense> = Vec::new();
    let mut next_id = Uuid::new_v4();

    println!("Welcome to the Blue Expense Tracker");

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        let line: Vec<&str> = input.trim().split_whitespace().collect();
        if line.is_empty() {
            continue;
        }

        match line[0] {
            "clear" => {
                let _ = clear_screen();
            }

            "add" if line.len() == 4 => {
                let date = NaiveDate::parse_from_str(line[2], "%Y-%m-%d")?;

                let amount: u16 = line[3].parse::<f64>()?.round() as u16;

                expenses.push(Expense {
                    id: next_id,
                    date,
                    category: line[1].to_string(),
                    amount,
                });
                next_id = Uuid::new_v4();
                print_expenses(&expenses);
            }

            "update" => {
                let target = Uuid::parse_str(line[1]).expect("line[1] was not valid id");
                if let Some(rec) = expenses.iter_mut().find(|r| r.id == target) {
                    rec.category = line[2].to_string();
                    rec.date = NaiveDate::parse_from_str(line[3], "%Y-%m-%d")?;
                    rec.amount = line[4].parse()?;
                    print_expenses(&expenses);
                } else {
                    println!("No record Found with this id {}", line[1]);
                }
            }

            "delete" => {
                let target = Uuid::parse_str(line[1]).expect("line[1] was not valid id");
                expenses.retain(|r| r.id != target);
                print_expenses(&expenses);
            }

            "list" => {
                print_expenses(&expenses);
            }

            "load" if line.len() == 2 => {
                let file = File::open(line[1])?;
                expenses = serde_json::from_reader(file)?;
                next_id = Uuid::new_v4();
                print_expenses(&expenses);
            }

            "save" if line.len() == 2 => {
                let file: File = File::create(line[1])?;
                serde_json::to_writer_pretty(file, &expenses)?;
                println!("saved {} expense(s) to {}", expenses.len(), line[1]);
            }

            "help" =>
                println!(
                    "Commands:\n  add <category> <YYYY-MM-DD> <amount>\n  list\n  save <file.json>\n  update <id> <category> <YYYY-MM-DD> <amount>\n  delete <id>\n  quit"
                ),

            "quit" => {
                break;
            }

            _ => println!("command not available - type `help`"),
        }
    }
    Ok(())
}
