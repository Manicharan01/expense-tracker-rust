mod add;
use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let mut id: String = String::from("");
    let mut description: String = String::from("");
    let mut date: String = String::from("");
    let mut category: String = String::from("");
    let mut amount: String = String::from("");

    for i in 0..arguments.len() {
        let content = match arguments[i].parse::<String>() {
            Ok(arg) => arg,
            Err(_) => {
                println!("Something bad happened");
                return;
            }
        };

        if content == "--id" {
            id = arguments[i + 1].clone();
        }
        if content == "--desc" {
            description = arguments[i + 1].clone();
        }
        if content == "--date" {
            date = arguments[i + 1].clone();
        }
        if content == "--category" {
            category = arguments[i + 1].clone();
        }
        if content == "--amount" {
            amount = arguments[i + 1].clone();
        }
    }

    add::add(id, description, date, category, amount);
}
