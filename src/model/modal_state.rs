struct CursorPosition {
    char_index: usize,
    line_index: usize,
}

impl Default for CursorPosition {
    fn default() -> Self {
        CursorPosition {
            char_index: 0,
            line_index: 0,
        }
    }
}

pub enum ModalType {
    CreateBoard,
    EditBoard,
    CreateColumn,
    RenameColumn,
    CreateTask,
    EditTask,
    // ConfirmDelete(ConfirmTarget), Post-MVP
    // Search, Post-MVP
}

pub struct ModalState {
    pub modal_type: ModalType,
    // pub data: ModalData,
    pub cursor_position: CursorPosition,
}

impl ModalState {
    pub fn new(modal_type: ModalType) -> Self {
        ModalState {
            modal_type,
            cursor_position: CursorPosition::default(),
        }
    }
}
