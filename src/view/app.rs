use crate::app::App;
use crate::model::app_state::AppMode;

use ratatui::Frame;

pub fn render(app: &App, frame: &mut Frame) {
    let area = frame.area();

    match app.model.mode {
        AppMode::Picker => {
            crate::view::picker::render(app, frame, area);
        }
        AppMode::Board => {
            crate::view::board::render(app, frame, area);
        }
    }

    // Overlays rendered on top
    if let Some(ref modal) = app.model.modal_state {
        crate::view::modal::render(app, frame, modal, area);
    }

    // help rendering logic can be added here if needed,
    // although help can be part of modal_state
}
