//! Data models for the Kanban board structure.

use crate::io::file_handling::to_snake_case;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A single checklist item within a task.
#[derive(Clone, Serialize, Deserialize)]
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
    pub checklist: Option<Vec<Item>>,
}

impl Task {
    /// Creates a new, empty task with a random UUID.
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
            title,
            file_name,
            columns: Vec::new(),
        }
    }

    /// Provides a mutable reference to the board's columns.
    pub fn get_columns(&mut self) -> &mut Vec<Column> {
        &mut self.columns
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
