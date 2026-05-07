//! State definitions for modal overlays.

use crate::message::action::InputField;
use ratatui_textarea::TextArea;

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
    /// Working copy of checklist items for a task.
    pub checklist: Vec<crate::model::board_state::Item>,
}

/// The complete state of an active modal dialog.
#[derive(Clone)]
pub struct ModalState {
    /// The type of modal being displayed.
    pub modal_type: ModalType,
    /// The data being edited in the modal.
    pub data: ModalData,
    /// The active text area for the currently focused field.
    pub active_textarea: TextArea<'static>,
    /// The currently focused input field.
    pub focus: InputField,
    /// Index of the currently focused checklist item.
    pub item_index: usize,
    /// Scroll offset for the checklist items.
    pub scroll_offset: usize,
}

impl PartialEq for ModalState {
    fn eq(&self, other: &Self) -> bool {
        self.modal_type == other.modal_type
            && self.data == other.data
            && self.focus == other.focus
            && self.item_index == other.item_index
            && self.scroll_offset == other.scroll_offset
        // We skip active_textarea as it doesn't implement PartialEq
    }
}

impl ModalState {
    /// Creates a new `ModalState` of the given type with default data.
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
            active_textarea: TextArea::default(),
            focus,
            item_index: 0,
            scroll_offset: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modal_state_new() {
        let ms = ModalState::new(ModalType::CreateTask);
        assert!(matches!(ms.modal_type, ModalType::CreateTask));
        assert_eq!(ms.focus, InputField::TaskTitle);
        assert_eq!(ms.item_index, 0);
    }
}
