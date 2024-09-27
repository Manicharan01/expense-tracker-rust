use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Serialize, Debug)]
struct Expenses {
    id: String,
    description: String,
    date: String,
    category: String,
    amount: String,
}

pub fn add(id: String, description: String, date: String, category: String, amount: String) {
    let mut expenses = read_to_string("/home/charan/Downloads/expenses.json").unwrap();

    let new_expense = Expenses {
        date: date.clone(),
        description: description.clone(),
        id: id.clone(),
        category: category.clone(),
        amount: amount.clone(),
    };

    expenses.insert(id, new_expense);
    write_to_file(&expenses).unwrap();
}

fn read_to_string<P: AsRef<Path>>(path: P) -> Result<HashMap<String, Expenses>, Box<dyn Error>> {
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
