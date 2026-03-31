use crate::{
    message::{
        action::Action, column_actions, io_actions, modal_actions, navigation_actions, task_actions,
    },
    model::app_state::AppState,
};

/// Given an action updates the AppState returning an Optional next Action if required
/// such as ExitModal or MarkDirty
pub fn update(model: &mut AppState, action: Action) -> Option<Action> {
    match action {
        // Navigation
        Action::MoveUp => navigation_actions::move_up(model),
        Action::MoveDown => navigation_actions::move_down(model),
        Action::MoveLeft => navigation_actions::move_left(model),
        Action::MoveRight => navigation_actions::move_right(model),

        // Board Picker Actions

        // Column Handling
        Action::CreateColumn(title) => column_actions::create_column(model, title),
        Action::RenameColumn(new_name) => column_actions::rename_column(model, new_name),
        Action::DeleteColumn => column_actions::delete_column(model),
        Action::MoveColumnLeft => column_actions::move_column_left(model),
        Action::MoveColumnRight => column_actions::move_column_right(model),

        // Task Handling
        Action::CreateTask(title, description) => {
            task_actions::create_task(model, title, description)
        }
        Action::EditTask(input_field, edit) => task_actions::edit_task(model, input_field, edit),
        Action::DeleteTask => task_actions::delete_task(model),
        Action::MoveTaskUp => task_actions::move_task_up(model),
        Action::MoveTaskDown => task_actions::move_task_down(model),
        Action::MoveTaskToNextColumn => task_actions::move_task_to_next_column(model),
        Action::MoveTaskToPrevColumn => task_actions::move_task_to_prev_column(model),
        Action::ToggleCompletion => task_actions::toggle_completion(model),

        // Modal Actions
        Action::OpenModal(modal_type) => modal_actions::open_modal(model, modal_type),
        Action::CloseModal => modal_actions::close_modal(model),
        Action::UpdateField(field, value) => modal_actions::update_field(model, field, value),
        Action::Confirm => modal_actions::confirm(model),
        Action::Cancel => modal_actions::cancel(model),

        // I/O Operations
        Action::MarkDirty => io_actions::mark_dirty(model),
        _ => None,
    }
}
