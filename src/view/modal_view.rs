//! View components for rendering modal dialogs and overlays.

use crate::message::action::InputField;
use crate::model::modal_state::{ConfirmDelete, ModalState};
use crate::view::board_view::board_modal_view;
use crate::view::column_view::column_modal_view;
use crate::view::task_view::task_modal_view;
use crate::widgets::floating_window::centered_rect;
use crate::widgets::text_input::TextInput;
use crate::{app::App, model::modal_state::ModalType};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph},
};

/// The primary render function for modals.
///
/// Dispatches the rendering task to a specialized function based on the modal's type.
pub fn render(app: &App, frame: &mut Frame, modal: &ModalState, area: Rect) {
    match modal.modal_type {
        ModalType::CreateBoard => board_modal_view(
            app,
            frame,
            modal,
            area,
            " Create New Board ",
            "Press Enter to create, Esc to cancel",
        ),
        ModalType::EditBoard => board_modal_view(
            app,
            frame,
            modal,
            area,
            " Rename Board ",
            "Press Enter to rename, Esc to cancel",
        ),
        ModalType::CreateColumn => column_modal_view(
            app,
            frame,
            modal,
            area,
            " Create New Column ",
            "Press Enter to create, Esc to cancel",
        ),
        ModalType::RenameColumn => column_modal_view(
            app,
            frame,
            modal,
            area,
            " Rename Column ",
            "Press Enter to rename, Esc to cancel",
        ),
        ModalType::CreateTask => task_modal_view(
            app,
            frame,
            modal,
            area,
            " Create New Task ",
            "Tab: Switch fields | Enter: Create | Esc: Cancel",
        ),
        ModalType::EditTask => task_modal_view(
            app,
            frame,
            modal,
            area,
            " Edit Task ",
            "Tab: Switch fields | Enter: Save | Esc: Cancel",
        ),
        ModalType::ConfirmDelete(_) => confirm_delete(app, frame, modal, area),
        ModalType::Help => {
            todo!("Finish implementing the  Modal ")
        }
    }
}

/// Renders a confirmation dialog for deleting an item.
fn confirm_delete(app: &App, frame: &mut Frame, modal: &ModalState, area: Rect) {
    // --- Setup: Get colors and calculate centered area ---
    let colors = &app.model.color_scheme;
    let area = centered_rect(35, 10, area);
    frame.render_widget(Clear, area);

    // --- Extract target from modal ---
    let target = match modal.modal_type {
        ModalType::ConfirmDelete(t) => t,
        _ => return,
    };

    // --- Lookup: Get item title based on target type ---
    let (delete_type, item_title) = match target {
        ConfirmDelete::Board => {
            let idx = app.model.picker_state.index;
            let title = app
                .model
                .board_list
                .get(idx)
                .map(|b| b.title.as_str())
                .unwrap_or("this board");
            ("Board", title.to_string())
        }
        ConfirmDelete::Column => {
            let board_state = app.model.board_state.as_ref();
            let col_idx = board_state.map(|b| b.column_index).unwrap_or(0);
            let title = board_state
                .and_then(|board| board.board.columns.get(col_idx))
                .map(|col| col.title.as_str())
                .unwrap_or("this column");
            ("Column", title.to_string())
        }
        ConfirmDelete::Task => {
            let board_state = app.model.board_state.as_ref();
            let col_idx = board_state.map(|b| b.column_index).unwrap_or(0);
            let task_idx = board_state.map(|b| b.task_index).unwrap_or(0);
            let title = board_state
                .and_then(|board| board.board.columns.get(col_idx))
                .and_then(|col| col.tasks.get(task_idx))
                .map(|task| task.title.as_str())
                .unwrap_or("this task");
            ("Task", title.to_string())
        }
    };

    // boarded with the label based on the delete_type
    let block = Block::default()
        .title(format!(" Delete {} ", delete_type))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors.highlight))
        .style(Style::default().bg(colors.background).fg(colors.body_text));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    //  Build the confirmation message to contain name and type
    let question = format!(
        "Delete the \"{}\" {}?",
        item_title,
        delete_type.to_lowercase()
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // label
            Constraint::Min(1),    // confirmation question
            Constraint::Max(1),    // instructions
        ])
        .split(inner_area);

    let question_text = Paragraph::new(question)
        .style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(colors.highlight_text),
        )
        .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(question_text, chunks[1]);

    let instructions = Paragraph::new("Enter to confirm, Esc to cancel")
        .style(Style::default().fg(colors.inner_border))
        .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(instructions, chunks[2]);
}
