use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Clone, Hash, Deserialize, Serialize, Debug)]
struct Expenses {
    id: String,
    description: String,
    date: String,
    category: String,
    amount: String,
}

pub fn update(
    id: String,
    description: String,
    date: String,
    category: String,
    amount: String,
) -> Result<(), Box<dyn Error>> {
    let mut expenses = read_to_string("/home/charan/Downloads/expenses.json")?;

    let mut required_expense = Expenses {
        id: String::from(""),
        description: String::from(""),
        date: String::from(""),
        category: String::from(""),
        amount: String::from(""),
    };

    for (_idx, (_key, transaction)) in expenses.iter().enumerate() {
        if transaction.id == id {
            required_expense = transaction.clone();
        }
    }

    if id == "" {
        eprintln!("No ID is given");
    } else {
        if description != "" {
            required_expense.description = description;
        }
        if date != "" {
            required_expense.date = date;
        }
        if amount != "" {
            required_expense.amount = amount;
        }
        if category != "" {
            required_expense.category = category;
        }
    }

    expenses.insert(id, required_expense);
    write_to_file(&expenses).unwrap();

    Ok(())
}

fn read_to_string(path: &str) -> Result<HashMap<String, Expenses>, Box<dyn Error>> {
    let file = fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);

    let u: HashMap<String, Expenses> = serde_json::from_reader(reader)?;

    Ok(u)
}

fn write_to_file(expenses: &HashMap<String, Expenses>) -> Result<(), Box<dyn Error>> {
    let file = fs::File::create("/home/charan/Downloads/expenses.json")?;
    serde_json::to_writer(file, expenses)?;
    Ok(())
}
