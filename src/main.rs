use clap::{Parser, Subcommand};
use regex::Regex;
use utils::date::Date;

mod utils;
use crate::utils::{todolist::ListFile, date::parse_date};

#[derive(Debug, Clone, Parser)]
#[command(name="Todo", author="Ayhan Eyikan", version, about)]
struct CLI {
    #[command(subcommand)]
    command: Command,
}


#[derive(Debug, Clone, Subcommand)]
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
    Tasks {
        /// List tasks from all todolists
        #[arg(short, long)]
        all: bool,
    },
    /// Lists tasks within focused todolist
    Ts {
        /// List tasks from all todolists
        #[arg(short, long)]
        all: bool,
    },
    /// Add a task to the focused todolist
    Add {
        /// Task(s) as strings to add to the focused list
        #[arg(required=true)]
        task: Vec<String>,

        /// Task(s) due date
        #[arg(short, long, value_parser = parse_date)]
        date: Option<Date>,
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

    MINI DEV LIST

    - interesting discussion of autocompletions
        https://kbknapp.dev/shell-completions/


 */

fn ensure_valid_list_name(name: &String) {
    // create regex for list name validation
    let regex = Regex::new(r"^[a-zA-Z0-9]([a-zA-Z0-9-_]*[a-zA-Z0-9])?$").unwrap();

    // compare against regexp
    if !regex.is_match(&name) {
        println!("Invalid todolist name: '{}'", name);
        println!("Todolist names must start and end with a letter or number, and may only contain only letters, numbers, hyphens, and underscores");
        std::process::exit(1);
    }
}

fn main() {
    // attempt to retrieve a path to the todolists file within the user's home directory
    let todolists_path = match home::home_dir() {
        Some(home_path) if !home_path.as_os_str().is_empty() => format!("{}/.todolists", home_path.display()),
        _ => {
            eprintln!("Error: could not locate your home directory");
            std::process::exit(1)
        }
    };

    // initialize .todolists file if it doesn't already exist
    if !std::path::Path::new(&todolists_path).exists() {
        ListFile::new().to_file(&todolists_path);
    }

    const NO_LISTS_MSG: &str = "You have no lists, use `todo create <list-name>` to create one.";

    //
    // parse user command passed in

    match CLI::parse().command {
        //
        // LIST_FILE COMMANDS
        //
        Command::Create { name } => {
            ensure_valid_list_name(&name);

            // read in listfile
            let mut list_file = ListFile::from_file(&todolists_path);

            // attempt to create list
            match list_file.create_list(&name) {
                Ok(_) => println!("Created todolist '{}'", name),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1)
                }
            }

            // write todolist file
            list_file.to_file(&todolists_path);
        },

        Command::Delete { name } => {
            ensure_valid_list_name(&name);

            // read in listfile
            let mut list_file = ListFile::from_file(&todolists_path);

            // delete desired list
            match list_file.delete_list(&name) {
                Ok(_) => println!("Successfully deleted '{}'", name),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1)
                }
            }

            // write todolist file
            list_file.to_file(&todolists_path);
        },

        Command::Focus { name } => {
            ensure_valid_list_name(&name);

            // read in listfile
            let mut list_file = ListFile::from_file(&todolists_path);

            // shift focus
            if let Err(e) = list_file.shift_focus(&name) {
                eprintln!("Error: {}", e);
                std::process::exit(1)
            }

            // write todolist file
            list_file.to_file(&todolists_path);
        },

        Command::List | Command::Ls => {
            // read in listfile
            let list_file = ListFile::from_file(&todolists_path);

            // confirm there is at least one list
            let mut names: Vec<&String> = list_file.get_list_names();
            if names.is_empty() {
                eprintln!("{}", NO_LISTS_MSG);
                std::process::exit(1);
            }

            // retrieve focused list name
            let focus = list_file.focused.as_ref().unwrap();

            // print lists in alphabetical order
            names.sort();
            for n in names {
                if n == focus {
                    println!("* {n} *");
                } else {
                    println!("  {n}");
                }
            }
        },

        //
        // list commands
        //
        Command::Tasks { all } | Command::Ts { all } => {
            // read in todolist file
            let list_file = ListFile::from_file(&todolists_path);

            if list_file.num_lists() < 1 {
                eprintln!("{}", NO_LISTS_MSG);
                std::process::exit(1)
            }

            // retrieve focused TodoList
            let focused = match list_file.get_focused() {
                Ok(list) => list,
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1)
                }
            };

            // always print focused todolist
            focused.print_tasks();

            // print rest of tasks if requested
            if all {
                for (name, list) in &list_file.lists {
                    if *name != focused.name {
                        list.print_tasks();
                    }
                }
            }
        },

        Command::Add { task, date } => {
            // read in listfile
            let mut list_file = ListFile::from_file(&todolists_path);

            // retrieve focused TodoList
            let list = match list_file.get_mut_focused() {
                Ok(list) => list,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1)
                }
            };

            // add new task
            list.add_tasks(task, date);

            // write todolist file
            list_file.to_file(&todolists_path);
        },

        Command::Drop { index } => {
            // read in listfile
            let mut list_file = ListFile::from_file(&todolists_path);

            // retrieve focused TodoList
            let list = match list_file.get_mut_focused() {
                Ok(list) => list,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1)
                }
            };

            // drop tasks
            list.drop_tasks(index);

            // write todolist file
            list_file.to_file(&todolists_path);
        },
        
        Command::Done { index } => {
            // read in listfile
            let mut list_file = ListFile::from_file(&todolists_path);

            // retrieve focused TodoList
            let list = match list_file.get_mut_focused() {
                Ok(list) => list,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1)
                }
            };

            // mark tasks as done
            list.update_completions(index, true);

            // write todolist file
            list_file.to_file(&todolists_path);
        },

        Command::Undo { index } => {
            // read in listfile
            let mut list_file = ListFile::from_file(&todolists_path);

            // retrieve focused TodoList
            let list = match list_file.get_mut_focused() {
                Ok(list) => list,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1)
                }
            };

            // mark tasks as undone
            list.update_completions(index, false);

            // write todolist file
            list_file.to_file(&todolists_path);
        }
    }
}
