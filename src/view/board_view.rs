//! View components for rendering a full Kanban board.

use crate::app::App;
use crate::view::column_view::{self, COLUMN_WIDTH};
use crate::{APP_NAME, model::board_state::Board};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::Paragraph,
};

/// Handles associated view data for the Kanban board data.
///
/// In addition to the cursor position for tasks and current column, all scroll data for every column is stored
/// independently to ensure the scroll position is preserved when switching between columns.
pub struct BoardState {
    /// Index of the currently focused column.
    pub column_index: usize,
    /// Index of the currently focused task within the active column.
    pub task_index: usize,
    /// Persistent scroll positions for each column.
    pub column_scrolls: Vec<usize>,
    /// The underlying board data model.
    pub board: Board,
}

impl BoardState {
    /// Creates a new `BoardState` for the given board, initializing column scrolls to zero.
    pub fn new(board: Board) -> Self {
        let num_columns = board.columns.len();
        BoardState {
            column_index: 0,
            task_index: 0,
            column_scrolls: vec![0; num_columns],
            board,
        }
    }

    /// Returns true if the board contains no columns.
    pub fn column_list_empty(&self) -> bool {
        self.board.column_list_empty()
    }

    /// Returns true if the column at the provided index contains no tasks.
    pub fn task_list_empty(&self, column_index: usize) -> bool {
        self.board.task_list_empty(column_index)
    }
}

/// Renders the active board view, including its header, columns, and footer.
pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let board_state = app.model.board_state.as_ref().unwrap();
    let board = &board_state.board;

    let colors = &app.model.color_scheme;
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Header
            Constraint::Min(0),    // Kanban Manager
            Constraint::Length(1), // Footer
        ])
        .split(area);

    // Header
    let header_title = format!("{} - {}", APP_NAME, board.title);
    let title = Paragraph::new(header_title)
        .style(Style::default().fg(colors.body_text).bg(colors.background));
    frame.render_widget(title, chunks[0]);

    // Horizontal layout for columns with fixed width and scrolling
    let available_width = chunks[1].width;
    let max_visible_columns = (available_width / COLUMN_WIDTH).max(1) as usize;

    // Calculate which columns to display to keep the selected one in view
    // This is a simple "follow the cursor" scrolling logic
    let start_col = if board_state.column_index >= max_visible_columns {
        board_state
            .column_index
            .saturating_sub(max_visible_columns - 1)
    } else {
        0
    };

    let visible_count = std::cmp::min(board.columns.len() - start_col, max_visible_columns);

    let column_constraints: Vec<Constraint> =
        std::iter::repeat_n(Constraint::Length(COLUMN_WIDTH), visible_count).collect();

    let column_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(column_constraints)
        .split(chunks[1]);

    for (i, column_area) in column_areas.iter().enumerate() {
        let actual_idx = start_col + i;
        if let Some(column) = board.columns.get(actual_idx) {
            let selected = actual_idx == board_state.column_index;
            column_view::render(app, frame, column, *column_area, selected, actual_idx);
        }
    }

    // Footer
    let footer_text = format!("{}  [?] Help", board.title);
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(colors.body_text).bg(colors.background));
    frame.render_widget(footer, chunks[2]);
}
