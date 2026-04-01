use crate::model::modal_state::ModalState;
use crate::view::board::BoardState;
use crate::view::picker::PickerState;
use crate::view::theme::ColorScheme;

/// enum containing the various screens
pub enum AppMode {
    Picker,
    Board,
}

/// Stores the title of a board and it's associated file name
pub struct BoardName {
    pub title: String,
    pub snake_case: String,
}


impl BoardName {
    pub fn new(title: String) -> Self {
        let camel_case = Self::to_snake_case(&title);

        BoardName {
            title,
            snake_case: camel_case,
        }
    }

    pub fn to_snake_case(s: &str) -> String {
        s.split_whitespace()
            .map(|word| word.to_lowercase())
            .collect::<Vec<_>>()
            .join("_")
    }
}
/// Handles all state logic
///
/// mode: AppMode,
/// board_map : <BoardName> - A list of board names with their title and snake case
/// Picker State,
/// picker_state : PickerState,
/// board_state : Option<BoardState>,
/// modal : Option<ModalState>, - pop up windows
/// pending_changes : bool  -  If there any changes that are currently unsaved,
pub struct AppState {
    pub mode: AppMode,
    pub board_list: Vec<BoardName>,
    pub picker_state: PickerState,
    pub board_state: Option<BoardState>,
    pub modal_state: Option<ModalState>,
    pub color_scheme: ColorScheme,
    pub pending_changes: bool,
}

impl AppState {
    // TODO: Implement board loading
    pub fn new() -> Self {
        AppState {
            mode: AppMode::Picker,
            board_list: Vec::new(),
            picker_state: PickerState::new(),
            board_state: None,
            modal_state: None,
            color_scheme: ColorScheme::default(),
            pending_changes: false,
        }
    }
}
