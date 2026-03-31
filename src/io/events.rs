use crossterm::event::{KeyEvent, MouseButton, MouseEvent};

use crate::{app::App, message::action::Action, model::app_state::AppMode};

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

pub fn handle_key_event(app: &App, key: KeyEvent) -> Option<Action> {
    let mode = &app.model.mode;

    match mode {
        AppMode::Picker => {}
        AppMode::Board => {}
    }
    None
}
