use self::errors::ListError;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::Display,
    io::{self, Write},
};

use crate::utils::date::Date;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListFile {
    pub focused: Option<String>,
    pub lists: HashMap<String, TodoList>,
}

impl ListFile {
    /// Creates a new instance of a ListFile.
    /// This is only expected to happen when no ListFile is found in the user's home directory.
    /// ### Returns
    /// New ListFile instance
    pub fn new() -> Self {
        ListFile {
            focused: None,
            lists: HashMap::new(),
        }
    }

    /// Read from file and deserialize ListFile.
    /// ### Returns
    /// New ListFile instance from file
    pub fn from_file(file_path: &str) -> Self {
        // confirm that the listfile exists
        if !std::path::Path::new(file_path).exists() {
            eprintln!(
                "Error: ListFile {} does not exist. Todolists may have failed to initialize.",
                file_path
            );
            std::process::exit(1)
        }

        // read file contents and deserialize
        let contents =
            std::fs::read_to_string(file_path).expect("Error: failed to read ListFile from file");
        serde_json::from_str(&contents).expect("Error: failed to deserialize ListFile")
    }

    /// Serializes ListFile and writes it to file.
    pub fn to_file(&self, file_path: &str) {
        let encoded = serde_json::to_string(&self).expect("Error: failed to serialize ListFile");
        // overwrite old file
        std::fs::write(file_path, encoded)
            .expect("Error: failed to write serialized ListFile to file");
    }

    /// Create new list within the ListFile.
    /// ### Returns
    /// Result indicating success of list creation
    pub fn create_list(&mut self, name: &str) -> Result<(), ListError> {
        // confirm that list name is unique
        if self.lists.contains_key(name) {
            return Err(ListError::DuplicateListName {
                name: name.to_string(),
            });
        }

        // add list
        self.lists
            .insert(name.to_string(), TodoList::new(name.to_string()));

        // set as focused list if no list is focused
        if self.focused.is_none() {
            self.focused = Some(name.to_string());
        }
        Ok(())
    }

    /// Delete the given list from the ListFile.
    /// Confirms user selection with a list name retype.
    /// ### Returns
    /// Result indicating success of list deletion
    pub fn delete_list(&mut self, name: &str) -> Result<(), ListError> {
        // confirm that the list exists
        if !self.lists.contains_key(name) {
            return Err(ListError::NonexistentListName {
                name: name.to_string(),
            });
        }

        // prompt user to confirm delete
        print!("Please confirm list deletion by re-typing the list name: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim(); // remove whitespace

        // ensure input matches
        if input != name {
            return Err(ListError::FailedDeleteConfirmation {
                entered: input.to_string(),
                requested: name.to_string(),
            });
        }

        // delete list
        self.lists.remove(name);

        // if no focus, or focused list is the one being deleted, shift focus
        if self.focused.is_none() || self.focused.as_ref().unwrap() == name {
            // sort remaining lists
            let mut names: Vec<&String> = self.lists.keys().collect();
            names.sort();
            // set new focus
            if names.is_empty() {
                self.focused = None;
            } else {
                self.focused = Some(names.first().unwrap().to_string());
            }
        }
        Ok(())
    }

    /// Retrieves the number of lists available
    pub fn num_lists(&self) -> usize {
        self.lists.len()
    }
    /// Returns names of the available lists
    pub fn get_list_names(&self) -> Vec<&String> {
        self.lists.keys().collect()
    }

    /// Shifts focus to the given list.
    /// ### Returns
    /// Result indicating success of focus change
    pub fn shift_focus(&mut self, name: &str) -> Result<(), ListError> {
        // confirm that the list exists
        if !self.lists.contains_key(name) {
            return Err(ListError::NonexistentListName {
                name: name.to_string(),
            });
        }

        // shift focus
        self.focused = Some(name.to_string());
        Ok(())
    }

    /// Returns immutable ref to the currently focused TodoList.
    /// ### Returns
    /// &TodoList or ListError
    pub fn get_focused(&self) -> Result<&TodoList, ListError> {
        // retrieve requested TodoList
        if let Some(list) = self.focused.as_ref() {
            Ok(self.lists.get(list).unwrap())
        } else {
            Err(ListError::NoFocusedList)
        }
    }

    /// Returns mutable ref to the currently focused TodoList.
    /// ### Returns
    /// &mut TodoList or ListError
    pub fn get_mut_focused(&mut self) -> Result<&mut TodoList, ListError> {
        // retrieve requested TodoList
        if let Some(list) = self.focused.as_ref() {
            Ok(self.lists.get_mut(list).unwrap())
        } else {
            Err(ListError::NoFocusedList)
        }
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
    pub fn add_tasks(&mut self, tasks: Vec<String>, date: Option<Date>) {
        for t in tasks {
            self.tasks.push(Task {
                title: t,
                complete: false,
                date,
            })
        }
    }

    /// Helper func to sort, dedup, and reverse a list of usize.
    /// Used when receiving multiple indices to remove from the task list.
    /// Reversing the list allows for save deletion of elements while iterating over tasks.
    fn sort_uniq_reverse(mut l: Vec<usize>) -> Vec<usize> {
        // remove duplicate indices
        l.sort();
        l.dedup();
        l.reverse(); // reverse so that indices don't change

        // decrement each value
        for i in l.iter_mut() {
            *i -= 1;
        }
        l
    }

    /// Drop task(s) from the todolist
    pub fn drop_tasks(&mut self, mut index: Vec<usize>) {
        index = Self::sort_uniq_reverse(index);

        self.tasks.sort();

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

        self.tasks.sort();

        // mark each task as complete
        for i in index {
            if i < self.tasks.len() {
                self.tasks.get_mut(i).unwrap().complete = complete;
            }
        }
    }

    /// Print all tasks in the todolist
    pub fn print_tasks(&self) {
        // count digits in length of list to properly space indices
        let digits = self.tasks.len().to_string().len();

        // sort tasks prior to printing
        let mut tasks = self.tasks.clone();
        tasks.sort();

        println!("-- {} --", self.name);
        for (i, t) in tasks.iter().enumerate() {
            println!("{: <digits$}| {}", i + 1, t);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct Task {
    pub title: String,
    pub date: Option<Date>,
    pub complete: bool,
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.date {
            Some(d) => write!(
                f,
                "{} [{}] {}",
                if self.complete { "✓" } else { "✕" },
                d,
                self.title
            ),
            None => write!(
                f,
                "{} {}",
                if self.complete { "✓" } else { "✕" },
                self.title
            ),
        }
    }
}

// sorting impls

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.date == other.date && self.complete == other.complete
    }
}
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.date.cmp(&other.date)
    }
}

pub mod errors {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum ListError {
        /// Attempting to create a list with a used name
        #[error("Cannot create list named {name:?}, a list already exists with this name.")]
        DuplicateListName { name: String },
        /// Attempting to delete a list which doesn't exist
        #[error("Cannot delete list named {name:?}, no such list exists")]
        NonexistentListName { name: String },
        /// Deletion confirmation did not match
        #[error("Cannot delete list; List name entered {entered:?} does not match requested deletion {requested:?}.")]
        FailedDeleteConfirmation { entered: String, requested: String },
        /// Attempted to get focused list when no list is focused
        #[error("Cannot get focused list; there is none.")]
        NoFocusedList,
    }
}
