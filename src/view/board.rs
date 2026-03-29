use crate::model::board::Board;

/// Handles associated view data for the Kanban data
///
/// In addition to the selecting of tasks all scroll data for every column is stored independently to
/// make sure the state of scrolling is stored when switching between columns.
#[derive(Clone)]
pub struct BoardState {
    pub column_index: usize,
    pub task_index: usize,
    pub column_scrolls: Vec<usize>,
    pub board: Board,
}

impl BoardState {
    pub fn new(board: Board) -> Self {
        let num_columns = board.columns.len();
        BoardState {
            column_index: 0,
            task_index: 0,
            column_scrolls: vec![0; num_columns],
            board,
        }
    }
}
