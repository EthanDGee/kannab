//! Data models for the Kanban board structure.

use crate::io::file_handling::to_snake_case;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const VERSION_NUMBER: &str = "0.1.0";

/// A single checklist item within a task.
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Item {
    /// Unique identifier for the item.
    id: Uuid,
    /// The descriptive text for the item.
    pub description: String,
    /// Whether the item is checked.
    pub complete: bool,
}

impl Item {
    /// Creates a new, empty checklist item with a random UUID.
    pub fn new() -> Self {
        Item {
            id: Uuid::new_v4(),
            description: String::new(),
            complete: false,
        }
    }
}

/// A task within a Kanban column, optionally containing a checklist.
#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique identifier for the task.
    id: Uuid,
    /// The display title of the task.
    pub title: String,
    /// Detailed multi-line description of the task.
    pub description: String,
    /// Whether the overall task is marked as complete.
    pub complete: bool,
    /// Optional list of smaller steps/items.
    pub checklist: Vec<Item>,
}

impl Task {
    /// Creates a new, empty task with a random UUID.
    pub fn new() -> Self {
        Task {
            id: Uuid::new_v4(),
            title: String::new(),
            description: String::new(),
            complete: false,
            checklist: vec![],
        }
    }
}

/// A column on a Kanban board which contains a list of tasks.
#[derive(Clone, Serialize, Deserialize)]
pub struct Column {
    /// Unique identifier for the column.
    id: Uuid,
    /// The title displayed at the top of the column.
    pub title: String,
    /// The ordered list of tasks within this column.
    pub tasks: Vec<Task>,
}

impl Column {
    /// Creates a new, empty column with a random UUID.
    pub fn new() -> Self {
        Column {
            id: Uuid::new_v4(),
            title: String::new(),
            tasks: Vec::new(),
        }
    }

    /// Returns true if the column contains no tasks.
    pub fn task_list_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    /// Provides an immutable reference to the task at the given index.
    pub fn get_task(&self, index: usize) -> Option<&Task> {
        self.tasks.get(index)
    }

    /// Provides a mutable reference to the task at the given index.
    pub fn get_task_mut(&mut self, index: usize) -> Option<&mut Task> {
        self.tasks.get_mut(index)
    }

    /// Removes and returns the task at the given index, if it exists.
    pub fn remove_task(&mut self, index: usize) -> Option<Task> {
        if index < self.tasks.len() {
            Some(self.tasks.remove(index))
        } else {
            None
        }
    }

    /// Inserts a task at the given index.
    pub fn insert_task(&mut self, index: usize, task: Task) {
        if index <= self.tasks.len() {
            self.tasks.insert(index, task);
        } else {
            self.tasks.push(task);
        }
    }

    /// Swaps the tasks at the two given indices.
    pub fn swap_tasks(&mut self, i: usize, j: usize) {
        if i < self.tasks.len() && j < self.tasks.len() {
            self.tasks.swap(i, j);
        }
    }
}

/// A lightweight representation of a board used in lists and metadata.
#[derive(Clone, Serialize, Deserialize)]
pub struct BoardName {
    /// The display title of the board.
    pub title: String,
    /// The snake_case version of the title, used for file naming.
    pub snake_case: String,
}

impl BoardName {
    /// Creates a new `BoardName` from a title, automatically generating the snake_case filename.
    pub fn new(title: String) -> Self {
        let snake_case = to_snake_case(title.clone());

        BoardName { title, snake_case }
    }
}

/// A complete Kanban board containing multiple columns of tasks.
#[derive(Clone, Serialize, Deserialize)]
pub struct Board {
    /// Version number of the Board
    pub version_number: String,
    /// The title of the board.
    pub title: String,
    /// The filename where this board is stored (without extension).
    pub file_name: String,
    /// The ordered list of columns in this board.
    pub columns: Vec<Column>,
}

impl Board {
    /// Creates a new empty board with the given title.
    pub fn new(title: String) -> Self {
        let file_name = to_snake_case(title.clone());
        Board {
            version_number: VERSION_NUMBER.to_string(),
            title,
            file_name,
            columns: Vec::new(),
        }
    }

    /// Provides a mutable reference to the board's columns.
    pub fn get_columns(&mut self) -> &mut Vec<Column> {
        &mut self.columns
    }

    /// Provides an immutable reference to the column at the given index.
    pub fn get_column(&self, index: usize) -> Option<&Column> {
        self.columns.get(index)
    }

    /// Provides a mutable reference to the column at the given index.
    pub fn get_column_mut(&mut self, index: usize) -> Option<&mut Column> {
        self.columns.get_mut(index)
    }

    /// Removes and returns the column at the given index, if it exists.
    pub fn remove_column(&mut self, index: usize) -> Option<Column> {
        if index < self.columns.len() {
            Some(self.columns.remove(index))
        } else {
            None
        }
    }

    /// Swaps the columns at the two given indices.
    pub fn swap_columns(&mut self, i: usize, j: usize) {
        if i < self.columns.len() && j < self.columns.len() {
            self.columns.swap(i, j);
        }
    }

    /// Provides an immutable reference to the task in the column and at the index provided.
    pub fn get_task(&self, col_index: usize, task_index: usize) -> Option<&Task> {
        self.get_column(col_index)?.get_task(task_index)
    }

    /// Provides a mutable reference to the task in the column and at the index provided.
    pub fn get_task_mut(&mut self, col_index: usize, task_index: usize) -> Option<&mut Task> {
        self.get_column_mut(col_index)?.get_task_mut(task_index)
    }

    /// Returns true if the board contains no columns.
    pub fn column_list_empty(&self) -> bool {
        self.columns.is_empty()
    }

    /// Returns true if the column at the provided index contains no tasks.
    pub fn task_list_empty(&self, column_index: usize) -> bool {
        self.columns
            .get(column_index)
            .map(|col| col.task_list_empty())
            .unwrap_or(true)
    }
}
