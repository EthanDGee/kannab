use crate::message::action::{Action, InputField};
use crate::model::app_state::AppMode;
use crate::model::modal_state::{ConfirmDelete, ModalState};
use crate::{app::App, model::modal_state::ModalType};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};

pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Tick,
}

pub fn handle_event(app: &App, event: Event) -> Option<Action> {
    match event {
        Event::Key(key) => handle_key_event(app, key),

        _ => None,
    }
}

fn handle_key_event(app: &App, key: KeyEvent) -> Option<Action> {
    if let Some(modal) = &app.model.modal_state {
        return handle_modal_key(modal, key);
    }

    let mode = &app.model.mode;

    match mode {
        AppMode::Picker => handle_picker_keys(key),
        AppMode::Board => handle_board_keys(key),
    }
}

fn handle_modal_key(modal: &ModalState, key: KeyEvent) -> Option<Action> {
    if key.code == KeyCode::Esc {
        return Some(Action::CloseModal);
    }

    match modal.modal_type {
        ModalType::CreateBoard | ModalType::EditBoard => {
            single_line_modal(key, InputField::BoardTitle, modal.data.board_title.clone())
        }
        ModalType::CreateColumn | ModalType::RenameColumn => {
            single_line_modal(key, InputField::ColumnTitle, modal.data.column_title.clone())
        }
        ModalType::CreateTask | ModalType::EditTask => handle_task_creation(modal, key),
        ModalType::ConfirmDelete(_) => confirmation(key),
        ModalType::Help => None,
    }
}

/// Helper for handling task-related text input (title and description).
fn handle_task_creation(modal: &ModalState, key: KeyEvent) -> Option<Action> {
    // Handle hot keys
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        match key.code {
            // Save and Exit: Ctrl+S or Ctrl+Enter
            KeyCode::Char('s') | KeyCode::Enter => return Some(Action::Confirm),
            _ => return None,
        }
    }

    match key.code {
        KeyCode::Tab => Some(Action::SwitchInputField),
        // Submit the task / Newline support
        KeyCode::Enter => {
            if modal.focus == InputField::TaskTitle {
                Some(Action::Confirm)
            } else {
                // Insert newline in description
                let mut current_text = modal.data.task_description.clone();
                current_text.push('\n');
                Some(Action::UpdateField(
                    InputField::TaskDescription,
                    current_text,
                ))
            }
        }
        _ => {
            let (field, current_text) = match modal.focus {
                InputField::TaskTitle => (InputField::TaskTitle, modal.data.task_title.clone()),
                InputField::TaskDescription => (
                    InputField::TaskDescription,
                    modal.data.task_description.clone(),
                ),
                _ => (InputField::TaskTitle, modal.data.task_title.clone()),
            };
            update_text_field(key, field, current_text)
        }
    }
}

/// Helper for handling single line text edit modals
fn single_line_modal(key: KeyEvent, field: InputField, current_text: String) -> Option<Action> {
    if key.code == KeyCode::Enter {
        return Some(Action::Confirm);
    }
    update_text_field(key, field, current_text)
}

/// Helper for handling text input fields within modals.
fn update_text_field(key: KeyEvent, field: InputField, mut current_text: String) -> Option<Action> {
    match key.code {
        KeyCode::Char(c) => {
            current_text.push(c);
            Some(Action::UpdateField(field, current_text))
        }
        KeyCode::Backspace => {
            current_text.pop();
            Some(Action::UpdateField(field, current_text))
        }
        _ => None,
    }
}

fn confirmation(key: KeyEvent) -> Option<Action> {
    if key.code == KeyCode::Enter {
        return Some(Action::Confirm);
    }
    None
}

