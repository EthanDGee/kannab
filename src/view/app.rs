use crate::model::app_state::AppState;
use ratatui::{Frame, widgets::Paragraph};

pub fn view(App: &AppState, frame: &mut Frame) {
    let area = frame.area();

    let testing = Paragraph::new("HELLO WORLD");
    frame.render_widget(testing, area);
}
