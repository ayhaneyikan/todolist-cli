use std::{collections::HashMap, process::exit, io::{stdin, stdout, Write}};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ListFile {
    pub focused: Option<String>,  /* rethink what this will be */
    pub lists: HashMap<String, TodoList>,
}

impl ListFile {
    pub fn new() -> Self {
        ListFile {
            focused: None,
            lists: HashMap::new(),
        }
    }

    /// Create ListFile from serialized file
    /// Exits with error if file does not exist
    pub fn from_file(arg: &str) -> Self {
        // check that file exists
        if !std::path::Path::new(arg).exists() {
            println!("Todolist has not been initialized");
            exit(1);
        }

        // read file contents
        let contents = std::fs::read_to_string(arg)
            .expect("Something went wrong reading the todolist file");

        bincode::deserialize(&contents.as_bytes()).unwrap()
    }

    /// Add a new list to the listfile
    /// Exits with error if list already exists
    pub fn add_list(&mut self, name: String) {
        // check that list doesn't already exist
        if self.lists.contains_key(&name) {
            println!("Todolist '{}' already exists", name);
            exit(1);
        }
        
        // add list
        self.lists.insert(name.clone(), TodoList::new(name.clone()));

        // set as focused list if no list is focused
        if self.focused.is_none() {
            self.focused = Some(name.clone());
        }
        println!("Created todolist '{}'", name);
    }

    /// Writes serialized ListFile to file
    pub fn to_file(&self, arg: &str) {
        // serialize self
        let encoded = bincode::serialize(&self).unwrap();
        // write to file (overwrites old file)
        std::fs::write(arg, encoded).unwrap();
    }

    /// Deletes list with given name
    /// Confirms user selection
    pub fn delete_list(&mut self, name: String) {
        // check that list exists
        if !self.lists.contains_key(&name) {
            println!("No todolist '{}'", &name);
            exit(1);
        }

        // prompt user to confirm delete
        print!("Please confirm the delete by typing the list name: ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let input = input.trim();  // remove whitespace

        // ensure input matches
        if input != name {
            println!("Cancelling deletion since names don't match");
            exit(1);
        }

        // delete list
        self.lists.remove(&name);

        // if deleted list was focused, shift focus
        let mut names: Vec<&String> = self.lists.keys().collect();
        names.sort();
        if names.len() == 0 {
            self.focused = None;
        } else {
            self.focused = Some(names.get(0).unwrap().to_string());
        }

        println!("Successfully deleted '{}'", name);
    }

    
}



#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    pub name: String,

    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn new(name: String) -> Self {
        TodoList {
            name,
            tasks: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    // pub due: String,
    pub complete: bool,
}