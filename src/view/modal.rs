use crate::model::modal_state::ModalState;
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
        ModalType::EditBoard => {}
        ModalType::CreateColumn => {}
        ModalType::RenameColumn => {}
        ModalType::CreateTask => {}
        ModalType::EditTask => {}
        ModalType::ConfirmDelete(_) => {}
        ModalType::Help => {}
    }
}

fn create_board_view(app: &App, frame: &mut Frame, modal: &ModalState, area: Rect) {
    let colors = &app.model.color_scheme;
    let area = centered_rect(60, 20, area);

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
            Constraint::Min(0),    // Instructions
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
