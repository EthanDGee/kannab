//! Handlers for I/O and system-level actions.

use crate::{io::file_handling, message::action::Action, model::app_state::AppState};

/// Marks the application state as having unsaved changes, triggering future auto-saves.
pub fn mark_dirty(model: &mut AppState) -> Option<Action> {
    model.pending_changes = true;
    None
}

/// Responds to periodic tick events by checking if an auto-save is required.
pub fn handle_tick(model: &mut AppState) -> Option<Action> {
    if model.pending_changes {
        Some(Action::Save)
    } else {
        None
    }
}

/// Saves the current board and board list to local application data.
pub fn save(model: &mut AppState) -> Option<Action> {
    if let Some(board_state) = &model.board_state {
        file_handling::save_board(&board_state.board);
    }
    file_handling::save_board_list(&model.board_list);
    model.pending_changes = false;
    None
}

/// Prepares the application for exit by saving changes and raising the quit flag.
pub fn quit(model: &mut AppState) -> Option<Action> {
    model.should_quit = true;
    Some(Action::Save)
}
