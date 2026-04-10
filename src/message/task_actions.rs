//! Handlers for task-level actions.

use crate::message::action::{Action, InputField};
use crate::message::column_actions::get_current_column_mut;
use crate::message::io_actions::mark_dirty;
use crate::message::navigation_actions::{decrement_no_wrap, increment_no_wrap};
use crate::model::app_state::AppState;
use crate::model::board_state::Task;

/// Returns a mutable reference to the currently selected task, if a board and column are active.
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

/// Creates a new task with the specified title and description in the active column, if a board and
/// column are active.
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

/// Updates a specific field (title or description) of the currently selected task.
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

/// Deletes the currently selected task from its column.
pub fn delete_task(model: &mut AppState) -> Option<Action> {
    let board_state = model.board_state.as_mut()?;
    let task_index = board_state.task_index;
    let column_index = board_state.column_index;
    let column = board_state.board.columns.get_mut(column_index)?;
    let task_count = column.tasks.len();

    if task_index > task_count || task_count == 0 {
        return None;
    }

    column.tasks.remove(task_index);
    let has_tasks = !column.tasks.is_empty();

    // Adjust task_index if it's now out of bounds
    if task_index >= task_count && has_tasks {
        board_state.task_index = task_count - 1;
    } else if !has_tasks {
        board_state.task_index = 0;
    }

    // Update the scroll persistent state for this column
    if column_index < board_state.column_scrolls.len() {
        board_state.column_scrolls[column_index] = board_state.task_index;
    }

    mark_dirty(model)
}

/// Swaps the current task with the one at the next index.
pub fn move_task_up(model: &mut AppState) -> Option<Action> {
    let board_state = model.board_state.as_ref()?;
    let task_index = board_state.task_index;

    let column = get_current_column_mut(model)?;

    match decrement_no_wrap(task_index) {
        Some(new_index) => {
            column.tasks.swap(task_index, new_index);
            model.board_state.as_mut()?.task_index = new_index;
            mark_dirty(model)
        }
        None => None,
    }
}

/// Swaps the current task with the one at the previous index.
pub fn move_task_down(model: &mut AppState) -> Option<Action> {
    let board_state = model.board_state.as_ref()?;
    let task_index = board_state.task_index;

    let column = get_current_column_mut(model)?;

    match increment_no_wrap(task_index, column.tasks.len()) {
        Some(new_index) => {
            column.tasks.swap(task_index, new_index);
            model.board_state.as_mut()?.task_index = new_index;
            mark_dirty(model)
        }
        None => None,
    }
}

/// Moves the currently selected task to the beginning of the next column.
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

/// Moves the currently selected task to the beginning of the previous column.
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

/// Flips the `complete` status of the currently selected task.
pub fn toggle_completion(model: &mut AppState) -> Option<Action> {
    let _board_state = model.board_state.as_mut()?;
    let task = get_current_task_mut(model)?;

    // flips completion
    task.complete = !task.complete;
    mark_dirty(model)
}
