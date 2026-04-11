//! View components for rendering the board picker screen.

use crate::{APP_NAME, app::App};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, List, ListItem, Paragraph},
};

/// Transient UI state for the board picker.
#[derive(Clone, Default)]
pub struct PickerState {
    /// Index of the currently highlighted board in the list.
    pub index: usize,
}

impl PickerState {
    /// Creates a new `PickerState` with default values.
    pub fn new() -> Self {
        Self::default()
    }
}

/// Renders the board picker view, including the header, list of boards, and status bar.
pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let colors = &app.model.color_scheme;
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Board list
            Constraint::Length(1), // Status bar
        ])
        .split(area);

    // Header
    let title = Paragraph::new(format!("{} - Select Board", APP_NAME))
        .style(Style::default().fg(colors.body_text).bg(colors.background));
    frame.render_widget(title, chunks[0]);

    // Board list (custom scrollable list widget)
    let items: Vec<ListItem> = app
        .model
        .board_list
        .iter()
        .enumerate()
        .map(|(i, board)| {
            let selected = i == app.model.picker_state.index;
            ListItem::new(board.title.clone()).style(if selected {
                // Board Name
                Style::default()
                    .fg(colors.highlight_text)
                    .bg(colors.highlight)
            } else {
                Style::default().fg(colors.body_text).bg(colors.background)
            })
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title("Boards")
            .border_style(Style::default().fg(colors.outer_border))
            .style(Style::default().fg(colors.body_text).bg(colors.background)),
    );
    frame.render_widget(list, chunks[1]);

    // Status bar
    let status = format!("[NORMAL] [?] Help  ({} boards)", app.model.board_list.len());
    frame.render_widget(
        Paragraph::new(status).style(Style::default().fg(colors.body_text).bg(colors.background)),
        chunks[2],
    );
}
