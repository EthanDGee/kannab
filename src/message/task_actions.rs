//! Handlers for task-level actions.

use crate::message::action::{Action, InputField};
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
pub fn toggle_task_completion(model: &mut AppState) -> Option<Action> {
    let board_state = model.board_state.as_mut()?;
    let (col_idx, task_idx) = (board_state.column_index, board_state.task_index);
    let task = board_state.board.get_task_mut(col_idx, task_idx)?;

    // flips completion
    task.toggle_completion();
    mark_dirty(model)
}

/// flips the completion status of currently selected checklist item.
pub fn toggle_item_completion(model: &mut AppState) -> Option<Action> {
    let modal_state = model.modal_state.as_mut()?;

    if modal_state.focus != InputField::ItemDescription {
        return None;
    }

    let item_index = modal_state.item_index;

    if item_index < modal_state.data.checklist.len() {
        modal_state.data.checklist[item_index].toggle_completion();
        return Some(Action::MarkDirty);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::board_state::{Board, Column};
    use crate::view::board_view::BoardState;

    fn setup_test_state() -> AppState {
        let mut model = AppState::new();
        let mut board = Board::new("Test Board".to_string());
        board.columns.push(Column::new());
        model.board_state = Some(BoardState::new(board));
        model
    }

    #[test]
    fn test_create_task() {
        let mut model = setup_test_state();
        create_task(
            &mut model,
            "New Task".to_string(),
            "Desc".to_string(),
            vec![],
        );

        let board_state = model.board_state.as_ref().unwrap();
        let task = board_state.board.get_task(0, 0).unwrap();
        assert_eq!(task.title, "New Task");
        assert_eq!(board_state.task_index, 0);
        assert!(model.pending_changes);
    }

    #[test]
    fn test_edit_task() {
        let mut model = setup_test_state();
        create_task(
            &mut model,
            "Old Title".to_string(),
            "Old Desc".to_string(),
            vec![],
        );
        edit_task(
            &mut model,
            "New Title".to_string(),
            "New Desc".to_string(),
            vec![],
        );

        let board_state = model.board_state.as_ref().unwrap();
        let task = board_state.board.get_task(0, 0).unwrap();
        assert_eq!(task.title, "New Title");
        assert_eq!(task.description, "New Desc");
    }

    #[test]
    fn test_delete_task() {
        let mut model = setup_test_state();
        create_task(&mut model, "Task 1".to_string(), "".to_string(), vec![]);
        create_task(&mut model, "Task 2".to_string(), "".to_string(), vec![]);

        model.board_state.as_mut().unwrap().task_index = 1;
        delete_task(&mut model);

        let board_state = model.board_state.as_ref().unwrap();
        assert_eq!(board_state.board.columns[0].tasks.len(), 1);
        assert_eq!(board_state.task_index, 0);
    }

    #[test]
    fn test_move_task_up_down() {
        let mut model = setup_test_state();
        create_task(&mut model, "T1".to_string(), "".to_string(), vec![]);
        create_task(&mut model, "T2".to_string(), "".to_string(), vec![]);

        // T2 is at index 1
        move_task_up(&mut model);
        assert_eq!(model.board_state.as_ref().unwrap().task_index, 0);
        assert_eq!(
            model
                .board_state
                .as_ref()
                .unwrap()
                .board
                .get_task(0, 0)
                .unwrap()
                .title,
            "T2"
        );

        move_task_down(&mut model);
        assert_eq!(model.board_state.as_ref().unwrap().task_index, 1);
        assert_eq!(
            model
                .board_state
                .as_ref()
                .unwrap()
                .board
                .get_task(0, 1)
                .unwrap()
                .title,
            "T2"
        );
    }

    #[test]
    fn test_move_task_between_columns() {
        let mut model = setup_test_state();
        // Add second column
        model
            .board_state
            .as_mut()
            .unwrap()
            .board
            .columns
            .push(Column::new());

        create_task(&mut model, "Move Me".to_string(), "".to_string(), vec![]);

        move_task_to_next_column(&mut model);
        {
            let bs = model.board_state.as_ref().unwrap();
            assert_eq!(bs.column_index, 1);
            assert_eq!(bs.board.columns[1].tasks.len(), 1);
            assert_eq!(bs.board.columns[0].tasks.len(), 0);
        }

        move_task_to_prev_column(&mut model);
        {
            let bs = model.board_state.as_ref().unwrap();
            assert_eq!(bs.column_index, 0);
            assert_eq!(bs.board.columns[0].tasks.len(), 1);
        }
    }

    #[test]
    fn test_toggle_task_completion() {
        let mut model = setup_test_state();
        create_task(&mut model, "Task".to_string(), "".to_string(), vec![]);

        toggle_task_completion(&mut model);
        assert!(
            model
                .board_state
                .as_ref()
                .unwrap()
                .board
                .get_task(0, 0)
                .unwrap()
                .complete
        );

        toggle_task_completion(&mut model);
        assert!(
            !model
                .board_state
                .as_ref()
                .unwrap()
                .board
                .get_task(0, 0)
                .unwrap()
                .complete
        );
    }
}
