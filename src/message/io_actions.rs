use crate::{message::action::Action, model::app_state::AppState};

pub fn mark_dirty(model: &mut AppState) -> Option<Action> {
    model.pending_changes = true;
    None
}
