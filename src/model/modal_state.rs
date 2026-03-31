/// Position of the cursor within a modal input field
#[derive(Clone, Copy, Default, PartialEq)]
pub struct CursorPosition {
    pub char_index: usize,
    pub line_index: usize,
}

/// The various types of modals that can be displayed
#[derive(Clone, PartialEq)]
pub enum ModalType {
    CreateBoard,
    EditBoard,
    CreateColumn,
    RenameColumn,
    CreateTask,
    EditTask,
    ConfirmDelete(ConfirmDelete),
    Search,
    Help,
}

#[derive(Clone, PartialEq)]
pub enum ConfirmDelete {
    Board,
    Column,
    Task,
}

#[derive(Clone, Default, PartialEq)]
pub struct ModalData {
    pub board_name: String,
    pub column_name: String,
    pub task_title: String,
    pub task_description: String,
    pub item_title: String,
}

#[derive(Clone, PartialEq)]
pub struct ModalState {
    pub modal_type: ModalType,
    pub data: ModalData,
    pub cursor_position: CursorPosition,
}

impl ModalState {
    pub fn new(modal_type: ModalType) -> Self {
        ModalState {
            modal_type,
            data: ModalData::default(),
            cursor_position: CursorPosition::default(),
        }
    }
}
