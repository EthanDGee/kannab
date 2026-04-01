use crate::io::file_handling;
use crate::message::action::Action;
use crate::message::navigation_actions::{decrement_no_wrap, increment_no_wrap};
use crate::model::app_state::{AppMode, AppState};
use crate::model::board_state::{Board, BoardName};
use crate::view::board_view::BoardState;

// Board Handling

/// Creates a new board, adds it to the global map of boards, then sets the view to be the new board
pub fn create_board(model: &mut AppState, title: String) -> Option<Action> {
    let board = Board::new(title.clone());

    model.board_list.insert(0, BoardName::new(title.clone()));
    let board_state = BoardState::new(board);
    file_handling::save_board_list(&model.board_list);
    model.board_state = Some(board_state);
    model.mode = AppMode::Board;
    Some(Action::MarkDirty)
}

/// Opens the currently selected board from the picker
pub fn open_board(model: &mut AppState) -> Option<Action> {
    let index = model.picker_state.index;
    let board_name = model.board_list.get(index)?;
    let board = file_handling::load_board(&board_name.title)?;

    model.board_state = Some(BoardState::new(board));
    model.mode = AppMode::Board;
    None
}

/// Deletes the currently selected board
pub fn delete_board(model: &mut AppState) -> Option<Action> {
    let index = model.picker_state.index;
    if index >= model.board_list.len() {
        return None;
    }

    let board_name = model.board_list.remove(index);
    file_handling::delete_board(&board_name.title);
    file_handling::save_board_list(&model.board_list);

    // update picker index as needed to ensure that is at valid position
    if model.picker_state.index >= model.board_list.len() && !model.board_list.is_empty() {
        model.picker_state.index = model.board_list.len() - 1;
    } else if model.board_list.is_empty() {
        model.picker_state.index = 0;
    }

    Some(Action::MarkDirty)
}

/// Renames the currently selected board in the picker
pub fn rename_board(model: &mut AppState, new_title: String) -> Option<Action> {
    let index = model.picker_state.index;
    let board_name_entry = model.board_list.get_mut(index)?;
    let old_title = board_name_entry.title.clone();

    // 1. Handle file renaming on disk
    if let Some(mut board) = file_handling::load_board(&old_title) {
        file_handling::delete_board(&old_title);
        board.title = new_title.clone();
        board.file_name = file_handling::to_snake_case(new_title.clone());
        file_handling::save_board(&board);
    }

    // 2. Update board_list entry
    *board_name_entry = BoardName::new(new_title.clone());
    file_handling::save_board_list(&model.board_list);

    // 3. Update active board state if it's the same board
    if let Some(board_state) = &mut model.board_state
        && board_state.board.title == old_title
    {
        board_state.board.title = new_title;
    }

    Some(Action::MarkDirty)
}

// Board List Handling

pub fn move_board_up(model: &mut AppState) -> Option<Action> {
    let current_index = model.picker_state.index;
    let new_index = decrement_no_wrap(current_index)?;
    model.board_list.swap(current_index, new_index);
    model.picker_state.index = new_index;

    save_board_list(model);
    None
}

pub fn move_board_down(model: &mut AppState) -> Option<Action> {
    let current_index = model.picker_state.index;
    let max = model.board_list.len();
    let new_index = increment_no_wrap(current_index, max)?;
    model.board_list.swap(current_index, new_index);
    model.picker_state.index = new_index;

    save_board_list(model);
    None
}

/// Saves the current board state to the file system
pub fn save_board(model: &mut AppState) -> Option<Action> {
    if let Some(board_state) = &model.board_state {
        file_handling::save_board(&board_state.board);
    }
    None
}

/// Saves the current list of boards to the file system
pub fn save_board_list(model: &mut AppState) -> Option<Action> {
    file_handling::save_board_list(&model.board_list);
    None
}
