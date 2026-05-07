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
    if let Some(modal) = &mut model.modal_state {
        modal.scroll_offset = modal.scroll_offset.saturating_sub(1);
        return None;
    }

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
    if let Some(modal) = &mut model.modal_state {
        modal.scroll_offset = modal.scroll_offset.saturating_add(1);
        return None;
    }

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

#[cfg(test)]
mod tests {
    use crate::message::navigation_actions::*;
    use crate::model::app_state::{AppMode, AppState};
    use crate::model::board_state::{Board, Column, Task};
    use crate::view::board_view::BoardState;

    #[test]
    fn test_increment_wrap() {
        assert_eq!(increment_wrap(0, 3), 1);
        assert_eq!(increment_wrap(2, 3), 0);
        assert_eq!(increment_wrap(0, 1), 0);
        assert_eq!(increment_wrap(0, 0), 0);
    }

    #[test]
    fn test_increment_no_wrap() {
        assert_eq!(increment_no_wrap(0, 3), Some(1));
        assert_eq!(increment_no_wrap(2, 3), None);
        assert_eq!(increment_no_wrap(0, 1), None);
    }

    #[test]
    fn test_decrement_wrap() {
        assert_eq!(decrement_wrap(1, 3), 0);
        assert_eq!(decrement_wrap(0, 3), 2);
        assert_eq!(decrement_wrap(0, 1), 0);
    }

    #[test]
    fn test_decrement_no_wrap() {
        assert_eq!(decrement_no_wrap(1), Some(0));
        assert_eq!(decrement_no_wrap(0), None);
    }

    #[test]
    fn test_move_up_down_picker() {
        let mut model = AppState::new();
        model.mode = AppMode::Picker;
        model
            .board_list
            .push(crate::model::board_state::BoardName::new("B1".to_string()));
        model
            .board_list
            .push(crate::model::board_state::BoardName::new("B2".to_string()));

        model.picker_state.index = 0;
        move_down(&mut model);
        assert_eq!(model.picker_state.index, 1);

        move_down(&mut model);
        assert_eq!(model.picker_state.index, 0); // Wraps

        move_up(&mut model);
        assert_eq!(model.picker_state.index, 1); // Wraps
    }

    #[test]
    fn test_move_up_down_board() {
        let mut model = AppState::new();
        model.mode = AppMode::Board;
        let mut board = Board::new("Test".to_string());
        let mut col = Column::new();
        col.tasks.push(Task::new());
        col.tasks.push(Task::new());
        board.columns.push(col);
        model.board_state = Some(BoardState::new(board));

        move_down(&mut model);
        assert_eq!(model.board_state.as_ref().unwrap().task_index, 1);

        move_down(&mut model);
        assert_eq!(model.board_state.as_ref().unwrap().task_index, 0); // Wraps

        move_up(&mut model);
        assert_eq!(model.board_state.as_ref().unwrap().task_index, 1); // Wraps
    }

    #[test]
    fn test_move_down_auto_create_task() {
        let mut model = AppState::new();
        model.mode = AppMode::Board;
        let mut board = Board::new("Test".to_string());
        let mut col = Column::new();
        let mut task = Task::new();
        task.title = "Not Empty".to_string();
        col.tasks.push(task);
        board.columns.push(col);
        model.board_state = Some(BoardState::new(board));

        // At index 0, length 1, not empty. Should create new task.
        let result = move_down(&mut model);
        assert!(result.is_some()); // MarkDirty
        assert_eq!(model.board_state.as_ref().unwrap().task_index, 1);
        assert_eq!(
            model.board_state.as_ref().unwrap().board.columns[0]
                .tasks
                .len(),
            2
        );
    }

    #[test]
    fn test_move_left_right_board() {
        let mut model = AppState::new();
        model.mode = AppMode::Board;
        let mut board = Board::new("Test".to_string());
        board.columns.push(Column::new());
        board.columns.push(Column::new());
        model.board_state = Some(BoardState::new(board));

        move_right(&mut model);
        assert_eq!(model.board_state.as_ref().unwrap().column_index, 1);

        move_right(&mut model); // No wrap
        assert_eq!(model.board_state.as_ref().unwrap().column_index, 1);

        move_left(&mut model);
        assert_eq!(model.board_state.as_ref().unwrap().column_index, 0);

        move_left(&mut model); // No wrap
        assert_eq!(model.board_state.as_ref().unwrap().column_index, 0);
    }
}
