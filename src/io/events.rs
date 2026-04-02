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
        ModalType::CreateBoard => handle_create_board_key(modal, key),
        ModalType::ConfirmDelete(_) => handle_confirm_delete_key(key),
        _ => None,
    }
}

fn handle_confirm_delete_key(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Enter => Some(Action::Confirm),
        _ => None,
    }
}

fn handle_create_board_key(modal: &ModalState, key: KeyEvent) -> Option<Action> {
    let mut current_name = modal.data.board_name.clone();

    match key.code {
        KeyCode::Enter => Some(Action::Confirm),
        KeyCode::Char(c) => {
            current_name.push(c);
            Some(Action::UpdateField(InputField::BoardName, current_name))
        }
        KeyCode::Backspace => {
            // remove the last character of the board_name
            current_name.pop();
            Some(Action::UpdateField(InputField::BoardName, current_name))
        }
        _ => None,
    }
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
            _ => None,
        }
    } else {
        match key.code {
            // Navigation
            KeyCode::Left | KeyCode::Char('h') => Some(Action::MoveLeft),
            KeyCode::Right | KeyCode::Char('l') => Some(Action::MoveRight),
            KeyCode::Up | KeyCode::Char('k') => Some(Action::MoveUp),
            KeyCode::Down | KeyCode::Char('j') => Some(Action::MoveDown),

            // Create Task
            KeyCode::Char('n') => Some(Action::OpenModal(ModalType::CreateTask)),

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
