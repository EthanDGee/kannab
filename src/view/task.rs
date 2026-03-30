use crate::model::board_state::Task;
use crate::view::theme::ColorScheme;
use ratatui::prelude::*;
use ratatui::widgets::ListItem;

pub fn render_item<'a>(
    task: &Task,
    colors: &ColorScheme,
    selected: bool,
    width: u16,
) -> ListItem<'a> {
    let border_color = if selected {
        colors.highlight
    } else {
        colors.inner_border
    };

    let inner_width = width.saturating_sub(2) as usize;

    // Using Stylize trait for ergonomic styling on strings (returns a Span)
    let top = Line::from(format!("┌{}┐", "─".repeat(inner_width)).fg(border_color));

    let title = if task.title.len() > inner_width {
        format!("{}...", &task.title[..inner_width.saturating_sub(3)])
    } else {
        task.title.clone()
    };

    let mut title_span = format!("{: <width$}", title, width = inner_width).fg(if selected {
        colors.highlight_text
    } else {
        colors.body_text
    });
    if selected {
        title_span = title_span.bold();
    }

    let title_line = Line::from(vec!["│".fg(border_color), title_span, "│".fg(border_color)]);

    let mut lines = vec![top, title_line];

    if !task.description.is_empty() {
        let desc = if task.description.len() > inner_width {
            format!("{}...", &task.description[..inner_width.saturating_sub(3)])
        } else {
            task.description.clone()
        };
        let desc_line = Line::from(vec![
            "│".fg(border_color),
            format!("{: <width$}", desc, width = inner_width).fg(colors.inner_border),
            "│".fg(border_color),
        ]);
        lines.push(desc_line);
    }

    let bottom = Line::from(format!("└{}┘", "─".repeat(inner_width)).fg(border_color));
    lines.push(bottom);

    ListItem::new(lines)
}
