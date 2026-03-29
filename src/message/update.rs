use core::task;

use crate::{
    message::{action::Action, task_actions},
    model::app_state::AppState,
};

/// Given an action updates the AppState returning an Optional next Action if required
/// such as ExitModal or MarkDirty
pub fn update(model: &mut AppState, action: Action) -> Option<Action> {
    match action {
        // Task Handling
        Action::CreateTask => task_actions::create_task(model),
        Action::EditTask(input_field, edit) => task_actions::edit_task(model, input_field, edit),
        Action::DeleteTask => task_actions::delete_task(model),
        Action::MoveTaskUp => task_actions::move_task_up(model),
        Action::MoveTaskDown => task_actions::move_task_down(model),
        Action::MoveTaskToNextColumn => task_actions::move_task_to_next_column(model),
        Action::MoveTaskToPrevColumn => task_actions::move_task_to_prev_column(model),
        _ => None,
    }
}
