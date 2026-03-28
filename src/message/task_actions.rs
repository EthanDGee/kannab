use crate::message::action::{Action, InputField};
use crate::model::board::Column;
use crate::model::{
    app_state::AppState,
    board::Task,
    modal_state::{ModalState, ModalType},
};

/// Utility function to get currently highlighted column based on AppState.BoardState's index
fn get_current_column_mut(model: &mut AppState) -> Option<&mut Column> {
    // Extract the column index immutably first to avoid overlapping borrows
    let column_index = model.board_state.as_ref()?.column_index;
    // Now borrow mutably
    let board_state = model.board_state.as_mut()?;
    board_state.board.columns.get_mut(column_index)
}

/// Utility function to get currently highlighted task based on AppState.BoardState's index
fn get_current_task_mut(model: &mut AppState) -> Option<&mut Task> {
    let (column_index, task_index) = {
        let board_state = model.board_state.as_ref()?;
        (board_state.column_index, board_state.task_index)
    };
    let board_state = model.board_state.as_mut()?;
    let board = &mut board_state.board;
    let column = board.columns.get_mut(column_index)?;
    column.tasks.get_mut(task_index)
}

/// Enters the Task Creation model state which handles task creation
pub fn create_task(model: &mut AppState) -> Option<Action> {
    if model.board_state.is_some() {
        let new_state = ModalState::new(ModalType::CreateBoard);
        model.modal_state = Some(new_state);
    }

    None
}
/// Replaces the input field of the currently selected task with the new edit
pub fn edit_task(model: &mut AppState, input_field: InputField, edit: String) -> Option<Action> {
    model.board_state.as_ref()?;

    let task = get_current_task_mut(model)?;

    match input_field {
        InputField::TaskTitle => task.title = edit,
        InputField::TaskDescription => task.description = edit,
        _ => {}
    }
    // TODO: MarkDirty after update
    None
}

/// Deletes the currently highlighted task
pub fn delete_task(model: &mut AppState) -> Option<Action> {
    let board_state = model.board_state.as_mut()?;
    let task_index = board_state.task_index;
    let column = get_current_column_mut(model)?;
    column.tasks.remove(task_index);
    None
}

// MoveTaskUp,
// MoveTaskDown,
// MoveTaskToNextColumn,
// MoveTaskToPrevColumn,
//
