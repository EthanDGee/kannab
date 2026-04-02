use crate::io::file_handling;
use crate::message::action::Action;
use crate::model::app_state::{AppMode, AppState};

pub fn quit_to_picker(model: &mut AppState) -> Option<Action> {
    if let Some(board_state) = &model.board_state
        && model.pending_changes
    {
        file_handling::save_board(&board_state.board);
    }
    model.board_state = None;
    model.modal_state = None;
    model.mode = AppMode::Picker;
    model.pending_changes = false;
    None
}
