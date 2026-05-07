//! Handlers for modal-related actions and user input.

use crate::message::action::{Action, InputField};
use crate::model::app_state::AppState;
use crate::model::modal_state::{ModalState, ModalType};
use ratatui_textarea::{CursorMove, TextArea, WrapMode};

/// Initializes and opens a modal of the specified type.
pub fn open_modal(model: &mut AppState, modal_type: ModalType) -> Option<Action> {
    let mut modal_state = ModalState::new(modal_type.clone());

    match &modal_type {
        ModalType::RenameColumn => {
            if let Some(board_state) = &model.board_state
                && let Some(column) = board_state.current_column()
            {
                modal_state.data.column_title = column.title.clone();
            }
        }
        ModalType::EditBoard => {
            if let Some(board) = model.board_list.get(model.picker_state.index) {
                modal_state.data.board_title = board.title.clone();
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
            }
        }
        _ => {}
    }

    load_active_textarea(&mut modal_state);
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
        sync_active_textarea(modal);

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

        load_active_textarea(modal);
    }
    None
}

/// Helper to sync the content of the active TextArea back to the ModalData.
fn sync_active_textarea(modal: &mut ModalState) {
    let text = modal.active_textarea.lines().join("\n");
    match modal.focus {
        InputField::BoardTitle => modal.data.board_title = text,
        InputField::ColumnTitle => modal.data.column_title = text,
        InputField::TaskTitle => modal.data.task_title = text,
        InputField::TaskDescription => modal.data.task_description = text,
        InputField::ItemDescription => {
            if modal.item_index < modal.data.checklist.len() {
                modal.data.checklist[modal.item_index].description = text;
            } else {
                modal.data.item_description = text;
            }
        }
    }
}

/// Helper to load the current field's content from ModalData into the active TextArea.
fn load_active_textarea(modal: &mut ModalState) {
    let text = match modal.focus {
        InputField::BoardTitle => &modal.data.board_title,
        InputField::ColumnTitle => &modal.data.column_title,
        InputField::TaskTitle => &modal.data.task_title,
        InputField::TaskDescription => &modal.data.task_description,
        InputField::ItemDescription => {
            if modal.item_index < modal.data.checklist.len() {
                &modal.data.checklist[modal.item_index].description
            } else {
                &modal.data.item_description
            }
        }
    };
    modal.active_textarea = TextArea::from(text.lines());
    if modal.focus == InputField::TaskDescription {
        modal.active_textarea.set_wrap_mode(WrapMode::Word);
    }
    modal.active_textarea.move_cursor(CursorMove::End);
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
            load_active_textarea(modal);
            return Some(Action::MarkDirty);
        } else {
            // We are on the "new item" field, just clear it
            modal.data.item_description.clear();
            load_active_textarea(modal);
        }
    }
    None
}

/// Processes the 'Confirm' action for the active modal, returning the resulting functional action.
///
/// For example, confirming a `CreateBoard` modal returns a `CreateBoard(name)` action.
pub fn confirm(model: &mut AppState) -> Option<Action> {
    if let Some(modal) = &mut model.modal_state {
        sync_active_textarea(modal);
    }

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
    use crate::message::action::InputField;
    use crate::message::modal_actions::*;
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

    #[test]
    fn test_open_modal_create_board() {
        let mut model = AppState::new();
        open_modal(&mut model, ModalType::CreateBoard);

        let modal = model.modal_state.as_ref().unwrap();
        assert!(matches!(modal.modal_type, ModalType::CreateBoard));
        assert_eq!(modal.focus, InputField::BoardTitle);
    }

    #[test]
    fn test_switch_input_field_task() {
        let mut model = AppState::new();
        let mut modal = ModalState::new(ModalType::CreateTask);
        modal.focus = InputField::TaskTitle;
        model.modal_state = Some(modal);

        switch_input_field(&mut model);
        assert_eq!(
            model.modal_state.as_ref().unwrap().focus,
            InputField::TaskDescription
        );

        switch_input_field(&mut model);
        assert_eq!(
            model.modal_state.as_ref().unwrap().focus,
            InputField::ItemDescription
        );
    }

    #[test]
    fn test_confirm_create_board() {
        let mut model = AppState::new();
        let mut modal = ModalState::new(ModalType::CreateBoard);
        modal.active_textarea = TextArea::from(["New Board"]);
        model.modal_state = Some(modal);

        let action = confirm(&mut model);

        assert!(matches!(action, Some(Action::CreateBoard(name)) if name == "New Board"));
        assert!(model.modal_state.is_none());
    }

    #[test]
    fn test_finalize_task_data() {
        let mut modal = ModalState::new(ModalType::CreateTask);
        modal.data.task_title = "T".to_string();
        modal.data.task_description = "D".to_string();

        let mut item = Item::new();
        item.description = "Item 1".to_string();
        modal.data.checklist.push(item);
        modal.data.item_description = "New Item".to_string();

        let (title, desc, checklist) = finalize_task_data(&modal);
        assert_eq!(title, "T");
        assert_eq!(desc, "D");
        assert_eq!(checklist.len(), 2);
        assert_eq!(checklist[1].description, "New Item");
    }

    #[test]
    fn test_switch_input_field_auto_append() {
        let mut model = AppState::new();
        let mut modal = ModalState::new(ModalType::CreateTask);
        modal.focus = InputField::ItemDescription;
        modal.item_index = 0;
        modal.data.item_description = "Auto Append".to_string();
        load_active_textarea(&mut modal); // Load into active_textarea so sync works
        model.modal_state = Some(modal);

        // Switch should append "Auto Append" to checklist and move to next item slot
        switch_input_field(&mut model);

        let modal = model.modal_state.as_ref().unwrap();
        assert_eq!(modal.data.checklist.len(), 1);
        assert_eq!(modal.data.checklist[0].description, "Auto Append");
        assert_eq!(modal.data.item_description, "");
        assert_eq!(modal.item_index, 1);
        assert_eq!(modal.focus, InputField::ItemDescription);
    }
}
