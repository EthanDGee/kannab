use crate::{io::file_handling, message::action::Action, model::app_state::AppState};

pub fn mark_dirty(model: &mut AppState) -> Option<Action> {
    model.pending_changes = true;
    None
}

pub fn handle_tick(model: &mut AppState) -> Option<Action> {
    if model.pending_changes {
        Some(Action::Save)
    } else {
        None
    }
}

pub fn save(model: &mut AppState) -> Option<Action> {
    if let Some(board_state) = &model.board_state {
        file_handling::save_board(&board_state.board);
    }
    file_handling::save_board_list(&model.board_list);
    model.pending_changes = false;
    None
}

pub fn quit(model: &mut AppState) -> Option<Action> {
    model.should_quit = true;
    Some(Action::Save)
}
