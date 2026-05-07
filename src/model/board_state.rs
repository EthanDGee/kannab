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

    /// Toggles completion of the checklist item.
    pub fn toggle_completion(&mut self) {
        self.complete = !self.complete;
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

    /// Toggles completion of the task.
    pub fn toggle_completion(&mut self) {
        self.complete = !self.complete;
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use crate::model::board_state::*;

    #[test]
    fn test_item_new() {
        let item = Item::new();
        assert_eq!(item.description, "");
        assert!(!item.complete);
    }

    #[test]
    fn test_item_toggle_completion() {
        let mut item = Item::new();
        item.toggle_completion();
        assert!(item.complete);
        item.toggle_completion();
        assert!(!item.complete);
    }

    #[test]
    fn test_task_new() {
        let task = Task::new();
        assert_eq!(task.title, "");
        assert_eq!(task.description, "");
        assert!(!task.complete);
        assert!(task.checklist.is_empty());
    }

    #[test]
    fn test_task_toggle_completion() {
        let mut task = Task::new();
        task.toggle_completion();
        assert!(task.complete);
        task.toggle_completion();
        assert!(!task.complete);
    }

    #[test]
    fn test_column_new() {
        let column = Column::new();
        assert_eq!(column.title, "");
        assert!(column.task_list_empty());
    }

    #[test]
    fn test_column_task_manipulation() {
        let mut column = Column::new();
        let mut task = Task::new();
        task.title = "Test Task".to_string();

        column.insert_task(0, task.clone());
        assert_eq!(column.tasks.len(), 1);
        assert_eq!(column.get_task(0).unwrap().title, "Test Task");

        let mut task2 = Task::new();
        task2.title = "Second Task".to_string();
        column.insert_task(1, task2);
        assert_eq!(column.tasks.len(), 2);

        column.swap_tasks(0, 1);
        assert_eq!(column.get_task(0).unwrap().title, "Second Task");

        let removed = column.remove_task(0).unwrap();
        assert_eq!(removed.title, "Second Task");
        assert_eq!(column.tasks.len(), 1);
    }

    #[test]
    fn test_board_name_new() {
        let bn = BoardName::new("My Board".to_string());
        assert_eq!(bn.title, "My Board");
        assert_eq!(bn.snake_case, "my_board");
    }

    #[test]
    fn test_board_new() {
        let board = Board::new("Project X".to_string());
        assert_eq!(board.title, "Project X");
        assert_eq!(board.file_name, "project_x");
        assert!(board.column_list_empty());
    }

    #[test]
    fn test_board_column_manipulation() {
        let mut board = Board::new("Test Board".to_string());
        let mut col = Column::new();
        col.title = "Todo".to_string();

        board.columns.push(col);
        assert_eq!(board.columns.len(), 1);
        assert_eq!(board.get_column(0).unwrap().title, "Todo");

        let mut col2 = Column::new();
        col2.title = "Done".to_string();
        board.columns.push(col2);

        board.swap_columns(0, 1);
        assert_eq!(board.get_column(0).unwrap().title, "Done");

        board.remove_column(0);
        assert_eq!(board.columns.len(), 1);
        assert_eq!(board.get_column(0).unwrap().title, "Todo");
    }

    #[test]
    fn test_board_get_task_helpers() {
        let mut board = Board::new("Test".to_string());
        let mut col = Column::new();
        let mut task = Task::new();
        task.title = "T1".to_string();
        col.tasks.push(task);
        board.columns.push(col);

        assert!(board.get_task(0, 0).is_some());
        assert_eq!(board.get_task(0, 0).unwrap().title, "T1");
        assert!(board.get_task(1, 0).is_none());
        assert!(board.get_task(0, 1).is_none());

        if let Some(t) = board.get_task_mut(0, 0) {
            t.title = "Updated".to_string();
        }
        assert_eq!(board.get_task(0, 0).unwrap().title, "Updated");
    }

    #[test]
    fn test_board_serialization() {
        let mut board = Board::new("Full Board".to_string());
        let mut col = Column::new();
        col.title = "Col".to_string();
        let mut task = crate::model::board_state::Task::new();
        task.title = "Task".to_string();
        let mut item = crate::model::board_state::Item::new();
        item.description = "Item".to_string();
        task.checklist.push(item);
        col.tasks.push(task);
        board.columns.push(col);

        let json = serde_json::to_string(&board).unwrap();
        let deserialized: Board = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.title, board.title);
        assert_eq!(deserialized.columns.len(), 1);
        assert_eq!(deserialized.columns[0].tasks[0].title, "Task");
        assert_eq!(
            deserialized.columns[0].tasks[0].checklist[0].description,
            "Item"
        );
    }
}
