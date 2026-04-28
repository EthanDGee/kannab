//! Handlers for task-level actions.

use crate::message::action::Action;
use crate::message::io_actions::mark_dirty;
use crate::message::navigation_actions::{decrement_no_wrap, increment_no_wrap};
use crate::model::app_state::AppState;
use crate::model::board_state::{Item, Task};

/// Creates a new task with the specified data in the active column.
pub fn create_task(
    model: &mut AppState,
    title: String,
    description: String,
    checklist: Vec<Item>,
) -> Option<Action> {
    {
        let board_state = model.board_state.as_mut()?;
        let column_index = board_state.column_index;
        let column = board_state.board.get_column_mut(column_index)?;
        column.tasks.push(Task::new());
        board_state.task_index = column.tasks.len() - 1;
    }

    edit_task(model, title, description, checklist)
}

/// Updates the currently selected task's fields.
pub fn edit_task(
    model: &mut AppState,
    title: String,
    description: String,
    checklist: Vec<Item>,
) -> Option<Action> {
    let board_state = model.board_state.as_mut()?;
    let (col_idx, task_idx) = (board_state.column_index, board_state.task_index);
    let task = board_state.board.get_task_mut(col_idx, task_idx)?;
    task.title = title;
    task.description = description;
    task.checklist = checklist;

    mark_dirty(model)
}

/// Deletes the currently selected task from its column.
pub fn delete_task(model: &mut AppState) -> Option<Action> {
    let board_state = model.board_state.as_mut()?;
    let (col_idx, task_idx) = (board_state.column_index, board_state.task_index);

    if board_state.task_list_empty(col_idx) {
        return None;
    }

    let column = board_state.board.get_column_mut(col_idx)?;
    column.remove_task(task_idx);
    let has_tasks = !column.task_list_empty();

    let task_count = column.tasks.len();

    // Adjust task_index if it's now out of bounds
    if task_idx >= task_count && has_tasks {
        board_state.task_index = task_count - 1;
    } else if !has_tasks {
        board_state.task_index = 0;
    }

    // Update the scroll persistent state for this column
    if col_idx < board_state.column_scrolls.len() {
        board_state.column_scrolls[col_idx] = board_state.task_index;
    }

    mark_dirty(model)
}

/// Swaps the current task with the one at the next index.
pub fn move_task_up(model: &mut AppState) -> Option<Action> {
    let board_state = model.board_state.as_mut()?;
    let (col_idx, task_idx) = (board_state.column_index, board_state.task_index);

    if board_state.task_list_empty(col_idx) {
        return None;
    }

    let column = board_state.board.get_column_mut(col_idx)?;

    match decrement_no_wrap(task_idx) {
        Some(new_index) => {
            column.swap_tasks(task_idx, new_index);
            board_state.task_index = new_index;
            mark_dirty(model)
        }
        None => None,
    }
}

/// Swaps the current task with the one at the previous index.
pub fn move_task_down(model: &mut AppState) -> Option<Action> {
    let board_state = model.board_state.as_mut()?;
    let (col_idx, task_idx) = (board_state.column_index, board_state.task_index);

    if board_state.task_list_empty(col_idx) {
        return None;
    }

    let column = board_state.board.get_column_mut(col_idx)?;

    match increment_no_wrap(task_idx, column.tasks.len()) {
        Some(new_index) => {
            column.swap_tasks(task_idx, new_index);
            board_state.task_index = new_index;
            mark_dirty(model)
        }
        None => None,
    }
}

/// Moves the currently selected task to the beginning of the next column.
pub fn move_task_to_next_column(model: &mut AppState) -> Option<Action> {
    let board_state = model.board_state.as_mut()?;
    let (col_idx, task_idx) = (board_state.column_index, board_state.task_index);
    let num_columns = board_state.board.columns.len();

    if board_state.task_list_empty(col_idx) {
        return None;
    }

    let task = board_state
        .board
        .get_column_mut(col_idx)?
        .remove_task(task_idx)?;

    match increment_no_wrap(col_idx, num_columns) {
        Some(new_column_index) => {
            board_state
                .board
                .get_column_mut(new_column_index)?
                .insert_task(0, task);
            board_state.column_index = new_column_index;
            board_state.task_index = 0;
            mark_dirty(model)
        }
        None => {
            // Put it back if we can't move
            board_state
                .board
                .get_column_mut(col_idx)?
                .insert_task(task_idx, task);
            None
        }
    }
}

/// Moves the currently selected task to the beginning of the previous column.
pub fn move_task_to_prev_column(model: &mut AppState) -> Option<Action> {
    let board_state = model.board_state.as_mut()?;
    let (col_idx, task_idx) = (board_state.column_index, board_state.task_index);

    if board_state.task_list_empty(col_idx) {
        return None;
    }

    let task = board_state
        .board
        .get_column_mut(col_idx)?
        .remove_task(task_idx)?;

    match decrement_no_wrap(col_idx) {
        Some(new_column_index) => {
            board_state
                .board
                .get_column_mut(new_column_index)?
                .insert_task(0, task);
            board_state.column_index = new_column_index;
            board_state.task_index = 0;
            mark_dirty(model)
        }
        None => {
            // Put it back if we can't move
            board_state
                .board
                .get_column_mut(col_idx)?
                .insert_task(task_idx, task);
            None
        }
    }
}

/// Flips the `complete` status of the currently selected task.
pub fn toggle_completion(model: &mut AppState) -> Option<Action> {
    let board_state = model.board_state.as_mut()?;
    let (col_idx, task_idx) = (board_state.column_index, board_state.task_index);
    let task = board_state.board.get_task_mut(col_idx, task_idx)?;

    // flips completion
    task.complete = !task.complete;
    mark_dirty(model)
}
