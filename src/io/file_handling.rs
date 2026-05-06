//! File handling utilities for loading and saving application data.
//!
//! This module provides functions for saving board data, board lists,
//! and theme configurations to the user's local application data directory.

use crate::APP_NAME;
use crate::model::board_state::{Board, BoardName};
use crate::view::theme::ColorScheme;
use directories::ProjectDirs;
use std::{fs, path::PathBuf};

const DATA_FILE_TYPE: &str = ".json";
const CONFIG_FILE_TYPE: &str = ".toml";
const BOARD_LIST_FILENAME: &str = "boards";
const THEME_FILENAME: &str = "theme";

/// Converts a string to snake_case.
///
/// Used for generating consistent file names from user-provided titles.
pub fn to_snake_case(s: String) -> String {
    s.split_whitespace()
        .map(|word| word.to_lowercase())
        .collect::<Vec<_>>()
        .join("_")
}

// --- BOARD LIST HANDLING ---

/// Returns the path to the board list metadata file.
fn board_list_path() -> Option<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", APP_NAME, APP_NAME)?;
    let save_path = proj_dirs.data_local_dir();
    std::fs::create_dir_all(save_path).ok()?;
    Some(save_path.join(format!("{}{}", BOARD_LIST_FILENAME, DATA_FILE_TYPE)))
}

/// Saves the list of boards to the file system.
pub fn save_board_list(board_list: &Vec<BoardName>) -> Option<bool> {
    let path = board_list_path()?;
    let json = serde_json::to_string_pretty(board_list).ok()?;
    std::fs::write(path, json).ok()?;
    Some(true)
}

/// Loads the list of boards from the file system.
pub fn load_board_list() -> Option<Vec<BoardName>> {
    let path = board_list_path()?;
    let json = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&json).ok()
}

// --- BOARD HANDLING ---

/// Returns the absolute path to a specific board's data file.
pub fn board_path(title: &str) -> Option<PathBuf> {
    let file_name = to_snake_case(title.to_string());
    let proj_dirs = ProjectDirs::from("com", APP_NAME, APP_NAME)?;

    let board_directory: &str = "boards";
    let save_path = proj_dirs.data_local_dir().join(board_directory);
    std::fs::create_dir_all(&save_path).ok()?;

    Some(save_path.join(format!("{}{}", file_name, DATA_FILE_TYPE)))
}

/// Serializes and saves a `Board` to its corresponding data file.
pub fn save_board(board: &Board) -> Option<bool> {
    let board_file_path = board_path(&board.title)?;

    let json = serde_json::to_string_pretty(board).ok()?;
    std::fs::write(board_file_path, json).ok()?;

    Some(true)
}

/// Loads a `Board` from disk based on its title.
pub fn load_board(title: &str) -> Option<Board> {
    let board_file_path = board_path(title)?;

    let json = std::fs::read_to_string(board_file_path).ok()?;
    serde_json::from_str(&json).ok()
}

/// Deletes a board's data file from the file system.
pub fn delete_board(title: &str) -> bool {
    if let Some(board_file_path) = board_path(title) {
        fs::remove_file(board_file_path).is_ok()
    } else {
        false
    }
}

// --- THEME HANDLING ---

/// Returns the path to the theme configuration file.
fn theme_path() -> Option<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", APP_NAME, APP_NAME)?;
    let save_path = proj_dirs.config_local_dir();
    std::fs::create_dir_all(save_path).ok()?;
    Some(save_path.join(format!("{}{}", THEME_FILENAME, CONFIG_FILE_TYPE)))
}

/// Checks if the theme configuration file exists.
fn theme_exists() -> bool {
    theme_path().is_some_and(|path| path.exists())
}

/// Saves the provided `ColorScheme` to disk as TOML.
fn save_theme(color_scheme: &ColorScheme) -> Option<bool> {
    let path = theme_path()?;
    let json = toml::to_string_pretty(color_scheme).ok()?;
    std::fs::write(path, json).ok()?;
    Some(true)
}

/// Loads the `ColorScheme` from the configuration directory.
fn load_theme() -> Option<ColorScheme> {
    let path = theme_path()?;
    let json = std::fs::read_to_string(path).ok()?;
    toml::from_str(&json).ok()
}

/// Loads the user's theme or initializes the default configuration if none exists.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("Hello World".to_string()), "hello_world");
        assert_eq!(
            to_snake_case("Multiple   Spaces".to_string()),
            "multiple_spaces"
        );
        assert_eq!(to_snake_case("Lowercase".to_string()), "lowercase");
        assert_eq!(to_snake_case("UPPERCASE".to_string()), "uppercase");
        assert_eq!(to_snake_case("".to_string()), "");
    }
}
