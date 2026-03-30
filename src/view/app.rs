use crate::app::App;
use crate::model::app_state::AppMode;

use ratatui::Frame;

pub fn view(app: &App, frame: &mut Frame) {
    match app.model.mode {
        AppMode::Picker => {
            crate::view::picker::render(app, frame, frame.area());
        }
        AppMode::Board => {}
    }
}
