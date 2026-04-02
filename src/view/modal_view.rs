use std::f32::DIGITS;

use crate::io::file_handling::delete_board;
use crate::model::modal_state::{ConfirmDelete, ModalState};
use crate::{app::App, model::modal_state::ModalType};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph},
};

pub fn render(app: &App, frame: &mut Frame, modal: &ModalState, area: Rect) {
    match modal.modal_type {
        ModalType::CreateBoard => create_board_view(app, frame, modal, area),
        ModalType::EditBoard => {
            todo!("Finish implementing the  Modal ")
        }
        ModalType::CreateColumn => create_column_view(app, frame, modal, area),
        ModalType::RenameColumn => {
            todo!("Finish implementing the  Modal ")
        }
        ModalType::CreateTask => {
            todo!("Finish implementing the  Modal ")
        }
        ModalType::EditTask => {
            todo!("Finish implementing the  Modal ")
        }
        ModalType::ConfirmDelete(ConfirmDelete) => confirm_delete(app, frame, modal, area),
        ModalType::Help => {
            todo!("Finish implementing the  Modal ")
        }
    }
}

fn create_board_view(app: &App, frame: &mut Frame, modal: &ModalState, area: Rect) {
    let colors = &app.model.color_scheme;
    let area = centered_rect(60, 15, area);

    frame.render_widget(Clear, area); //this clears out the background

    let block = Block::default()
        .title(" Create New Board ")
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

    let label = Paragraph::new("Board Name:").style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(label, chunks[0]);

    let input = Paragraph::new(modal.data.board_name.as_str()).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(colors.inner_border)),
    );
    frame.render_widget(input, chunks[1]);

    let instructions = Paragraph::new("Press Enter to create, Esc to cancel")
        .style(Style::default().fg(colors.inner_border));
    frame.render_widget(instructions, chunks[2]);
}

fn create_column_view(app: &App, frame: &mut Frame, modal: &ModalState, area: Rect) {
    let colors = &app.model.color_scheme;
    let area = centered_rect(60, 15, area);

    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(" Create New Column ")
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

    let label = Paragraph::new("Column Title:").style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(label, chunks[0]);

    let input = Paragraph::new(modal.data.column_name.as_str()).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(colors.inner_border)),
    );
    frame.render_widget(input, chunks[1]);

    let instructions = Paragraph::new("Press Enter to create, Esc to cancel")
        .style(Style::default().fg(colors.inner_border));
    frame.render_widget(instructions, chunks[2]);
}

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
            Constraint::Length(1), // instructions
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

/// helper function to create a centered rect using up to certain % of available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
