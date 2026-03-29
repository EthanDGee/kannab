use crate::message::action::Action;
use crate::message::navigation_actions::{decrement_no_wrap, increment_no_wrap};
use crate::model::{
    app_state::AppState,
    board::Column,
    modal_state::{ModalState, ModalType},
};

/// Utility function to get currently highlighted column based on AppState.BoardState's index
pub fn get_current_column_mut(model: &mut AppState) -> Option<&mut Column> {
    // Extract the column index immutably first to avoid overlapping borrows
    let column_index = model.board_state.as_ref()?.column_index;
    // Now borrow mutably
    let board_state = model.board_state.as_mut()?;
    board_state.board.columns.get_mut(column_index)
}

/// Places a new column after the current index of the selected column or if empty places it in
/// index 0
pub fn create_column(model: &mut AppState) -> Option<Action> {
    if model.board_state.is_some() {
        let new_state = ModalState::new(ModalType::CreateColumn);
        model.modal_state = Some(new_state);
    }
    None
}

/// Renames the title of the currently selected column
pub fn rename_column(model: &mut AppState, new_name: String) -> Option<Action> {
    model.board_state.as_ref()?;

    let column = get_current_column_mut(model)?;
    column.title = new_name;
    None
}

/// Delete Columns the currently selected column
pub fn delete_column(model: &mut AppState) -> Option<Action> {
    let column_index = model.board_state.as_ref()?.column_index;
    let mut columns_ref = model.board_state.as_mut()?.board.get_columns();
    let columns = columns_ref;
    columns.remove(column_index);
    None
}

/// Moves the column and the cursor to the left.
pub fn move_column_left(model: &mut AppState) -> Option<Action> {
    let column_index = model.board_state.as_ref()?.column_index;
    let columns_ref = model.board_state.as_mut()?.board.get_columns();
    let columns = columns_ref;

    match increment_no_wrap(column_index, columns.len()) {
        Some(new_index) => {
            columns.swap(column_index, new_index);
            None
        }
        None => None,
    }
}

/// Moves the column and the cursor to the right.
pub fn move_column_right(model: &mut AppState) -> Option<Action> {
    model.board_state.as_ref()?;
    let column_index = model.board_state.as_ref()?.column_index;
    let columns_ref = model.board_state.as_mut()?.board.get_columns();
    let columns = columns_ref;

    match decrement_no_wrap(column_index) {
        Some(new_index) => {
            columns.swap(column_index, new_index);
            None
        }
        None => None,
    }
}
