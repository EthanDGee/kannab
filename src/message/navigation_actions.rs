use crate::message::action::Action;
use crate::model::app_state::{AppMode, AppState};

// ---------------------
// Navigation Utilities
// ---------------------

/// Increments the index with wrapping based on the length of the collection.
pub fn increment_wrap(index: usize, len: usize) -> usize {
    if len <= 1 {
        0
    } else if index >= len - 1 {
        0
    } else {
        index + 1
    }
}

/// Increments the index without wrapping, returning None if at the end.
pub fn increment_no_wrap(index: usize, len: usize) -> Option<usize> {
    if len == 0 || index >= len - 1 {
        None
    } else {
        Some(index + 1)
    }
}

/// Decrements the index with wrapping based on the length of the collection.
pub fn decrement_wrap(index: usize, len: usize) -> usize {
    if len <= 1 {
        0
    } else if index == 0 {
        len - 1
    } else {
        index - 1
    }
}

/// Decrements the index without wrapping, returning None if at the start.
pub fn decrement_no_wrap(index: usize) -> Option<usize> {
    if index == 0 { None } else { Some(index - 1) }
}

// ---------------------------
// Global Navigation Bindings
// ---------------------------

/// Handles the MoveUp action, navigating to the previous item.
pub fn move_up(model: &mut AppState) -> Option<Action> {
    match model.mode {
        AppMode::Picker => {
            let len = model.board_map.len();
            model.picker_state.index = decrement_wrap(model.picker_state.index, len);
        }
        AppMode::Board => {
            if let Some(board_state) = &mut model.board_state {
                let current_col = board_state.column_index;
                if let Some(column) = board_state.board.columns.get(current_col) {
                    let len = column.tasks.len();
                    board_state.task_index = decrement_wrap(board_state.task_index, len);
                }
            }
        }
    }
    None
}

/// Handles the MoveDown action, navigating to the next item.
pub fn move_down(model: &mut AppState) -> Option<Action> {
    match model.mode {
        AppMode::Picker => {
            let len = model.board_map.len();
            model.picker_state.index = increment_wrap(model.picker_state.index, len);
        }
        AppMode::Board => {
            if let Some(board_state) = &mut model.board_state {
                let current_col = board_state.column_index;
                if let Some(column) = board_state.board.columns.get(current_col) {
                    let len = column.tasks.len();
                    board_state.task_index = increment_wrap(board_state.task_index, len);
                }
            }
        }
    }
    None
}

/// Handles the MoveLeft action
pub fn move_left(model: &mut AppState) -> Option<Action> {
    match model.mode {
        AppMode::Picker => {}
        AppMode::Board => {
            if let Some(board_state) = &mut model.board_state {
                let num_columns = board_state.board.columns.len();
                if num_columns == 0 {
                    return None;
                }

                // Save current task_index
                if board_state.column_index < board_state.column_scrolls.len() {
                    board_state.column_scrolls[board_state.column_index] = board_state.task_index;
                }

                board_state.column_index = decrement_no_wrap(board_state.column_index)?;

                // Fetch new task_index
                if board_state.column_index < board_state.column_scrolls.len() {
                    board_state.task_index = board_state.column_scrolls[board_state.column_index];
                }

                // Clamp task_index to new column's task count
                let new_col = board_state.column_index;
                let num_tasks = board_state.board.columns[new_col].tasks.len();
                if num_tasks == 0 {
                    board_state.task_index = 0;
                } else if board_state.task_index >= num_tasks {
                    board_state.task_index = num_tasks - 1;
                }
            }
        }
    }
    None
}

/// Handles the MoveRight action
pub fn move_right(model: &mut AppState) -> Option<Action> {
    match model.mode {
        AppMode::Picker => {}
        AppMode::Board => {
            if let Some(board_state) = &mut model.board_state {
                let current_index = board_state.column_index;
                let num_columns = board_state.board.columns.len();
                if num_columns == 0 {
                    return None;
                }

                // Save current task_index
                if board_state.column_index < board_state.column_scrolls.len() {
                    board_state.column_scrolls[board_state.column_index] = board_state.task_index;
                }

                board_state.column_index = increment_no_wrap(current_index, num_columns)?;

                // Fetch new task_index
                if board_state.column_index < board_state.column_scrolls.len() {
                    board_state.task_index = board_state.column_scrolls[board_state.column_index];
                }

                // Clamp task_index to new column's task count
                let new_col = board_state.column_index;
                let num_tasks = board_state.board.columns[new_col].tasks.len();
                if num_tasks == 0 {
                    board_state.task_index = 0;
                } else if board_state.task_index >= num_tasks {
                    board_state.task_index = num_tasks - 1;
                }
            }
        }
    }
    None
}
