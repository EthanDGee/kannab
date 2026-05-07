//! The central update function that routes actions to their specific handlers.

use crate::message::{
    action::Action, board_actions, column_actions, io_actions, modal_actions, navigation_actions,
    picker_actions, task_actions,
};
use crate::model::app_state::AppState;

/// Updates the application state based on the given action.
///
/// This function acts as a dispatcher, delegating complex logic to specialized modules
/// based on the type of action received. It may return a new action to be processed
/// in the next iteration of the update loop.
pub fn update(model: &mut AppState, action: Action) -> Option<Action> {
    match action {
        // General Actions
        Action::Quit => io_actions::quit(model),

        // Navigation
        Action::MoveUp => navigation_actions::move_up(model),
        Action::MoveDown => navigation_actions::move_down(model),
        Action::MoveLeft => navigation_actions::move_left(model),
        Action::MoveRight => navigation_actions::move_right(model),

        // Board Picker Actions
        Action::MoveBoardUp => board_actions::move_board_up(model),
        Action::MoveBoardDown => board_actions::move_board_down(model),
        Action::QuitToPicker => picker_actions::quit_to_picker(model),

        // Board Actions
        Action::OpenBoard => board_actions::open_board(model),
        Action::CreateBoard(title) => board_actions::create_board(model, title),
        Action::RenameBoard(title) => board_actions::rename_board(model, title),
        Action::DeleteBoard => board_actions::delete_board(model),

        // Column Handling
        Action::CreateColumn(title) => column_actions::create_column(model, title),
        Action::RenameColumn(new_name) => column_actions::rename_column(model, new_name),
        Action::DeleteColumn => column_actions::delete_column(model),
        Action::MoveColumnLeft => column_actions::move_column_left(model),
        Action::MoveColumnRight => column_actions::move_column_right(model),

        // Task Handling
        Action::CreateTask(title, description, checklist) => {
            task_actions::create_task(model, title, description, checklist)
        }
        Action::EditTask(title, description, checklist) => {
            task_actions::edit_task(model, title, description, checklist)
        }
        Action::DeleteTask => task_actions::delete_task(model),
        Action::MoveTaskUp => task_actions::move_task_up(model),
        Action::MoveTaskDown => task_actions::move_task_down(model),
        Action::MoveTaskToNextColumn => task_actions::move_task_to_next_column(model),
        Action::MoveTaskToPrevColumn => task_actions::move_task_to_prev_column(model),
        Action::ToggleTaskCompletion => task_actions::toggle_task_completion(model),
        Action::ToggleItemCompletion => task_actions::toggle_item_completion(model),
        Action::DeleteChecklistItem => modal_actions::delete_checklist_item(model),

        // Modal Actions
        Action::OpenModal(modal_type) => modal_actions::open_modal(model, modal_type),
        Action::CloseModal => modal_actions::close_modal(model),
        Action::SwitchInputField => modal_actions::switch_input_field(model),
        Action::ModalInput(key) => {
            if let Some(modal) = &mut model.modal_state
                && modal.active_textarea.input(key)
            {
                return Some(Action::MarkDirty);
            }
            None
        }
        Action::Confirm => modal_actions::confirm(model),
        Action::Cancel => modal_actions::cancel(model),

        // I/O Operations
        Action::Save => io_actions::save(model),
        Action::MarkDirty => io_actions::mark_dirty(model),
        Action::Tick => io_actions::handle_tick(model),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::app_state::AppMode;

    #[test]
    fn test_update_quit() {
        let mut model = AppState::new();
        update(&mut model, Action::Quit);
        assert!(model.should_quit);
    }

    #[test]
    fn test_update_mark_dirty() {
        let mut model = AppState::new();
        update(&mut model, Action::MarkDirty);
        assert!(model.pending_changes);
    }

    #[test]
    fn test_update_navigation_picker() {
        let mut model = AppState::new();
        model.mode = AppMode::Picker;
        model
            .board_list
            .push(crate::model::board_state::BoardName::new("B1".to_string()));
        model
            .board_list
            .push(crate::model::board_state::BoardName::new("B2".to_string()));

        update(&mut model, Action::MoveDown);
        assert_eq!(model.picker_state.index, 1);
    }
}
