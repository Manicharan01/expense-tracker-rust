use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Serialize, Debug)]
struct Todo {
    id: String,
    description: String,
    date: String,
    category: String,
    amount: String,
}

pub fn add(id: String, description: String, date: String, category: String, amount: String) {
    let mut todos = read_to_string("/home/charan/Downloads/todo.json").unwrap();

    let new_todo = Todo {
        date: date.clone(),
        description: description.clone(),
        id: id.clone(),
        category: category.clone(),
        amount: amount.clone(),
    };

    todos.insert(id, new_todo);
    write_to_file(&todos).unwrap();
}

fn read_to_string<P: AsRef<Path>>(path: P) -> Result<HashMap<String, Todo>, Box<dyn Error>> {
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
