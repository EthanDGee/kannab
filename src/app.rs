use color_eyre::eyre::Result;

use crate::io::tui::Tui;
use crate::message;
use crate::message::action::Action;
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

    pub fn run(&mut self) -> Result<()> {
        // initialize
        let mut terminal = Tui::new()?;

        // run
        loop {
            terminal.render(self.model);

            self.update(Action::Render);

            if self.should_quit {
                break;
            }

            // render
        }

        if self.model.pending_changes {
            self.update(Action::Save);
        }
        terminal.exit();

        Ok(())
    }

    pub fn update(&mut self, mut action: Action) {
        while let Some(next_action) = message::update::update(&mut self.model, action) {
            action = next_action;
        }
    }
}
