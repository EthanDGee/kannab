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

    let checkmark = if task.complete { "" } else { "" };

    let formatted_title = format!("{} {}", checkmark, task.title.clone());

    let header = if task.title.len() > inner_width {
        format!("{}...", formatted_title)
    } else {
        formatted_title
    };

    let mut header_span = format!("{: <width$}", header, width = inner_width).fg(if selected {
        colors.highlight_text
    } else {
        colors.body_text
    });
    if selected {
        header_span = header_span.bold();
    }

    let header_line = Line::from(vec![
        "│".fg(border_color),
        header_span,
        "│".fg(border_color),
    ]);

    let mut lines = vec![top, header_line];

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
