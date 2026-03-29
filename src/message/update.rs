use core::task;

use crate::{
    message::{action::Action, column_actions, io_actions, task_actions},
    model::app_state::AppState,
};

/// Given an action updates the AppState returning an Optional next Action if required
/// such as ExitModal or MarkDirty
pub fn update(model: &mut AppState, action: Action) -> Option<Action> {
    match action {
        // Column Handling
        Action::CreateColumn => column_actions::create_column(model),
        Action::RenameColumn(new_name) => column_actions::rename_column(model, new_name),
        Action::DeleteColumn => column_actions::delete_column(model),
        Action::MoveColumnLeft => column_actions::move_column_left(model),
        Action::MoveColumnRight => column_actions::move_column_right(model),

        // Task Handling
        Action::CreateTask => task_actions::create_task(model),
        Action::EditTask(input_field, edit) => task_actions::edit_task(model, input_field, edit),
        Action::DeleteTask => task_actions::delete_task(model),
        Action::MoveTaskUp => task_actions::move_task_up(model),
        Action::MoveTaskDown => task_actions::move_task_down(model),
        Action::MoveTaskToNextColumn => task_actions::move_task_to_next_column(model),
        Action::MoveTaskToPrevColumn => task_actions::move_task_to_prev_column(model),
        Action::ToggleCompletion => task_actions::toggle_completion(model),

        // I/O Operations
        Action::MarkDirty => io_actions::mark_dirty(model),

        _ => None,
    }
}
