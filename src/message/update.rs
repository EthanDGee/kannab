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
        Action::NextColumn => todo!("Implement column movement"),
        Action::PrevColumn => todo!("Implement column movement"),
        Action::DeleteColumn => column_actions::delete_column(model),
        Action::MoveColumnLeft => column_actions::move_column_left(model),
        Action::MoveColumnRight => column_actions::move_column_right(model),

        // Task Handling
        Action::CreateTask(title, description) => {
            task_actions::create_task(model, title, description)
        }
        Action::EditTask(input_field, edit) => task_actions::edit_task(model, input_field, edit),
        Action::DeleteTask => task_actions::delete_task(model),
        Action::NextTask => todo!("Implement task navigation_actions"),
        Action::PrevTask => todo!("Implement task navigation_actions"),
        Action::MoveTaskUp => task_actions::move_task_up(model),
        Action::MoveTaskDown => task_actions::move_task_down(model),
        Action::MoveTaskToNextColumn => task_actions::move_task_to_next_column(model),
        Action::MoveTaskToPrevColumn => task_actions::move_task_to_prev_column(model),
        Action::ToggleCompletion => task_actions::toggle_completion(model),

        // Modal Actions
        Action::OpenModal(modal_type) => modal_actions::open_modal(model, modal_type),
        Action::CloseModal => modal_actions::close_modal(model),
        Action::SwitchInputField => modal_actions::switch_input_field(model),
        Action::UpdateField(field, value) => modal_actions::update_field(model, field, value),
        Action::MoveCursor(x, y) => modal_actions::move_cursor(model, x, y),
        Action::Confirm => modal_actions::confirm(model),
        Action::Cancel => modal_actions::cancel(model),

        // I/O Operations
        Action::Save => io_actions::save(model),
        Action::MarkDirty => io_actions::mark_dirty(model),
        Action::Tick => io_actions::handle_tick(model),
    }
}
