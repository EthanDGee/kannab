//! View components for rendering individual columns within a board.

use crate::app::App;
use crate::model::board_state::Column;
use crate::model::modal_state::ModalState;
use crate::view::task_view;
use crate::widgets::floating_window::centered_rect;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Clear, Paragraph};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListState},
};

/// The fixed width used for all Kanban columns.
pub const COLUMN_WIDTH: u16 = 40;

/// Renders a single column, including its title and the list of tasks it contains.
pub fn render(
    app: &App,
    frame: &mut Frame,
    column: &Column,
    area: Rect,
    selected: bool,
    column_index: usize,
) {
    let colors = &app.model.color_scheme;
    let board_state = app.model.board_state.as_ref().expect("BoardState missing");

    let block_style = if selected {
        Style::new()
            .fg(colors.highlight)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::new().fg(colors.body_text)
    };

    let border_style = if selected {
        Style::new().fg(colors.highlight)
    } else {
        Style::new().fg(colors.outer_border)
    };

    let block = Block::default()
        .title(format!(" {} ({})", column.title, column.tasks.len()))
        .style(block_style)
        .borders(Borders::ALL)
        .border_style(border_style);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Task list within column
    let inner_width = inner.width;
    let task_items: Vec<ratatui::widgets::ListItem> = column
        .tasks
        .iter()
        .enumerate()
        .map(|(i, task)| {
            let is_selected = selected && i == board_state.task_index;
            task_view::render(task, colors, is_selected, inner_width)
        })
        .collect();

    let list = List::new(task_items).block(Block::default());

    let mut list_state = ListState::default();
    if selected {
        list_state.select(Some(board_state.task_index));
    } else {
        // Use the persistent scroll position for non-selected columns
        let scroll_index = board_state.column_scrolls.get(column_index).cloned();
        list_state.select(scroll_index);
    }

    frame.render_stateful_widget(list, inner, &mut list_state);
}

/// Renders a modal for creating or renaming a column.
pub fn column_modal_view(
    app: &App,
    frame: &mut Frame,
    modal: &ModalState,
    area: Rect,
    title: &str,
    instruction_text: &str,
) {
    let colors = app.model.color_scheme;
    let area = centered_rect(60, 15, area);

    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors.highlight))
        .style(Style::default().bg(colors.background).fg(colors.body_text));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Title Label
            Constraint::Length(3), // Input Field
            Constraint::Length(1), // Instructions
        ])
        .split(inner_area);

    let label =
        Paragraph::new("Column Title:").style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(label, chunks[0]);

    // Use TextArea for input
    let mut textarea = modal.active_textarea.clone();
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(colors.inner_border)),
    );
    textarea.set_cursor_style(Style::default().fg(colors.background).bg(colors.highlight));
    frame.render_widget(&textarea, chunks[1]);

    let instructions =
        Paragraph::new(instruction_text).style(Style::default().fg(colors.inner_border));
    frame.render_widget(instructions, chunks[2]);
}
