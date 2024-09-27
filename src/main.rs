mod add;
mod delete;
mod list;
mod update;
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
    if arguments[1] == "add" {
        if arguments[2] == "--help" || arguments[2] == "-h" {
            println!("Usage: todo add [options]\n");
            println!("Options:");
            println!("  -id, --id <number>               Please enter a number to assign a ID to expense");
            println!("  -desc, --description [words...]  Add a description to the Expense");
            println!("  -a, --amount [number]            Money expended");
            println!("  -date, --date [letters]          Enter the date of expense in yyyy-mm-dd");
            println!("  -c, --category [category]        Assign a category for given Expense");
            println!("  -h, --help                       display help for command");
        } else {
            add::add(id, description, date, category, amount);
        }
    } else if arguments[1] == "listall" {
        let _ = list::listall();
    } else if arguments[1] == "list" {
        if arguments[2] == "--help" || arguments[2] == "-h" {
            println!("Usage: todo list [options]\n");
            println!("Options:");
            println!("  -id, --id <number>               Please enter a number to assign a ID to expense");
            println!("  -h, --help                       display help for command");
        } else {
            let _ = list::listone(id);
        }
    } else if arguments[1] == "update" {
        if arguments[2] == "--help" || arguments[2] == "-h" {
            println!("Usage: todo update [options]\n");
            println!("Options:");
            println!("  -id, --id <number>               Please enter a number to assign a ID to expense");
            println!("  -desc, --description [words...]  Add a description to the Expense");
            println!("  -a, --amount [number]            Money expended");
            println!("  -date, --date [letters]          Enter the date of expense in yyyy-mm-dd");
            println!("  -c, --category [category]        Assign a category for given Expense");
            println!("  -h, --help                       display help for command");
        } else {
            let _ = update::update(id, description, date, category, amount);
        }
    } else if arguments[1] == "delete" {
        let _ = delete::delete();
    } else if arguments[1] == "--help" || arguments[1] == "-h" {
        println!("Usage: expense-tracker [options] [command]\n");
        println!("One Todo-list to track them all\n");
        println!("Options: ");
        println!("  -V, --version      output the version number");
        println!("  -h, --help         display help for command\n");
        println!("Commands: ");
        println!("  add [options]");
        println!("  update [options]");
        println!("  delete [id]");
        println!("  listall");
        println!("  list [options]");
        println!("  help [command]     display help for command");
    }
}
