use crate::model::app_state::AppState;
use std::time::Duration;

const TICK_RATE: Duration = Duration::new(5, 0);

/// Main application state container
pub struct App {
    pub model: AppState,
    pub tick_rate: Duration, // Tick rate for auto-save
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            model: AppState::new(),
            tick_rate: TICK_RATE,
            should_quit: false,
        }
    }

    pub fn run(&mut self) {
        // initialize

        // run
        loop {
            if self.should_quit {
                break;
            }

            // render
        }
    }

    fn update(&mut self) {
        todo!("Implement event handling")
    }
}
