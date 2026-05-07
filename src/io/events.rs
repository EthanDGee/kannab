//! Event handling and keyboard input processing.
//!
//! This module translates raw terminal events (primarily key presses) into
//! application-level `Action`s, taking into account the current `AppMode`
//! and active modals.

use crate::app::App;
use crate::message::action::{Action, InputField};
use crate::model::app_state::AppMode;
use crate::model::modal_state::ModalType;
use crate::model::modal_state::{ConfirmDelete, ModalState};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};

/// Top-level event type.
#[allow(dead_code)]
pub enum Event {
    /// A keyboard event.
    Key(KeyEvent),
    /// A mouse event.
    #[allow(dead_code)]
    Mouse(MouseEvent),
    /// A periodic timer event.
    #[allow(dead_code)]
    Tick,
}

/// Dispatches an event to the appropriate handler based on its type.
pub fn handle_event(app: &App, event: Event) -> Option<Action> {
    match event {
        Event::Key(key) => handle_key_event(app, key),
        _ => None,
    }
}

/// Routes key events to either modal handlers or mode-specific handlers.
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

/// Handles keyboard input when a modal overlay is active.
fn handle_modal_key(modal: &ModalState, key: KeyEvent) -> Option<Action> {
    if key.code == KeyCode::Esc {
        return Some(Action::CloseModal);
    }

    match modal.modal_type {
        ModalType::CreateBoard
        | ModalType::EditBoard
        | ModalType::CreateColumn
        | ModalType::RenameColumn
        | ModalType::CreateTask
        | ModalType::EditTask => handle_text_modal_key(modal, key),
        ModalType::ConfirmDelete(_) => confirmation(key),
        ModalType::Help => None,
    }
}

/// Helper for handling text input modals (Board, Column, Task).
fn handle_text_modal_key(modal: &ModalState, key: KeyEvent) -> Option<Action> {
    // Handle Global Modal Hotkeys
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        match key.code {
            // Save and Exit: Ctrl+S or Ctrl+Enter
            KeyCode::Char('s') | KeyCode::Enter => return Some(Action::Confirm),
            // Delete Checklist Item: Ctrl+Backspace or Ctrl+H (only in ItemDescription)
            KeyCode::Backspace | KeyCode::Char('h')
                if modal.focus == InputField::ItemDescription =>
            {
                return Some(Action::DeleteChecklistItem);
            }
            _ => {}
        }
    }

    // Handle Structural/Navigation Keys
    match key.code {
        KeyCode::Tab => Some(Action::SwitchInputField),
        KeyCode::Enter => {
            // Context-sensitive Enter behavior
            match modal.modal_type {
                ModalType::CreateTask | ModalType::EditTask => {
                    if modal.focus == InputField::TaskTitle {
                        // Confirm on Title field
                        return Some(Action::Confirm);
                    }
                    if modal.focus == InputField::ItemDescription {
                        // Toggle completion on Checklist Items
                        return Some(Action::ToggleItemCompletion);
                    }
                    // For TaskDescription, let TextArea handle the newline
                }
                _ => {
                    // For other modals (Board/Column), Enter confirms
                    return Some(Action::Confirm);
                }
            }
            // If we didn't return, pass it to the TextArea (e.g., for newlines in description)
            Some(Action::ModalInput(key))
        }
        // Pass all other keys (Arrows, Backspace, Home/End, etc.) to the TextArea
        _ => Some(Action::ModalInput(key)),
    }
}

/// Processes input for confirmation modals.
fn confirmation(key: KeyEvent) -> Option<Action> {
    if key.code == KeyCode::Enter {
        return Some(Action::Confirm);
    }
    None
}

/// Processes keyboard input when in the board picker mode.
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

/// Processes keyboard input when viewing a specific board.
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
            KeyCode::Tab => Some(Action::ToggleTaskCompletion),

            // Help
            KeyCode::Char('?') => Some(Action::OpenModal(ModalType::Help)),

            // Exit Board mode
            KeyCode::Char('q') | KeyCode::Esc => Some(Action::QuitToPicker),

            _ => None,
        }
    }
}
