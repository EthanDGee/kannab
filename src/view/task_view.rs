//! View components for rendering individual tasks and task-related UI elements.

use crate::app::App;
use crate::message::action::InputField;
use crate::model::board_state::Task;
use crate::model::modal_state::ModalState;
use crate::view::theme::ColorScheme;
use crate::widgets::floating_window::centered_rect;
use crate::widgets::text_input::TextInput;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, ListItem, Paragraph};

/// Returns the checkbox symbol based on completion
pub fn checkbox_symbol(completed: bool) -> &'static str {
    if completed { "" } else { "" }
}

/// Renders a single task as a `ListItem` of its column.
///
/// Each task is drawn within a bordered box, showing its completion status,
/// title, and an optional description preview.
pub fn render<'a>(task: &Task, colors: &ColorScheme, selected: bool, width: u16) -> ListItem<'a> {
    let border_color = if selected {
        colors.highlight
    } else {
        colors.inner_border
    };

    let inner_width = width.saturating_sub(2) as usize;

    let top = Line::from(format!("┌{}┐", "─".repeat(inner_width)).fg(border_color));

    let checkmark = checkbox_symbol(task.complete);
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

/// Renders a modal for creating or editing a task.
pub fn task_modal(
    app: &App,
    frame: &mut Frame,
    modal: &ModalState,
    area: Rect,
    title: &str,
    instruction_text: &str,
) {
    let colors = app.model.color_scheme;
    let area = centered_rect(70, 60, area);

    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors.highlight))
        .style(Style::default().bg(colors.background).fg(colors.body_text));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Title Label
            Constraint::Length(3), // Title Input
            Constraint::Length(1), // Description Label
            Constraint::Length(8), // Description Input
            Constraint::Length(1), // Checklist Label
            Constraint::Min(3),    // Checklist Items Area
            Constraint::Length(1), // Instructions
        ])
        .split(inner_area);

    // Title Field
    let title_label =
        Paragraph::new("Task Title:").style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(title_label, main_chunks[0]);

    let title_active = modal.focus == InputField::TaskTitle;
    let title_input = TextInput::new(colors, &modal.data.task_title, modal.cursor_position)
        .active(title_active)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(if title_active {
                    colors.highlight
                } else {
                    colors.inner_border
                })),
        );
    frame.render_widget(title_input, main_chunks[1]);

    // Description Field
    let desc_label =
        Paragraph::new("Description:").style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(desc_label, main_chunks[2]);

    let desc_active = modal.focus == InputField::TaskDescription;
    let desc_input = TextInput::new(colors, &modal.data.task_description, modal.cursor_position)
        .active(desc_active)
        .multiline()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(if desc_active {
                    colors.highlight
                } else {
                    colors.inner_border
                })),
        );
    frame.render_widget(desc_input, main_chunks[3]);

    // Checklist Label
    let checklist_label =
        Paragraph::new("Checklist:").style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(checklist_label, main_chunks[4]);

    // Checklist Items Scrolling Logic
    let checklist_area = main_chunks[5];
    let checklist_items = &modal.data.checklist;
    let total_items = checklist_items.len() + 1; // Existing items + one new item field
    let items_per_page = (checklist_area.height / 3) as usize;

    // Calculate effective scroll offset to ensure focused item is visible
    let mut scroll_offset = modal.scroll_offset;
    if modal.focus == InputField::ItemDescription {
        if modal.item_index < scroll_offset {
            scroll_offset = modal.item_index;
        } else if items_per_page > 0 && modal.item_index >= scroll_offset + items_per_page {
            scroll_offset = modal.item_index - items_per_page + 1;
        }
    }
    // Final boundary check
    scroll_offset = scroll_offset.min(total_items.saturating_sub(1));

    let visible_count = items_per_page.min(total_items.saturating_sub(scroll_offset));
    let checklist_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(3); visible_count])
        .split(checklist_area);

    for i in 0..visible_count {
        let item_idx = scroll_offset + i;
        if item_idx < checklist_items.len() {
            let item = &checklist_items[item_idx];
            render_check_list_item(
                frame,
                checklist_chunks[i],
                modal,
                &colors,
                item_idx,
                &item.description,
                item.complete,
            );
        } else if item_idx == checklist_items.len() {
            // New Checklist Item
            render_check_list_item(
                frame,
                checklist_chunks[i],
                modal,
                &colors,
                item_idx,
                &modal.data.item_description,
                false,
            );
        }
    }

    let instructions =
        Paragraph::new(instruction_text).style(Style::default().fg(colors.inner_border));
    frame.render_widget(instructions, main_chunks[6]);
}

/// Renders an individual checklist item within the task modal.
pub fn render_check_list_item(
    frame: &mut Frame,
    area: Rect,
    modal: &ModalState,
    colors: &ColorScheme,
    item_index: usize,
    content: &str,
    completed: bool,
) {
    let item_active = modal.focus == InputField::ItemDescription && modal.item_index == item_index;
    let item_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(3), // Checkbox
            Constraint::Min(0),    // Item Description
        ])
        .split(area);

    // Center checkbox vertically
    let checkbox_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Top padding
            Constraint::Length(1), // Checkbox line
            Constraint::Min(0),    // Bottom padding
        ])
        .split(item_chunks[0])[1];

    // Render checkbox
    let checkbox = Paragraph::new(format!(" {} ", checkbox_symbol(completed)))
        .style(Style::default().fg(colors.body_text));
    frame.render_widget(checkbox, checkbox_area);

    let item_input = TextInput::new(*colors, content, modal.cursor_position)
        .active(item_active)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(if item_active {
                    colors.highlight
                } else {
                    colors.inner_border
                })),
        );
    frame.render_widget(item_input, item_chunks[1]);
}
