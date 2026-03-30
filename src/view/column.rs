use crate::app::App;
use crate::model::board_state::Column;
use crate::view::task;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListState},
};

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
            task::render_item(task, colors, is_selected, inner_width)
        })
        .collect();

    let list = List::new(task_items).block(Block::default());

    let mut list_state = ListState::default();
    if selected {
        list_state.select(Some(board_state.task_index));
    } else {
        // If not selected, we might want to show where the scroll was,
        // but for now let's just show the top.
        // If we want persistent scroll we'd use column_scrolls.
        if column_index < board_state.column_scrolls.len() {
            list_state.select(None);
            // Ratatui doesn't easily let us set offset without select.
            // Actually it does if we use a more complex state or manual rendering.
        }
    }

    frame.render_stateful_widget(list, inner, &mut list_state);
}
