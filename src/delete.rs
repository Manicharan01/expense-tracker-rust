use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{self, Write};

#[derive(Clone, Hash, Deserialize, Serialize, Debug)]
struct Todo {
    id: String,
    description: String,
    date: String,
    category: String,
    amount: String,
}

pub fn delete() -> Result<(), Box<dyn Error>> {
    let mut todos = read_to_string("/home/charan/Downloads/todo.json")?;
    let mut keys: Vec<String> = todos.keys().cloned().collect();
    keys.sort_by(|a, b| {
        a.parse::<usize>()
            .unwrap_or(0)
            .cmp(&b.parse::<usize>().unwrap_or(0))
    });

    println!("Available transactions:");
    for (i, key) in keys.iter().enumerate() {
        if let Some(todo) = todos.get(key) {
            println!("{}: {} - {}", i + 1, key, todo.description);
        }
    }

    let mut input = String::new();
    print!("Enter the index of the transaction to remove: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;

    let idx: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Input");
            return Ok(());
        }
    };
    println!("{:?}", keys);

    if let Some(key) = keys.get(idx - 1) {
        println!("Removing the entry with {} from list", key);
        todos.remove(key);
    } else {
        println!("Index out of bounds");
    }

    write_to_file(&todos).unwrap();

    Ok(())
}

fn read_to_string(path: &str) -> Result<HashMap<String, Todo>, Box<dyn Error>> {
    let file = fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);

    let u: HashMap<String, Todo> = serde_json::from_reader(reader)?;

    Ok(u)
}

fn write_to_file(todos: &HashMap<String, Todo>) -> Result<(), Box<dyn Error>> {
    let file = fs::File::create("/home/charan/Downloads/todo.json")?;
    serde_json::to_writer(file, todos)?;
    Ok(())
}