fn handle_picker_keys(key: KeyEvent) -> Option<Action> {
    let shift = key.modifiers.contains(KeyModifiers::SHIFT);
    if shift {
        match key.code {
            // Board Rearrangement
            KeyCode::Up | KeyCode::Char('K') => Some(Action::MoveBoardUp),
            KeyCode::Down | KeyCode::Char('J') | KeyCode::Tab => Some(Action::MoveBoardDown),
            _ => None,
        }
    } else {
        match key.code {
            // Navigation
            KeyCode::Up | KeyCode::Char('k') => Some(Action::MoveUp),
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Tab => Some(Action::MoveDown),

            // Actions
            KeyCode::Char('c') | KeyCode::Char('n') => {
                Some(Action::OpenModal(ModalType::CreateBoard))
            }
            KeyCode::Char('r') | KeyCode::Char('e') => {
                Some(Action::OpenModal(ModalType::EditBoard))
            }
            KeyCode::Char('d') => Some(Action::OpenModal(ModalType::ConfirmDelete(
                ConfirmDelete::Board,
            ))),

            // Open selected board
            KeyCode::Enter => Some(Action::OpenBoard),

            // Help
            KeyCode::Char('?') => Some(Action::OpenModal(ModalType::Help)),

            // Quit
            KeyCode::Char('q') | KeyCode::Esc => Some(Action::Quit),

            _ => None,
        }
    }
}

fn handle_board_keys(key: KeyEvent) -> Option<Action> {
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
    let shift = key.modifiers.contains(KeyModifiers::SHIFT);

    if ctrl {
        match key.code {
            // Column Reordering
            KeyCode::Left | KeyCode::Char('h') => Some(Action::MoveColumnLeft),
            KeyCode::Right | KeyCode::Char('l') => Some(Action::MoveColumnRight),

            // Column Creation
            KeyCode::Char('n') => Some(Action::OpenModal(ModalType::CreateColumn)),

            // Column Renaming
            KeyCode::Char('r') => Some(Action::OpenModal(ModalType::RenameColumn)),

            // Delete Column
            KeyCode::Char('d') => Some(Action::OpenModal(ModalType::ConfirmDelete(
                ConfirmDelete::Column,
            ))),

            _ => None,
        }
    } else if shift {
        match key.code {
            // Task reordering
            KeyCode::Up | KeyCode::Char('K') => Some(Action::MoveTaskUp),
            KeyCode::Down | KeyCode::Char('J') => Some(Action::MoveTaskDown),
            KeyCode::Left | KeyCode::Char('H') => Some(Action::MoveTaskToPrevColumn),
            KeyCode::Right | KeyCode::Char('L') => Some(Action::MoveTaskToNextColumn),

            // Column Actions
            KeyCode::Char('C') => Some(Action::OpenModal(ModalType::CreateColumn)),
            KeyCode::Char('E') => Some(Action::OpenModal(ModalType::RenameColumn)),
            KeyCode::Char('D') => Some(Action::OpenModal(ModalType::ConfirmDelete(
                ConfirmDelete::Column,
            ))),

            _ => None,
        }
    } else {
        match key.code {
            // Navigation
            KeyCode::Left | KeyCode::Char('h') => Some(Action::MoveLeft),
            KeyCode::Right | KeyCode::Char('l') => Some(Action::MoveRight),
            KeyCode::Up | KeyCode::Char('k') => Some(Action::MoveUp),
            KeyCode::Down | KeyCode::Char('j') => Some(Action::MoveDown),

            // Column Renaming (also allow 'r' if not used for anything else)
            KeyCode::Char('r') => Some(Action::OpenModal(ModalType::RenameColumn)),

            // Create Task
            KeyCode::Char('n') | KeyCode::Char('c') => {
                Some(Action::OpenModal(ModalType::CreateTask))
            }

            // Edit Task
            KeyCode::Char('e') => Some(Action::OpenModal(ModalType::EditTask)),

            // Delete Task
            KeyCode::Char('d') => Some(Action::OpenModal(ModalType::ConfirmDelete(
                ConfirmDelete::Task,
            ))),

            // Toggle Completion
            KeyCode::Tab => Some(Action::ToggleCompletion),

            // Help
            KeyCode::Char('?') => Some(Action::OpenModal(ModalType::Help)),

            // Exit Board mode
            KeyCode::Char('q') | KeyCode::Esc => Some(Action::QuitToPicker),

            _ => None,
        }
    }
}
