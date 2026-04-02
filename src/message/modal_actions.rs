//! Handlers for modal-related actions and user input.

use crate::message::action::{Action, InputField};
use crate::model::{
    app_state::AppState,
    modal_state::{ModalState, ModalType},
};

/// Initializes and opens a modal of the specified type.
pub fn open_modal(model: &mut AppState, modal_type: ModalType) -> Option<Action> {
    model.modal_state = Some(ModalState::new(modal_type));
    None
}

/// Discards any active modal state.
pub fn close_modal(model: &mut AppState) -> Option<Action> {
    model.modal_state = None;
    None
}

/// Toggles focus between title and description fields in a task-related modal.
pub fn switch_input_field(model: &mut AppState) -> Option<Action> {
    if let Some(modal) = &mut model.modal_state {
        modal.focus = match modal.focus {
            InputField::TaskTitle => InputField::TaskDescription,
            _ => InputField::TaskTitle,
        };

        // Reset cursor position to end of the new field's content
        let content = match modal.focus {
            InputField::TaskTitle => &modal.data.task_title,
            InputField::TaskDescription => &modal.data.task_description,
            _ => "",
        };

        modal.cursor_position.char_index = content.chars().count();
        modal.cursor_position.line_index = 0; // Simple for now
    }
    None
}

/// Updates the transient value of a specific input field within the active modal.
pub fn update_field(model: &mut AppState, field: InputField, value: String) -> Option<Action> {
    let mut new_cursor_pos = None;
    if let Some(modal) = &mut model.modal_state {
        // Calculate cursor position based on the end of the text
        let lines: Vec<&str> = value.split('\n').collect();
        let line_index = lines.len().saturating_sub(1);
        let char_index = lines.last().map_or(0, |l| l.chars().count());
        new_cursor_pos = Some((char_index, line_index));

        match field {
            InputField::BoardName => modal.data.board_name = value,
            InputField::ColumnName => modal.data.column_name = value,
            InputField::TaskTitle => modal.data.task_title = value,
            InputField::TaskDescription => modal.data.task_description = value,
            InputField::TaskItem => {} // Handle if checklists are implemented
        }
    }

    new_cursor_pos.map(|(x, y)| Action::MoveCursor(x, y))
}

/// Updates the cursor's character and line position in the active modal.
pub fn move_cursor(model: &mut AppState, x: usize, y: usize) -> Option<Action> {
    if let Some(modal) = &mut model.modal_state {
        modal.cursor_position.char_index = x;
        modal.cursor_position.line_index = y;
    }
    None
}

/// Processes the 'Confirm' action for the active modal, returning the resulting functional action.
///
/// For example, confirming a `CreateBoard` modal returns a `CreateBoard(name)` action.
pub fn confirm(model: &mut AppState) -> Option<Action> {
    let modal = model.modal_state.as_ref()?;
    let action = match &modal.modal_type {
        ModalType::CreateBoard => {
            let name = modal.data.board_name.clone();
            Some(Action::CreateBoard(name))
        }
        ModalType::CreateColumn => {
            let name = modal.data.column_name.clone();
            Some(Action::CreateColumn(name))
        }
        ModalType::CreateTask => {
            let title = modal.data.task_title.clone();
            let description = modal.data.task_description.clone();
            Some(Action::CreateTask(title, description))
        }
        ModalType::ConfirmDelete(target) => match target {
            crate::model::modal_state::ConfirmDelete::Board => Some(Action::DeleteBoard),
            crate::model::modal_state::ConfirmDelete::Column => Some(Action::DeleteColumn),
            crate::model::modal_state::ConfirmDelete::Task => Some(Action::DeleteTask),
        },
        _ => None,
    };

    model.modal_state = None;
    action
}

/// Cancels the current modal operation, discards any data, and closes it.
pub fn cancel(model: &mut AppState) -> Option<Action> {
    model.modal_state = None;
    None
}
