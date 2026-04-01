use crate::view::column;
use crate::{APP_NAME, model::board_state::Board};

/// Handles associated view data for the Kanban data
///
/// In addition to the selecting of tasks all scroll data for every column is stored independently to
/// make sure the state of scrolling is stored when switching between columns.
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

use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::Paragraph,
};

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

    // Horizontal layout for columns

    // Calculate column widths based on terminal width
    let column_count = board.columns.len().max(1);
    let column_width = (area.width as usize / column_count).max(20);

    let column_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(std::iter::repeat(Constraint::Length(column_width as u16)).take(column_count))
        .split(area);

    for (i, (column, column_area)) in board.columns.iter().zip(column_areas.iter()).enumerate() {
        let selected = i == board_state.column_index;
        column::render(app, frame, column, *column_area, selected, i);
    }
}
