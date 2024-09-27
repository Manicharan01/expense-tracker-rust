use colored::Colorize;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Hash, Deserialize, Serialize, Debug)]
struct Expenses {
    id: String,
    description: String,
    date: String,
    category: String,
    amount: String,
}

pub fn listall() -> Result<(), Box<dyn Error>> {
    let expenses = read_to_string("/home/charan/Downloads/expenses.json")?;

    println!(
        "{}, {}, {}, {}, {}",
        "Id".bold().red(),
        "Description".bold().red(),
        "Amount".bold().red(),
        "Category".bold().red(),
        "Date".bold().red()
    );

    for (_index, (_key, transaction)) in expenses.iter().enumerate() {
        println!(
            "{}, {}, {}, {}, {}",
            transaction.id.green(),
            transaction.description.green(),
            transaction.amount.green(),
            transaction.category.green(),
            transaction.date.green()
        );
    }

    Ok(())
}

pub fn listone(id: String) -> Result<(), Box<dyn Error>> {
    let expenses = read_to_string("/home/charan/Downloads/expenses.json")?;

    println!(
        "{}, {}, {}, {}, {}",
        "Id".bold().red(),
        "Description".bold().red(),
        "Amount".bold().red(),
        "Category".bold().red(),
        "Date".bold().red()
    );

    for (_index, (_key, transaction)) in expenses.iter().enumerate() {
        if id == transaction.id {
            println!(
                "{}, {}, {}, {}, {}",
                transaction.id.green(),
                transaction.description.green(),
                transaction.amount.green(),
                transaction.category.green(),
                transaction.date.green()
            );
        }
    }

    Ok(())
}

fn read_to_string(path: &str) -> Result<HashMap<String, Expenses>, Box<dyn Error>> {
    let file = fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);

    let u: HashMap<String, Expenses> = serde_json::from_reader(reader)?;

    Ok(u)
}
