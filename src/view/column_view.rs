//! View components for rendering individual columns within a board.

use crate::app::App;
use crate::model::board_state::Column;
use crate::view::task_view;
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
            task_view::render_item(task, colors, is_selected, inner_width)
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
