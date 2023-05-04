use clap::{Parser, Subcommand};

mod todolist;
use crate::todolist::TodoList;

#[derive(Debug, Clone)]
#[derive(Parser)]
#[command(name="Todo", author, version, about)]
struct CLI {
    #[command(subcommand)]
    command: Command,
}


#[derive(Debug, Clone)]
#[derive(Subcommand)]
enum Command {
    /// Create new list
    Create {
        /// Name of the new list
        name: String,
    },
    /// Delete existing list
    Delete {
        /// Name of list to delete
        name: String,
    },
}


/*
 -- store all lists in one file in current directory
 -- track current (focused) list

todo create school
todo list
todo focus school
todo add 'proj 1'
todo add 'task1 for this class' 'task2 for other class'
 */




fn main() {
    let cli: CLI = CLI::parse();

    match cli.command {
        Command::Create { name } => {
            // check directory for existing list
            if std::path::Path::new(&format!("{}.json", name)).exists() {
                println!("Todolist '{}' already exists", name);
                return;
            }

            // create new list representation
            let list = TodoList::new(name.clone());
            // serialize to json
            let json = serde_json::to_string(&list).unwrap();
            // write to file
            std::fs::write(format!("{}.json", name), json).unwrap();
            println!("Created todolist '{}'", name);
        },
        Command::Delete { name } => {
            // check if file exists
            if !std::path::Path::new(&format!("{}.json", name)).exists() {
                println!("No todolist named '{}' found", name);
                return;
            }
            // delete file
            std::fs::remove_file(format!("{}.json", name)).unwrap();
            println!("Deleted todolist '{}'", name);
        },
    }
}
