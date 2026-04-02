//! High-level application state and mode definitions.

pub use crate::model::board_state::BoardName;
use crate::model::modal_state::ModalState;
use crate::view::board_view::BoardState;
use crate::view::picker_view::PickerState;
use crate::view::theme::ColorScheme;

/// The various view modes available in the application.
pub enum AppMode {
    /// The board selection screen.
    Picker,
    /// The active Kanban board view.
    Board,
}

/// The root state container for the entire application session.
pub struct AppState {
    /// The current view mode (Picker or Board).
    pub mode: AppMode,
    /// List of board titles and filenames available to the picker.
    pub board_list: Vec<BoardName>,
    /// UI state for the board picker (selection index, scroll, etc.).
    pub picker_state: PickerState,
    /// UI and data state for the currently active board, if any.
    pub board_state: Option<BoardState>,
    /// UI and data state for the currently active modal, if any.
    pub modal_state: Option<ModalState>,
    /// The application's theme and styling configuration.
    pub color_scheme: ColorScheme,
    /// Flag indicating whether there are unsaved changes.
    pub pending_changes: bool,
    /// Flag indicating the application should terminate.
    pub should_quit: bool,
}

impl AppState {
    /// Creates a new `AppState` with default values and an empty board list.
    pub fn new() -> Self {
        AppState {
            mode: AppMode::Picker,
            board_list: Vec::new(),
            picker_state: PickerState::new(),
            board_state: None,
            modal_state: None,
            color_scheme: ColorScheme::default(),
            pending_changes: false,
            should_quit: false,
        }
    }
}
