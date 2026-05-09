//! View components for rendering the board picker screen.

use crate::{APP_NAME, app::App};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, List, ListItem, Paragraph};

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
    let mut title_style = Style::default().fg(colors.body_text);

    if !colors.transparent {
        title_style = title_style.bg(colors.background);
    }
    let title = Paragraph::new(format!("{} - Select Board", APP_NAME)).style(title_style);
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
            } else if colors.transparent {
                Style::default().fg(colors.body_text)
            } else {
                Style::default().fg(colors.body_text).bg(colors.background)
            })
        })
        .collect();

    let mut list_style = Style::default().fg(colors.body_text);

    if !colors.transparent {
        list_style = list_style.bg(colors.background);
    }

    let list = List::new(items).block(
        Block::default()
            .title("Boards")
            .border_style(Style::default().fg(colors.outer_border))
            .style(list_style),
    );
    frame.render_widget(list, chunks[1]);

    // Status bar
    let mut status_style = Style::default().fg(colors.body_text);

    if !colors.transparent {
        status_style = status_style.bg(colors.background);
    }

    let status = format!("Help [?] ({} boards)", app.model.board_list.len());
    frame.render_widget(Paragraph::new(status).style(status_style), chunks[2]);
}
