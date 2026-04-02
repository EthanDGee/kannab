//! Handlers for column-level actions within a board.

use crate::message::action::Action;
use crate::message::io_actions::mark_dirty;
use crate::message::navigation_actions::{decrement_no_wrap, increment_no_wrap};
use crate::model::{app_state::AppState, board_state::Column};

/// Returns a mutable reference to the currently selected column, if a board is active.
pub fn get_current_column_mut(model: &mut AppState) -> Option<&mut Column> {
    // Extract the column index immutably first to avoid overlapping borrows
    let column_index = model.board_state.as_ref()?.column_index;
    // Now borrow mutably
    let board_state = model.board_state.as_mut()?;
    board_state.board.columns.get_mut(column_index)
}

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
    model.board_state.as_ref()?;
    let column = get_current_column_mut(model)?;
    column.title = new_name;
    mark_dirty(model)
}

/// Removes the currently selected column from the board.
pub fn delete_column(model: &mut AppState) -> Option<Action> {
    let column_index = model.board_state.as_ref()?.column_index;
    let columns_ref = model.board_state.as_mut()?.board.get_columns();
    let columns = columns_ref;
    columns.remove(column_index);
    mark_dirty(model)
}

/// Swaps the current column with the one to its left (lower index).
pub fn move_column_left(model: &mut AppState) -> Option<Action> {
    let column_index = model.board_state.as_ref()?.column_index;
    let columns_ref = model.board_state.as_mut()?.board.get_columns();
    let columns = columns_ref;

    match increment_no_wrap(column_index, columns.len()) {
        Some(new_index) => {
            columns.swap(column_index, new_index);
            mark_dirty(model)
        }
        None => None,
    }
}

/// Swaps the current column with the one to its right (higher index).
pub fn move_column_right(model: &mut AppState) -> Option<Action> {
    model.board_state.as_ref()?;
    let column_index = model.board_state.as_ref()?.column_index;
    let columns_ref = model.board_state.as_mut()?.board.get_columns();
    let columns = columns_ref;

    match decrement_no_wrap(column_index) {
        Some(new_index) => {
            columns.swap(column_index, new_index);
            mark_dirty(model)
        }
        None => None,
    }
}
