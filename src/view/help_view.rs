//! Help view components for displaying application keybindings.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::app::App;
use crate::view::theme::ColorScheme;
use crate::widgets::floating_window::centered_rect;

/// A helpful hint that explains a specific action.
struct Hint {
    /// The name of the command.
    command: String,

    /// A short description of the command.
    description: String,

    /// The associated keybindings for the command.
    keybind: String,
}

impl Hint {
    fn new(command: &str, description: &str, keybind: &str) -> Self {
        Self {
            command: command.to_string(),
            description: description.to_string(),
            keybind: keybind.to_string(),
        }
    }
}

struct Category {
    name: String,
    hints: Vec<Hint>,
}

/// renders the complete help modal with all of the associated help sections
pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let colors = &app.model.color_scheme;
    let help_area = centered_rect(80, 80, area);
    frame.render_widget(Clear, help_area);

    let mut block_style = Style::default().fg(colors.body_text);
    if !colors.transparent {
        block_style = block_style.bg(colors.background);
    }

    let block = Block::default()
        .title(" Help ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors.highlight))
        .style(block_style);
    frame.render_widget(block, help_area);

    let inner_area = Rect::new(
        help_area.x + 2,
        help_area.y + 1,
        help_area.width.saturating_sub(4),
        help_area.height.saturating_sub(2),
    );

    let categories = vec![
        Category {
            name: "General".to_string(),
            hints: vec![
                Hint::new("Help", "Show this help overlay with keybindings", "?"),
                Hint::new(
                    "Quit",
                    "Exit the application or close current view",
                    "q / Esc",
                ),
            ],
        },
        Category {
            name: "Board Picker (Main Menu)".to_string(),
            hints: vec![
                Hint::new(
                    "Up",
                    "Select the previous board in the list",
                    "k / ↑ / BackTab",
                ),
                Hint::new("Down", "Select the next board in the list", "j / ↓ / Tab"),
                Hint::new("Open", "Open the selected Kanban board", "Enter"),
                Hint::new(
                    "Move Up",
                    "Move the selected board up in the order",
                    "Shift+k / Shift+↑",
                ),
                Hint::new(
                    "Move Down",
                    "Move the selected board down in the order",
                    "Shift+j / Shift+↓",
                ),
                Hint::new("New Board", "Create a brand new Kanban board", "n / c"),
                Hint::new("Rename", "Rename the currently selected board", "e / r"),
                Hint::new("Delete", "Permanently delete the selected board", "d"),
            ],
        },
        Category {
            name: "Board View (Task Navigation)".to_string(),
            hints: vec![
                Hint::new("Left", "Focus the column to the left", "h / ←"),
                Hint::new("Right", "Focus the column to the right", "l / →"),
                Hint::new("Up", "Select the task above in the current column", "k / ↑"),
                Hint::new(
                    "Down",
                    "Select the task below in the current column",
                    "j / ↓",
                ),
                Hint::new(
                    "Complete",
                    "Toggle completion status of the selected task",
                    "Tab",
                ),
                Hint::new(
                    "Move Up",
                    "Move selected task up within its column",
                    "Shift+k / Shift+↑",
                ),
                Hint::new(
                    "Move Down",
                    "Move selected task down within its column",
                    "Shift+j / Shift+↓",
                ),
                Hint::new(
                    "Move Left",
                    "Move selected task to the previous column",
                    "Shift+h / Shift+←",
                ),
                Hint::new(
                    "Move Right",
                    "Move selected task to the next column",
                    "Shift+l / Shift+→",
                ),
                Hint::new(
                    "New Task",
                    "Create a new task in the current column",
                    "n / c",
                ),
                Hint::new(
                    "Edit Task",
                    "Open the task editor for the selected task",
                    "e",
                ),
                Hint::new(
                    "Delete Task",
                    "Delete the selected task from the board",
                    "d",
                ),
                Hint::new(
                    "New Col",
                    "Add a new column to the board",
                    "Ctrl+n / Shift+c",
                ),
                Hint::new(
                    "Rename Col",
                    "Rename the currently focused column",
                    "Ctrl+r / Shift+e / r",
                ),
                Hint::new(
                    "Delete Col",
                    "Delete the focused column and its tasks",
                    "Ctrl+d / Shift+d",
                ),
                Hint::new(
                    "Move Col L",
                    "Move the focused column to the left",
                    "Ctrl+h / Ctrl+←",
                ),
                Hint::new(
                    "Move Col R",
                    "Move the focused column to the right",
                    "Ctrl+l / Ctrl+→",
                ),
                Hint::new("Exit Board View", "Return to the board picker", "q / Esc"),
            ],
        },
        Category {
            name: "Task Modal (Create / Edit Task)".to_string(),
            hints: vec![
                Hint::new("Next Field", "Cycle focus to the next input field", "Tab"),
                Hint::new(
                    "Prev Field",
                    "Cycle focus to the previous input field",
                    "Shift+Tab / BackTab",
                ),
                Hint::new(
                    "Save",
                    "Save all changes and close the modal",
                    "Ctrl+s / Ctrl+Enter",
                ),
                Hint::new(
                    "Quick Save",
                    "Confirm and save (when Title is focused)",
                    "Enter",
                ),
                Hint::new(
                    "Toggle Item",
                    "Toggle completion of the currently selected checklist item",
                    "Enter",
                ),
                Hint::new(
                    "Delete Item",
                    "Remove the selected checklist item",
                    "Ctrl+Backspace / Ctrl+h",
                ),
                Hint::new("Close", "Discard all changes and close the modal", "Esc"),
            ],
        },
        Category {
            name: "Help Modal".to_string(),
            hints: vec![
                Hint::new("Scroll Up", "Scroll up through the help sections", "k / ↑"),
                Hint::new(
                    "Scroll Down",
                    "Scroll down through the help sections",
                    "j / ↓",
                ),
                Hint::new("Close", "Close the help overlay", "q / Esc / ?"),
            ],
        },
        Category {
            name: "Other Modals & Dialogs".to_string(),
            hints: vec![
                Hint::new("Confirm", "Confirm the action or save changes", "Enter"),
                Hint::new("Cancel", "Cancel the action or discard changes", "Esc"),
            ],
        },
    ];

    let scroll = app
        .model
        .modal_state
        .as_ref()
        .map(|m| m.scroll_offset)
        .unwrap_or(0);

    let mut current_y = 0;
    for category in categories {
        // Render category header
        if current_y >= scroll && current_y < scroll + inner_area.height as usize {
            let header_area = Rect::new(
                inner_area.x,
                inner_area.y + (current_y - scroll) as u16,
                inner_area.width,
                1,
            );
            render_category_header(&category.name, frame, header_area, colors);
        }
        current_y += 1;

        // Render hints
        for hint in category.hints {
            if current_y >= scroll && current_y < scroll + inner_area.height as usize {
                let hint_area = Rect::new(
                    inner_area.x,
                    inner_area.y + (current_y - scroll) as u16,
                    inner_area.width,
                    1,
                );
                render_hint(hint, frame, hint_area, colors);
            }
            current_y += 1;
        }
        current_y += 1; // Gap between categories
    }
}

fn render_category_header(name: &str, frame: &mut Frame, area: Rect, colors: &ColorScheme) {
    let text = format!("-- {} --", name);
    let paragraph = Paragraph::new(text).style(
        Style::default()
            .fg(colors.highlight)
            .add_modifier(Modifier::BOLD),
    );
    frame.render_widget(paragraph, area);
}

/// renders an individual hint struct
fn render_hint(hint: Hint, frame: &mut Frame, area: Rect, colors: &ColorScheme) {
    let help_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            // Command
            Constraint::Length(15),
            // Description
            Constraint::Min(20),
            // keybindings
            Constraint::Length(25),
        ])
        .split(area);

    let command = Paragraph::new(hint.command).style(Style::default().fg(colors.body_text));
    let description = Paragraph::new(hint.description).style(Style::default().fg(colors.body_text));
    let keybinding = Paragraph::new(hint.keybind).style(Style::default().fg(colors.highlight));
    frame.render_widget(command, help_chunks[0]);
    frame.render_widget(description, help_chunks[1]);
    frame.render_widget(keybinding, help_chunks[2]);
}
