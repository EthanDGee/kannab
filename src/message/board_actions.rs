//! Handlers for board-level actions.

use crate::io::file_handling;
use crate::message::action::Action;
use crate::message::navigation_actions::{decrement_no_wrap, increment_no_wrap};
use crate::model::app_state::{AppMode, AppState};
use crate::model::board_state::{Board, BoardName};
use crate::view::board_view::BoardState;

/// Creates a new board with the specified title and sets it as active.
pub fn create_board(model: &mut AppState, title: String) -> Option<Action> {
    let board = Board::new(title.clone());

    model.board_list.insert(0, BoardName::new(title.clone()));
    let board_state = BoardState::new(board);
    model.board_state = Some(board_state);
    model.mode = AppMode::Board;
    Some(Action::MarkDirty)
}

/// Loads and opens the board selected in the board picker.
pub fn open_board(model: &mut AppState) -> Option<Action> {
    let index = model.picker_state.index;
    let board_name = model.board_list.get(index)?;
    let board = file_handling::load_board(&board_name.title)?;

    model.board_state = Some(BoardState::new(board));
    model.mode = AppMode::Board;
    None
}

/// Deletes the currently selected board from the picker and deletes its file.
pub fn delete_board(model: &mut AppState) -> Option<Action> {
    let index = model.picker_state.index;
    if index >= model.board_list.len() {
        return None;
    }

    let board_name = model.board_list.remove(index);
    file_handling::delete_board(&board_name.title);

    // update picker index as needed to ensure that is at valid position
    if model.picker_state.index >= model.board_list.len() && !model.board_list.is_empty() {
        model.picker_state.index = model.board_list.len() - 1;
    } else if model.board_list.is_empty() {
        model.picker_state.index = 0;
    }

    Some(Action::MarkDirty)
}

/// Renames a board in the picker and updates its filename on disk.
pub fn rename_board(model: &mut AppState, new_title: String) -> Option<Action> {
    let index = model.picker_state.index;
    let board_name_entry = model.board_list.get_mut(index)?;
    let old_title = board_name_entry.title.clone();

    // 1. Update board_list entry
    *board_name_entry = BoardName::new(new_title.clone());

    // 2. Handle file renaming and state update
    let is_active = if let Some(board_state) = &mut model.board_state
        && board_state.board.title == old_title
    {
        board_state.board.title = new_title.clone();
        board_state.board.file_name = file_handling::to_snake_case(new_title.clone());
        file_handling::save_board(&board_state.board);
        true
    } else {
        false
    };

    if let Some(mut board) = file_handling::load_board(&old_title)
        && is_active
    {
        board.title = new_title.clone();
        board.file_name = file_handling::to_snake_case(new_title);
        file_handling::save_board(&board);
    }

    // 3. Delete the old file
    file_handling::delete_board(&old_title);

    Some(Action::MarkDirty)
}

/// Reorders the board list by swapping the selected board with the one above it.
pub fn move_board_up(model: &mut AppState) -> Option<Action> {
    let current_index = model.picker_state.index;
    let new_index = decrement_no_wrap(current_index)?;
    model.board_list.swap(current_index, new_index);
    model.picker_state.index = new_index;

    Some(Action::MarkDirty)
}

/// Reorders the board list by swapping the selected board with the one below it.
pub fn move_board_down(model: &mut AppState) -> Option<Action> {
    let current_index = model.picker_state.index;
    let max = model.board_list.len();
    let new_index = increment_no_wrap(current_index, max)?;
    model.board_list.swap(current_index, new_index);
    model.picker_state.index = new_index;

    Some(Action::MarkDirty)
}
