use crate::model::{
    app_state::{AppMode, AppState},
    modal_state::{self, ModalType},
};
use ratatui::Frame;

pub fn view(app: &AppState, frame: &mut Frame) {
    let area = frame.area();

    match app.mode {
        AppMode::Picker => {}
        AppMode::Board => {}
    }
}
