use crate::APP_NAME;
use crate::model::board_state::{Board, BoardName};
use crate::view::theme::ColorScheme;
use directories::ProjectDirs;
use std::{fs, path::PathBuf};

const DATA_FILE_TYPE: &str = ".json";
const CONFIG_FILE_TYPE: &str = ".toml";
const BOARD_LIST_FILENAME: &str = "boards";
const THEME_FILENAME: &str = "theme";

/// Converts a string to snake_case
pub fn to_snake_case(s: String) -> String {
    s.split_whitespace()
        .map(|word| word.to_lowercase())
        .collect::<Vec<_>>()
        .join("_")
}

// BOARD LIST HANDLING

/// Returns the path to the board list file
fn board_list_path() -> Option<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", APP_NAME, APP_NAME)?;
    let save_path = proj_dirs.data_local_dir();
    std::fs::create_dir_all(save_path).ok()?;
    Some(save_path.join(format!("{}{}", BOARD_LIST_FILENAME, DATA_FILE_TYPE)))
}

/// Saves the list of boards to the file system
pub fn save_board_list(board_list: &Vec<BoardName>) -> Option<bool> {
    let path = board_list_path()?;
    let json = serde_json::to_string_pretty(board_list).ok()?;
    std::fs::write(path, json).ok()?;
    Some(true)
}

/// Loads the list of boards from the file system
pub fn load_board_list() -> Option<Vec<BoardName>> {
    let path = board_list_path()?;
    let json = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&json).ok()
}

// BOARD HANDLING

/// Returns the path to a board file based on its title
pub fn board_path(title: &str) -> Option<PathBuf> {
    let file_name = to_snake_case(title.to_string());
    let proj_dirs = ProjectDirs::from("com", APP_NAME, APP_NAME)?;

    let board_directory: &str = "boards";
    let save_path = proj_dirs.data_local_dir().join(board_directory);
    std::fs::create_dir_all(&save_path).ok()?;

    Some(save_path.join(format!("{}{}", file_name, DATA_FILE_TYPE)))
}

/// Saves a board to the file system
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

// THEME HANDLING

/// Returns the path to the theme file
fn theme_path() -> Option<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", APP_NAME, APP_NAME)?;
    let save_path = proj_dirs.config_local_dir();
    std::fs::create_dir_all(save_path).ok()?;
    Some(save_path.join(format!("{}{}", THEME_FILENAME, CONFIG_FILE_TYPE)))
}

/// Checks if the theme file exists on the file system
fn theme_exists() -> bool {
    theme_path().map_or(false, |path| path.exists())
}

/// Saves the color scheme to the file system
fn save_theme(color_scheme: &ColorScheme) -> Option<bool> {
    let path = theme_path()?;
    let json = toml::to_string_pretty(color_scheme).ok()?;
    std::fs::write(path, json).ok()?;
    Some(true)
}

/// Loads the color scheme from the file system
fn load_theme() -> Option<ColorScheme> {
    let path = theme_path()?;
    let json = std::fs::read_to_string(path).ok()?;
    toml::from_str(&json).ok()
}

/// Attempts to load config theme if fails to load to toml loading it fails back to the default. If
/// no config exists save the default theme to the project directory
pub fn initialize_theme() -> ColorScheme {
    if theme_exists() {
        match load_theme() {
            Some(theme) => return theme,
            None => return ColorScheme::default(),
        }
    }

    let theme = ColorScheme::default();
    save_theme(&theme);
    theme
}
