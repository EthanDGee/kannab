//! Handlers for modal-related actions and user input.

use crate::message::action::{Action, InputField};
use crate::model::{
    app_state::AppState,
    modal_state::{ModalState, ModalType},
};

/// Initializes and opens a modal of the specified type.
pub fn open_modal(model: &mut AppState, modal_type: ModalType) -> Option<Action> {
    let mut modal_state = ModalState::new(modal_type.clone());

    match &modal_type {
        ModalType::RenameColumn => {
            if let Some(board_state) = &model.board_state
                && let Some(column) = board_state.current_column()
            {
                modal_state.data.column_title = column.title.clone();
                modal_state.cursor_position.char_index =
                    modal_state.data.column_title.chars().count();
            }
        }
        ModalType::EditBoard => {
            if let Some(board) = model.board_list.get(model.picker_state.index) {
                modal_state.data.board_title = board.title.clone();
                modal_state.cursor_position.char_index =
                    modal_state.data.board_title.chars().count();
            }
        }
        ModalType::CreateTask => {
            if let Some(board_state) = &model.board_state
                && board_state.column_list_empty()
            {
                return None;
            }
        }
        ModalType::ConfirmDelete(confirm_delete) => match confirm_delete {
            crate::model::modal_state::ConfirmDelete::Board => {
                if model.board_list_empty() {
                    return None;
                }
            }
            crate::model::modal_state::ConfirmDelete::Column => {
                if let Some(board_state) = &model.board_state {
                    if board_state.column_list_empty()
                        || board_state.column_index >= board_state.board.columns.len()
                    {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            crate::model::modal_state::ConfirmDelete::Task => {
                if let Some(board_state) = &model.board_state {
                    let column = board_state.current_column()?;
                    if board_state.task_index >= column.tasks.len() || column.task_list_empty() {
                        return None;
                    }
                } else {
                    return None;
                }
            }
        },
        ModalType::EditTask => {
            if let Some(board_state) = &model.board_state
                && let Some(task) = board_state.current_task()
            {
                modal_state.data.task_title = task.title.clone();
                modal_state.data.task_description = task.description.clone();
                modal_state.data.checklist = task.checklist.clone();
                modal_state.cursor_position.char_index =
                    modal_state.data.task_title.chars().count();
            }
        }
        _ => {}
    }

    model.modal_state = Some(modal_state);
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
            InputField::TaskDescription => {
                modal.item_index = 0;
                InputField::ItemDescription
            }
            InputField::ItemDescription => {
                // check if the next index is out of range of items
                // then increment to next item or switch to task description
                let item_count = modal.data.checklist.len();

                // If we are at the end of the checklist and have text, append it and move to a new empty item
                if modal.item_index == item_count && !modal.data.item_description.is_empty() {
                    let mut new_item = crate::model::board_state::Item::new();
                    new_item.description = modal.data.item_description.clone();
                    modal.data.checklist.push(new_item);
                    modal.data.item_description.clear();
                    modal.item_index += 1;
                    InputField::ItemDescription
                } else if modal.item_index + 1 > item_count {
                    InputField::TaskTitle
                } else {
                    modal.item_index += 1;
                    InputField::ItemDescription
                }
            }

            _ => InputField::TaskTitle,
        };

        // Reset cursor position to end of the new field's content
        let char_count = match modal.focus {
            InputField::TaskTitle => modal.data.task_title.chars().count(),
            InputField::TaskDescription => modal.data.task_description.chars().count(),
            InputField::ItemDescription => {
                if modal.item_index < modal.data.checklist.len() {
                    modal.data.checklist[modal.item_index]
                        .description
                        .chars()
                        .count()
                } else {
                    modal.data.item_description.chars().count()
                }
            }
            _ => 0,
        };

        modal.cursor_position.char_index = char_count;
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
            InputField::BoardTitle => modal.data.board_title = value,
            InputField::ColumnTitle => modal.data.column_title = value,
            InputField::TaskTitle => modal.data.task_title = value,
            InputField::TaskDescription => modal.data.task_description = value,
            InputField::ItemDescription => {
                if modal.item_index < modal.data.checklist.len() {
                    modal.data.checklist[modal.item_index].description = value;
                } else {
                    modal.data.item_description = value;
                }
            }
        }
    }

    new_cursor_pos.map(|(x, y)| Action::MoveCursor(x, y))
}

