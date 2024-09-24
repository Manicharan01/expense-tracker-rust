use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Clone, Hash, Deserialize, Serialize, Debug)]
struct Todo {
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
    let mut todos = read_to_string("/home/charan/Downloads/todo.json")?;

    let mut required_todo = Todo {
        id: String::from(""),
        description: String::from(""),
        date: String::from(""),
        category: String::from(""),
        amount: String::from(""),
    };

    for (_idx, (_key, transaction)) in todos.iter().enumerate() {
        if transaction.id == id {
            required_todo = transaction.clone();
        }
    }

    if id == "" {
        eprintln!("No ID is given");
    } else {
        if description != "" {
            required_todo.description = description;
        }
        if date != "" {
            required_todo.date = date;
        }
        if amount != "" {
            required_todo.amount = amount;
        }
        if category != "" {
            required_todo.category = category;
        }
    }

    todos.insert(id, required_todo);
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
