//! The root view component that coordinates rendering of the entire application.

use crate::app::App;
use crate::model::app_state::AppMode;

use ratatui::Frame;

/// The top-level render function.
///
/// This function determines which main view to render based on the current `AppMode`
/// and ensures that any active modals are drawn as overlays.
pub fn render(app: &App, frame: &mut Frame) {
    let area = frame.area();

    match app.model.mode {
        AppMode::Picker => {
            crate::view::picker_view::render(app, frame, area);
        }
        AppMode::Board => {
            crate::view::board_view::render(app, frame, area);
        }
    }

    // Overlays rendered on top
    if let Some(ref modal) = app.model.modal_state {
        crate::view::modal_view::render(app, frame, modal, area);
    }

    // help rendering logic can be added here if needed,
    // although help can be part of modal_state
}
