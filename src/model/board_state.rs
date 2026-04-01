use crate::{APP_NAME, io::file_handling::to_snake_case};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use uuid::Uuid;

/// Handles the member of a todo list for a given task
///
/// id: Uuid
/// pub description: String
/// complete: bool
#[derive(Clone, Serialize, Deserialize)]
pub struct Item {
    id: Uuid,
    pub description: String,
    pub complete: bool,
}

impl Item {
    pub fn new() -> Self {
        Item {
            id: Uuid::new_v4(),
            description: String::new(),
            complete: false,
        }
    }
}

/// The member task of a Kanban column They contain the name of the task, a more detailed
/// description of the task, the ability to have an attached to-do list, and whether or not the task
/// is completed.
///
/// id: Uuid,
/// title: String,
/// description: String,
/// complete: bool,
/// checklist: Option<Vec<Item>> = the items of of the optional to-do list
#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    id: Uuid,
    pub title: String,
    pub description: String,
    pub complete: bool,
    pub checklist: Option<Vec<Item>>,
}

impl Task {
    pub fn new() -> Self {
        Task {
            id: Uuid::new_v4(),
            title: String::new(),
            description: String::new(),
            complete: false,
            checklist: None,
        }
    }
}

///  A named column of the Kanban board containing a list of tasks as well
///
/// id: Uuid,
/// title: String,
/// tasks: Vec<Task>,
#[derive(Clone, Serialize, Deserialize)]
pub struct Column {
    id: Uuid,
    pub title: String,
    pub tasks: Vec<Task>,
}

impl Column {
    pub fn new() -> Self {
        Column {
            id: Uuid::new_v4(),
            title: String::new(),
            tasks: Vec::new(),
        }
    }
}

/// Stores the title of a board and it's associated file name
pub struct BoardName {
    pub title: String,
    pub snake_case: String,
}

impl BoardName {
    pub fn new(title: String) -> Self {
        let snake_case = to_snake_case(title.clone());

        BoardName { title, snake_case }
    }
}

/// The model containing the data of the Kanban board containing columns of tasks as well as it's
/// own name.
///
/// id: Uuid,
/// title: String,
/// pub columns: Vec<Column>,
#[derive(Clone, Serialize, Deserialize)]
pub struct Board {
    pub title: String,
    pub file_name: String,
    pub columns: Vec<Column>,
}

impl Board {
    pub fn new(title: String) -> Self {
        let file_name = to_snake_case(title.clone());
        Board {
            title,
            file_name,
            columns: Vec::new(),
        }
    }

    pub fn get_columns(&mut self) -> &mut Vec<Column> {
        &mut self.columns
    }
}
