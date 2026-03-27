use crate::model::board::Board;

/// Handles associated view data for the Kanban data
///
/// In addition to the selecting of tasks all scroll data for every column is stored independently to
/// make sure the state of scrolling is stored when switching between columns.
pub struct BoardState {
    column_index: usize,
    task_index: usize,
    column_scrolls: Vec<usize>,
}

impl BoardState {
    pub fn new(board: Board) -> Self {
        let num_columns = board.columns.len();
        BoardState {
            column_index: 0,
            task_index: 0,
            column_scrolls: vec![num_columns; 0],
        }
    }
}
