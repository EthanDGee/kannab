//! Utilities and handlers for navigating through menus and board items.

use crate::message::action::Action;
use crate::model::app_state::{AppMode, AppState};

// ---------------------
// Navigation Utilities
// ---------------------

/// Increments an index with wrapping: returns 0 if at the last element.
pub fn increment_wrap(index: usize, len: usize) -> usize {
    if len <= 1 || index >= len - 1 {
        0
    } else {
        index + 1
    }
}

/// Increments an index without wrapping: returns `None` if at the last element.
pub fn increment_no_wrap(index: usize, len: usize) -> Option<usize> {
    if len == 0 || index >= len - 1 {
        None
    } else {
        Some(index + 1)
    }
}

/// Decrements an index with wrapping: returns `len - 1` if at the first element.
pub fn decrement_wrap(index: usize, len: usize) -> usize {
    if len <= 1 {
        0
    } else if index == 0 {
        len - 1
    } else {
        index - 1
    }
}

/// Decrements an index without wrapping: returns `None` if at the first element.
pub fn decrement_no_wrap(index: usize) -> Option<usize> {
    if index == 0 { None } else { Some(index - 1) }
}

// ---------------------------
// Global Navigation Bindings
// ---------------------------

/// Moves the active selection up.
///
/// In Picker mode, selects the previous board.
/// In Board mode, selects the previous task in the current column.
pub fn move_up(model: &mut AppState) -> Option<Action> {
    match model.mode {
        AppMode::Picker => {
            let len = model.board_list.len();
            model.picker_state.index = decrement_wrap(model.picker_state.index, len);
        }
        AppMode::Board => {
            if let Some(board_state) = &mut model.board_state
                && let Some(column) = board_state.current_column()
            {
                let len = column.tasks.len();
                board_state.task_index = decrement_wrap(board_state.task_index, len);
            }
        }
    }
    None
}

/// Moves the active selection down.
///
/// In Picker mode, selects the next board.
/// In Board mode, selects the next task in the current column.
pub fn move_down(model: &mut AppState) -> Option<Action> {
    match model.mode {
        AppMode::Picker => {
            let len = model.board_list.len();
            model.picker_state.index = increment_wrap(model.picker_state.index, len);
        }
        AppMode::Board => {
            if let Some(board_state) = &mut model.board_state {
                let task_index = board_state.task_index;
                let col_index = board_state.column_index;
                if let Some(column) = board_state.board.get_column_mut(col_index) {
                    let len = column.tasks.len();
                    if len > 0 && task_index == len - 1 && !column.tasks[len - 1].title.is_empty() {
                        column.tasks.push(crate::model::board_state::Task::new());
                        board_state.task_index = len;
                        return Some(Action::MarkDirty);
                    }
                    board_state.task_index = increment_wrap(task_index, len);
                }
            }
        }
    }
    None
}

/// Moves the active selection left.
///
/// In Board mode, selects the column to the left.
pub fn move_left(model: &mut AppState) -> Option<Action> {
    match model.mode {
        AppMode::Picker => {}
        AppMode::Board => {
            if let Some(board_state) = &mut model.board_state {
                if board_state.column_list_empty() {
                    return None;
                }

                if let Some(new_index) = decrement_no_wrap(board_state.column_index) {
                    board_state.switch_column(new_index);
                }
            }
        }
    }
    None
}

/// Moves the active selection right.
///
/// In Board mode, selects the column to the right.
pub fn move_right(model: &mut AppState) -> Option<Action> {
    match model.mode {
        AppMode::Picker => {}
        AppMode::Board => {
            if let Some(board_state) = &mut model.board_state {
                let current_index = board_state.column_index;
                let num_columns = board_state.board.columns.len();
                if board_state.column_list_empty() {
                    return None;
                }

                if let Some(new_index) = increment_no_wrap(current_index, num_columns) {
                    board_state.switch_column(new_index);
                }
            }
        }
    }
    None
}
