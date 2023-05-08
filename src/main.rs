use std::process::exit;

use clap::{Parser, Subcommand};
use regex::Regex;

mod todolist;
use crate::todolist::ListFile;

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
    /// Initialize directory to contain todolists
    Init,
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
    /// Shift focus to provided list
    Focus {
        /// Name of list to focus
        name: String,
    },
    /// List of existing todolists
    List,
    /// List of existing todolists
    Ls,

    /// Lists tasks within focused todolist
    Tasks,
    /// Lists tasks within focused todolist
    Ts,
    /// Add a task to the focused todolist
    Add {
        /// Task(s) as strings to add to the focused list
        #[arg(required=true)]
        task: Vec<String>,
    },
    /// Drops given task(s) from the focused todolist
    Drop {
        /// Index(ices) of task(s) to delete from the focused list
        #[arg(required=true)]
        index: Vec<usize>,
    },
    /// Marks given task(s) as complete
    Done {
        /// Index(ices) of task(s) to mark as complete
        #[arg(required=true)]
        index: Vec<usize>,
    },
    /// Marks given task(s) as incomplete
    Undo {
        /// Index(ices) of task(s) to mark as incomplete
        #[arg(required=true)]
        index: Vec<usize>,
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


 fn ensure_valid_list_name(name: &String) {
    // create regex for list name validation
    let regex = Regex::new(r"^[a-zA-Z0-9]([a-zA-Z0-9-_]*[a-zA-Z0-9])?$").unwrap();

    // compare against regexp
    if !regex.is_match(&name) {
        println!("Invalid todolist name '{}'", name);
        println!("Todolist names must start and end with a letter or number, and may only contain only letters, numbers, hyphens, and underscores");
        exit(1);
    }
}

fn main() {
    let user_home = home::home_dir().expect("Unable to locate your home directory");
    let todolists_path = format!("{}/.todolists", user_home.display());

    match CLI::parse().command {
        Command::Init => {
            // check if already initialized
            if std::path::Path::new(&todolists_path).exists() {
                println!("Todolist has already been initialized");
                exit(1);
            }

            // create new list file
            let list_file = ListFile::new();
            // serialize with bincode
            let encoded = bincode::serialize(&list_file).unwrap();
            // write to file
            std::fs::write(todolists_path, encoded).unwrap();
            println!("Todolists initialized");
        },

        Command::Create { name } => {
            ensure_valid_list_name(&name);

            // read in todolist file
            let mut list_file = ListFile::from_file(&todolists_path);

            // create new list representation
            list_file.add_list(name);
            
            // write todolist file
            list_file.to_file(&todolists_path);
        },
        
        Command::Delete { name } => {
            ensure_valid_list_name(&name);
            
            // read in todolist file
            let mut list_file = ListFile::from_file(&todolists_path);
            
            // delete desired list
            list_file.delete_list(name);
            
            // write todolist file
            list_file.to_file(&todolists_path);
        },

        Command::Focus { name } => {
            ensure_valid_list_name(&name);

            // read in todolist file
            let mut list_file = ListFile::from_file(&todolists_path);

            // shift focus
            list_file.shift_focus(name);
            
            // write todolist file
            list_file.to_file(&todolists_path);
        }
        
        Command::List | Command::Ls => {
            // read in todolist file
            let list_file = ListFile::from_file(&todolists_path);
            
            // collect list names
            let mut names: Vec<&String> = list_file.lists.keys().collect();
            // sort list names
            names.sort();

            // check if there are any lists
            if names.len() == 0 {
                println!("You have no lists");
                exit(1);
            }

            let focus = list_file.focused.unwrap();

            for n in names {
                if n == &focus {
                    println!("* {n} *");
                } else {
                    println!("  {n}");
                }
            }
        }



        Command::Tasks | Command::Ts => {
            // read in todolist file
            let mut list_file = ListFile::from_file(&todolists_path);

            // access focused todolist
            let list = list_file.get_focused();

            // print tasks
            let mut i = 1;
            println!("-- {} --", list.name);
            for t in &list.tasks {
                println!("{i}| {t}");
                i += 1;
            }
        },
        
        Command::Add { task } => {
            // read in todolist file
            let mut list_file = ListFile::from_file(&todolists_path);
            
            // access focused todolist
            let list = list_file.get_focused();
            
            // add new task
            list.add_tasks(task);
            
            // write todolist file
            list_file.to_file(&todolists_path);
        },

        Command::Drop { index } => {
            // read in todolist file
            let mut list_file = ListFile::from_file(&todolists_path);

            // access focused todolist
            let list = list_file.get_focused();

            // drop tasks
            list.drop_tasks(index);

            // write todolist file
            list_file.to_file(&todolists_path);
        },
        
        Command::Done { index } => {
            // read in todolist file
            let mut list_file = ListFile::from_file(&todolists_path);

            // access focused todolist
            let list = list_file.get_focused();

            // mark tasks as done
            list.update_completions(index, true);

            // write todolist file
            list_file.to_file(&todolists_path);
        },
        Command::Undo { index } => {
            // read in todolist file
            let mut list_file = ListFile::from_file(&todolists_path);

            // access focused todolist
            let list = list_file.get_focused();

            // mark tasks as undone
            list.update_completions(index, false);

            // write todolist file
            list_file.to_file(&todolists_path);
        }
    }
}
