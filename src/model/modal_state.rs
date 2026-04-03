//! State definitions for modal overlays.

use crate::message::action::InputField;

/// Cursor position within a modal's text input fields.
#[derive(Clone, Copy, Default, PartialEq)]
pub struct CursorPosition {
    /// The character index within the current line.
    pub char_index: usize,
    /// The line index (for multi-line inputs like descriptions).
    pub line_index: usize,
}

/// The specific type and purpose of an active modal.
#[derive(Clone, PartialEq)]
pub enum ModalType {
    /// Dialog for creating a new board.
    CreateBoard,
    /// Dialog for editing board properties.
    EditBoard,
    /// Dialog for creating a new column.
    CreateColumn,
    /// Dialog for renaming an existing column.
    RenameColumn,
    /// Dialog for creating a new task.
    CreateTask,
    /// Dialog for editing an existing task.
    EditTask,
    /// Confirmation dialog before deleting an item.
    ConfirmDelete(ConfirmDelete),
    /// Help overlay showing keybindings.
    Help,
}

/// The target of a deletion confirmation.
#[derive(Copy, Clone, PartialEq)]
pub enum ConfirmDelete {
    /// Confirming board deletion.
    Board,
    /// Confirming column deletion.
    Column,
    /// Confirming task deletion.
    Task,
}

/// Data buffer for text inputs within modals.
#[derive(Clone, Default, PartialEq)]
pub struct ModalData {
    /// Buffer for board title input.
    pub board_title: String,
    /// Buffer for column title input.
    pub column_title: String,
    /// Buffer for task title input.
    pub task_title: String,
    /// Buffer for task description input.
    pub task_description: String,
    /// Buffer for checklist item description input.
    pub item_description: String,
}

/// The complete state of an active modal dialog.
#[derive(Clone, PartialEq)]
pub struct ModalState {
    /// The type of modal being displayed.
    pub modal_type: ModalType,
    /// The data being edited in the modal.
    pub data: ModalData,
    /// The current cursor position in the active field.
    pub cursor_position: CursorPosition,
    /// The currently focused input field.
    pub focus: InputField,
}

impl ModalState {
    /// Creates a new `ModalState` of the given type with default data and cursor position.
    pub fn new(modal_type: ModalType) -> Self {
        let focus = match modal_type {
            ModalType::CreateBoard | ModalType::EditBoard => InputField::BoardTitle,
            ModalType::CreateColumn | ModalType::RenameColumn => InputField::ColumnTitle,
            ModalType::CreateTask | ModalType::EditTask => InputField::TaskTitle,
            _ => InputField::BoardTitle,
        };

        ModalState {
            modal_type,
            data: ModalData::default(),
            cursor_position: CursorPosition::default(),
            focus,
        }
    }
}
