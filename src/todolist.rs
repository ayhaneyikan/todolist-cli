use serde::{Serialize, Deserialize};

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