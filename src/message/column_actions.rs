//! Handlers for column-level actions within a board.

use crate::message::action::Action;
use crate::message::io_actions::mark_dirty;
use crate::message::navigation_actions::{decrement_no_wrap, increment_no_wrap};
use crate::model::app_state::AppState;
use crate::model::board_state::Column;

// TODO: create_column should insert column after the current column index

/// Appends a new column with the specified title to the current board.
pub fn create_column(model: &mut AppState, title: String) -> Option<Action> {
    if let Some(board_state) = &mut model.board_state {
        let mut column = Column::new();
        column.title = title;
        board_state.board.columns.push(column);
        board_state.column_scrolls.push(0);
        mark_dirty(model)
    } else {
        None
    }
}

/// Updates the title of the currently selected column.
pub fn rename_column(model: &mut AppState, new_name: String) -> Option<Action> {
    let board_state = model.board_state.as_mut()?;
    if board_state.column_list_empty() {
        return None;
    }
    let column_index = board_state.column_index;
    let column = board_state.board.get_column_mut(column_index)?;
    column.title = new_name;
    mark_dirty(model)
}

/// Removes the currently selected column from the board.
pub fn delete_column(model: &mut AppState) -> Option<Action> {
    let board_state = model.board_state.as_mut()?;
    let column_index = board_state.column_index;

    if board_state.column_list_empty() {
        return None;
    }

    board_state.board.remove_column(column_index);

    // Keep column_scrolls in sync with columns
    if column_index < board_state.column_scrolls.len() {
        board_state.column_scrolls.remove(column_index);
    }

    let has_columns = !board_state.column_list_empty();

    // Adjust column_index if it's now out of bounds
    if has_columns && board_state.column_index >= board_state.board.columns.len() {
        board_state.column_index = board_state.board.columns.len() - 1;
    } else if !has_columns {
        board_state.column_index = 0;
    }

    // Update task_index for the new column
    board_state.task_index = *board_state
        .column_scrolls
        .get(board_state.column_index)
        .unwrap_or(&0);

    mark_dirty(model)
}

/// Swaps the current column with the one to its left (lower index).
pub fn move_column_left(model: &mut AppState) -> Option<Action> {
    let board_state = model.board_state.as_mut()?;
    if board_state.column_list_empty() {
        return None;
    }
    let column_index = board_state.column_index;

    match decrement_no_wrap(column_index) {
        Some(new_index) => {
            board_state.board.swap_columns(column_index, new_index);
            board_state.column_index = new_index;
            mark_dirty(model)
        }
        None => None,
    }
}

/// Swaps the current column with the one to its right (higher index).
pub fn move_column_right(model: &mut AppState) -> Option<Action> {
    let board_state = model.board_state.as_mut()?;
    if board_state.column_list_empty() {
        return None;
    }
    let column_index = board_state.column_index;
    let num_columns = board_state.board.columns.len();

    match increment_no_wrap(column_index, num_columns) {
        Some(new_index) => {
            board_state.board.swap_columns(column_index, new_index);
            board_state.column_index = new_index;
            mark_dirty(model)
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::message::column_actions::*;
    use crate::model::app_state::AppState;
    use crate::model::board_state::Board;
    use crate::view::board_view::BoardState;

    fn setup_test_state() -> AppState {
        let mut model = AppState::new();
        let board = Board::new("Test Board".to_string());
        model.board_state = Some(BoardState::new(board));
        model
    }

    #[test]
    fn test_create_column() {
        let mut model = setup_test_state();
        create_column(&mut model, "Todo".to_string());

        let bs = model.board_state.as_ref().unwrap();
        assert_eq!(bs.board.columns.len(), 1);
        assert_eq!(bs.board.columns[0].title, "Todo");
        assert_eq!(bs.column_scrolls.len(), 1);
    }

    #[test]
    fn test_rename_column() {
        let mut model = setup_test_state();
        create_column(&mut model, "Old".to_string());
        rename_column(&mut model, "New".to_string());

        let bs = model.board_state.as_ref().unwrap();
        assert_eq!(bs.board.columns[0].title, "New");
    }

    #[test]
    fn test_delete_column() {
        let mut model = setup_test_state();
        create_column(&mut model, "C1".to_string());
        create_column(&mut model, "C2".to_string());

        model.board_state.as_mut().unwrap().column_index = 1;
        delete_column(&mut model);

        let bs = model.board_state.as_ref().unwrap();
        assert_eq!(bs.board.columns.len(), 1);
        assert_eq!(bs.column_index, 0);
        assert_eq!(bs.column_scrolls.len(), 1);
    }

    #[test]
    fn test_move_column_left_right() {
        let mut model = setup_test_state();
        create_column(&mut model, "C1".to_string());
        create_column(&mut model, "C2".to_string());

        // At C2 (index 1)
        model.board_state.as_mut().unwrap().column_index = 1;
        move_column_left(&mut model);
        assert_eq!(model.board_state.as_ref().unwrap().column_index, 0);
        assert_eq!(
            model.board_state.as_ref().unwrap().board.columns[0].title,
            "C2"
        );

        move_column_right(&mut model);
        assert_eq!(model.board_state.as_ref().unwrap().column_index, 1);
        assert_eq!(
            model.board_state.as_ref().unwrap().board.columns[1].title,
            "C2"
        );
    }
}
