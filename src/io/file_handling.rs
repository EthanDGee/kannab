use crate::{APP_NAME, model::board_state::Board};
use directories::ProjectDirs;
use std::{fs, path::PathBuf};

const DATA_FILE_TYPE: &str = ".json";

/// Converts a string to snake_case
pub fn to_snake_case(s: String) -> String {
    s.split_whitespace()
        .map(|word| word.to_lowercase())
        .collect::<Vec<_>>()
        .join("_")
}

/// Returns the path to a board file based on its title
pub fn board_path(title: &str) -> Option<PathBuf> {
    let file_name = to_snake_case(title.to_string());
    let proj_dirs = ProjectDirs::from("com", APP_NAME, APP_NAME)?;

    let board_directory: &str = "boards";
    let save_path = proj_dirs.data_local_dir().join(board_directory);
    std::fs::create_dir_all(&save_path).ok()?;

    Some(save_path.join(format!("{}{}", file_name, DATA_FILE_TYPE)))
}

/// Saves a board to the filesystem
pub fn save_board(board: &Board) -> Option<bool> {
    let board_file_path = board_path(&board.title)?;

    let json = serde_json::to_string_pretty(board).ok()?;
    std::fs::write(board_file_path, json).ok()?;

    Some(true)
}

/// Loads a board based on its title
pub fn load_board(title: &str) -> Option<Board> {
    let board_file_path = board_path(title)?;

    let json = std::fs::read_to_string(board_file_path).ok()?;
    serde_json::from_str(&json).ok()
}

/// Deletes a board from the save directory
pub fn delete_board(title: &str) -> bool {
    if let Some(board_file_path) = board_path(title) {
        fs::remove_file(board_file_path).is_ok()
    } else {
        false
    }
}
