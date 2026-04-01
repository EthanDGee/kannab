use crate::{
    message::action::Action,
    model::{
        app_state::{AppMode, AppState},
        board_state::{self, Board},
    },
    view::board::BoardState,
};

/// Creates a new board, adds it to the global map of boards, then sets the view to be the new board
pub fn create_board(model: &mut AppState, title: String) -> Option<Action> {
    let board = Board::new(title.clone());

    model
        .board_map
        .insert(title.clone(), board.get_id().to_string());
    let board_state = BoardState::new(board);
    model.board_state = Some(board_state);
    model.mode = AppMode::Board;
    Some(Action::MarkDirty)
}
