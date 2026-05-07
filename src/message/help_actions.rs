//! Handlers for help related actions.

use crate::{
    message::action::Action,
    model::{app_state::AppState, modal_state::ModalState},
};

/// Opens the help modal window
pub fn open_help_modal(model: &mut AppState) -> Option<Action> {
    let help_modal = ModalState::new(crate::model::modal_state::ModalType::Help);
    model.modal_state = Some(help_modal);
    None
}
