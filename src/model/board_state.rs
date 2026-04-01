use crate::APP_NAME;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const BOARD_SAVE_DIRECTORY: &str = "Boards";

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

/// The model containing the data of the Kanban board containing columns of tasks as well as it's
/// own name.
///
/// id: Uuid,
/// title: String,
/// pub columns: Vec<Column>,
#[derive(Clone, Serialize, Deserialize)]
pub struct Board {
    id: Uuid,
    pub title: String,
    pub columns: Vec<Column>,
}

impl Board {
    pub fn new(title: String) -> Self {
        Board {
            id: Uuid::new_v4(),
            title,
            columns: Vec::new(),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_columns(&mut self) -> &mut Vec<Column> {
        &mut self.columns
    }

    pub fn load_board(id: Uuid) -> Option<Self> {
        let proj_dirs = ProjectDirs::from("com", APP_NAME, APP_NAME)?;
        let save_path = proj_dirs.data_local_dir().join(BOARD_SAVE_DIRECTORY);
        let board_file_path = save_path.join(format!("{}.json", id));

        let json = std::fs::read_to_string(board_file_path).ok()?;
        serde_json::from_str(&json).ok()
    }

    pub fn save_board(&self) -> Option<bool> {
        let proj_dirs = ProjectDirs::from("com", APP_NAME, APP_NAME)?;

        let save_path = proj_dirs.data_local_dir().join(BOARD_SAVE_DIRECTORY);
        std::fs::create_dir_all(&save_path).ok()?;

        let board_file_path = save_path.join(format!("{}.json", self.id));

        let json = serde_json::to_string_pretty(self).ok()?;
        std::fs::write(board_file_path, json).ok()?;

        Some(true)
    }
}
