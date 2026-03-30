use crate::message::action::{Action, InputField};
use crate::message::column_actions::get_current_column_mut;
use crate::message::io_actions::mark_dirty;
use crate::message::navigation_actions::{decrement_no_wrap, increment_no_wrap};
use crate::model::{
    app_state::AppState,
    board_state::Task,
    modal_state::{ModalState, ModalType},
};

/// Utility function to get currently highlighted task based on AppState.BoardState's index
pub fn get_current_task_mut(model: &mut AppState) -> Option<&mut Task> {
    let (column_index, task_index) = {
        let board_state = model.board_state.as_ref()?;
        (board_state.column_index, board_state.task_index)
    };
    let board_state = model.board_state.as_mut()?;
    let board = &mut board_state.board;
    let column = board.columns.get_mut(column_index)?;
    column.tasks.get_mut(task_index)
}

/// Creates a new task with the specified title and description and adds it to the current column.
pub fn create_task(model: &mut AppState, title: String, description: String) -> Option<Action> {
    if let Some(board_state) = &mut model.board_state {
        let mut task = Task::new();
        task.title = title;
        task.description = description;

        let column_index = board_state.column_index;
        if let Some(column) = board_state.board.columns.get_mut(column_index) {
            column.tasks.push(task);
            board_state.task_index = column.tasks.len() - 1;
            mark_dirty(model)
        } else {
            None
        }
    } else {
        None
    }
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
    mark_dirty(model)
}

/// Deletes the currently highlighted task
pub fn delete_task(model: &mut AppState) -> Option<Action> {
    let board_state = model.board_state.as_mut()?;
    let task_index = board_state.task_index;
    let column = get_current_column_mut(model)?;
    column.tasks.remove(task_index);
    mark_dirty(model)
}

/// Move currently selected task up in the current column
pub fn move_task_up(model: &mut AppState) -> Option<Action> {
    let board_state = model.board_state.as_mut()?;
    let task_index = board_state.task_index;
    let column = get_current_column_mut(model)?;

    match increment_no_wrap(task_index, column.tasks.len()) {
        Some(new_index) => {
            column.tasks.swap(task_index, new_index);
            mark_dirty(model)
        }
        None => None,
    }
}

/// Move currently selected task down in the current column
pub fn move_task_down(model: &mut AppState) -> Option<Action> {
    let board_state = model.board_state.as_mut()?;
    let task_index = board_state.task_index;
    let column = get_current_column_mut(model)?;

    match decrement_no_wrap(task_index) {
        Some(new_index) => {
            column.tasks.swap(task_index, new_index);
            mark_dirty(model)
        }
        None => None,
    }
}

/// Move currently highlighted task and cursor to the top of the right column
pub fn move_task_to_next_column(model: &mut AppState) -> Option<Action> {
    let board_state = model.board_state.as_mut()?;
    let column_index = board_state.column_index;
    let task_index = board_state.task_index;
    let num_columns = board_state.board.columns.len();

    let task = {
        let column = board_state.board.columns.get_mut(column_index)?;
        column.tasks.remove(task_index)
    };

    match increment_no_wrap(column_index, num_columns) {
        Some(new_column_index) => {
            board_state
                .board
                .columns
                .get_mut(new_column_index)?
                .tasks
                .insert(0, task);
            board_state.column_index = new_column_index;
            board_state.task_index = 0;
            mark_dirty(model)
        }
        None => None,
    }
}

/// Move currently highlighted task and cursor to the top of the left column
pub fn move_task_to_prev_column(model: &mut AppState) -> Option<Action> {
    let board_state = model.board_state.as_mut()?;
    let column_index = board_state.column_index;
    let task_index = board_state.task_index;

    let task = {
        let column = board_state.board.columns.get_mut(column_index)?;
        column.tasks.remove(task_index)
    };

    match decrement_no_wrap(column_index) {
        Some(new_column_index) => {
            board_state
                .board
                .columns
                .get_mut(new_column_index)?
                .tasks
                .insert(0, task);
            board_state.column_index = new_column_index;
            board_state.task_index = 0;
            mark_dirty(model)
        }
        None => None,
    }
}

/// Toggles the completion of the currently selected task
pub fn toggle_completion(model: &mut AppState) -> Option<Action> {
    let _board_state = model.board_state.as_mut()?;
    let task = get_current_task_mut(model)?;

    // flips completion
    task.complete = !task.complete;
    mark_dirty(model)
}
