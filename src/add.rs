use serde_json::json;
use std::fs;

pub fn add(id: String, description: String, date: String, category: String, amount: String) {
    let json = json!({
        "id": id,
        "description": description,
        "date": date,
        "category": category,
        "amount": amount
    });

    let result = fs::write("/home/charan/Downloads/todo.json", &json.to_string());

    match result {
        Ok(v) => println!("{v:?}"),
        Err(e) => println!("{e:?}"),
    }
}
