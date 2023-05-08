use std::{collections::HashMap, process::exit, io::{stdin, stdout, Write}, fmt::Display};

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
    /// Confirms user selection with a list name retype
    /// Exits with error if there no such list or an input mismatch
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

    /// Shifts focused list to the given
    /// Exits with error if list doesn't exist
    pub fn shift_focus(&mut self, name: String) {
        // check that list exists
        if !self.lists.contains_key(&name) {
            println!("No todolist '{}'", &name);
            exit(1);
        }

        // shift focus
        self.focused = Some(name);
    }

    /// Returns TodoList currently focused
    /// Exits with error if no list is focused
    pub fn get_focused(&mut self) -> &mut TodoList {
        // check that there's a focused list
        if self.focused.is_none() {
            println!("You have no lists");
            exit(1);
        }

        // access list
        self.lists.get_mut(self.focused.as_ref().unwrap()).unwrap()
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

    /// Add task(s) to the todolist
    pub fn add_tasks(&mut self, tasks: Vec<String>) {
        for t in tasks {
            self.tasks.push(
                Task {
                    title: t,
                    complete: false,
                }
            )
        }
    }
    
    /// Helper func to sort, dedup, and reverse a list of usize
    fn sort_uniq_reverse(mut l: Vec<usize>) -> Vec<usize> {
        // remove duplicate indices
        l.sort();
        l.dedup();
        l.reverse();  // reverse so that indices don't change

        // decrement each value
        for i in l.iter_mut() {
            *i -= 1;
        }
        l
    }

    /// Drop task(s) from the todolist
    pub fn drop_tasks(&mut self, mut index: Vec<usize>) {
        index = Self::sort_uniq_reverse(index);

        // drop them from the list
        for i in index {
            if i < self.tasks.len() {
                self.tasks.remove(i);
            }
        }
    }

    /// Update task(s) as complete or incomplete
    pub fn update_completions(&mut self, mut index: Vec<usize>, complete: bool) {
        index = Self::sort_uniq_reverse(index);

        // mark each task as complete
        for i in index {
            if i < self.tasks.len() {
                self.tasks.get_mut(i).unwrap().complete = complete;
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    // pub due: String,
    pub complete: bool,
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", if self.complete { "x" } else { "o" }, self.title)
    }
}