//! Handlers for actions specifically within the board picker.

use crate::io::file_handling;
use crate::message::action::Action;
use crate::model::app_state::{AppMode, AppState};

/// Exits the active board and returns to the picker view, saving any unsaved board changes.
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

#[cfg(test)]
mod tests {
    use crate::message::picker_actions::*;
    use crate::model::board_state::Board;
    use crate::view::board_view::BoardState;

    #[test]
    fn test_quit_to_picker() {
        let mut model = AppState::new();
        model.mode = AppMode::Board;
        model.board_state = Some(BoardState::new(Board::new("T".to_string())));
        model.pending_changes = true;

        quit_to_picker(&mut model);

        assert!(matches!(model.mode, AppMode::Picker));
        assert!(model.board_state.is_none());
        assert!(!model.pending_changes);
    }
}