/// Removes the currently selected checklist item.
pub fn delete_checklist_item(model: &mut AppState) -> Option<Action> {
    if let Some(modal) = &mut model.modal_state
        && modal.focus == InputField::ItemDescription
    {
        if modal.item_index < modal.data.checklist.len() {
            modal.data.checklist.remove(modal.item_index);
            // Adjust index if we deleted the last item and it wasn't the only one
            if modal.item_index >= modal.data.checklist.len() && modal.item_index > 0 {
                modal.item_index -= 1;
            }
            return Some(Action::MarkDirty);
        } else {
            // We are on the "new item" field, just clear it
            modal.data.item_description.clear();
        }
    }
    None
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
            let name = modal.data.board_title.clone();
            Some(Action::CreateBoard(name))
        }
        ModalType::CreateColumn => {
            let name = modal.data.column_title.clone();
            Some(Action::CreateColumn(name))
        }
        ModalType::RenameColumn => {
            let name = modal.data.column_title.clone();
            Some(Action::RenameColumn(name))
        }
        ModalType::EditBoard => {
            let name = modal.data.board_title.clone();
            Some(Action::RenameBoard(name))
        }
        ModalType::CreateTask => {
            let (title, description, checklist) = finalize_task_data(modal);
            Some(Action::CreateTask(title, description, checklist))
        }
        ModalType::EditTask => {
            let (title, description, checklist) = finalize_task_data(modal);
            Some(Action::EditTask(title, description, checklist))
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

/// Extracts and prepares task data from the modal state, including appending any new checklist item.
fn finalize_task_data(
    modal: &ModalState,
) -> (String, String, Vec<crate::model::board_state::Item>) {
    let mut checklist = modal.data.checklist.clone();
    if !modal.data.item_description.is_empty() {
        let mut new_item = crate::model::board_state::Item::new();
        new_item.description = modal.data.item_description.clone();
        checklist.push(new_item);
    }
    (
        modal.data.task_title.clone(),
        modal.data.task_description.clone(),
        checklist,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::action::InputField;
    use crate::model::app_state::AppState;
    use crate::model::board_state::Item;
    use crate::model::modal_state::{ModalState, ModalType};

    #[test]
    fn test_delete_checklist_item() {
        let mut model = AppState::new();
        let mut modal_state = ModalState::new(ModalType::EditTask);

        // Add some items
        let mut item1 = Item::new();
        item1.description = "Item 1".to_string();
        let mut item2 = Item::new();
        item2.description = "Item 2".to_string();

        modal_state.data.checklist.push(item1);
        modal_state.data.checklist.push(item2);
        modal_state.focus = InputField::ItemDescription;
        modal_state.item_index = 0;

        model.modal_state = Some(modal_state);

        // Delete first item
        let action = delete_checklist_item(&mut model);
        assert!(matches!(action, Some(Action::MarkDirty)));

        let modal = model.modal_state.as_ref().unwrap();
        assert_eq!(modal.data.checklist.len(), 1);
        assert_eq!(modal.data.checklist[0].description, "Item 2");
        assert_eq!(modal.item_index, 0);

        // Delete remaining item
        delete_checklist_item(&mut model);
        let modal = model.modal_state.as_ref().unwrap();
        assert_eq!(modal.data.checklist.len(), 0);
        assert_eq!(modal.item_index, 0);
    }

    #[test]
    fn test_delete_new_item_clears_description() {
        let mut model = AppState::new();
        let mut modal_state = ModalState::new(ModalType::EditTask);

        modal_state.data.item_description = "New Item".to_string();
        modal_state.focus = InputField::ItemDescription;
        modal_state.item_index = 0; // Index 0 when checklist is empty is the "new item" field

        model.modal_state = Some(modal_state);

        let action = delete_checklist_item(&mut model);
        assert!(action.is_none());

        let modal = model.modal_state.as_ref().unwrap();
        assert_eq!(modal.data.item_description, "");
    }
}
