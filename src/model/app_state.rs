use crate::message;
use crate::message::action::Action;
use crate::model::modal_state::ModalState;
use crate::view::board::BoardState;
use crate::view::picker::PickerState;
use std::collections::HashMap;

/// enum containing the various screens
pub enum AppMode {
    Picker,
    Board,
}

/// Handles all state logic
///
/// mode: AppMode,
/// board_map : HashMap<&str, &str> - A map of board names to their file paths for use by the
/// Picker State,
/// picker_state : PickerState,
/// board_state : Option<BoardState>,
/// modal : Option<ModalState>, - pop up windows
/// pending_changes : bool  -  If there any changes that are currently unsaved,
pub struct AppState {
    pub mode: AppMode,
    pub board_map: HashMap<String, String>,
    pub picker_state: PickerState,
    pub board_state: Option<BoardState>,
    pub modal_state: Option<ModalState>,
    pub pending_changes: bool,
}

impl AppState {
    // TODO: Implement board loading
    pub fn new() -> Self {
        AppState {
            mode: AppMode::Picker,
            board_map: HashMap::new(),
            picker_state: PickerState::new(),
            board_state: None,
            modal_state: None,
            pending_changes: false,
        }
    }
}
