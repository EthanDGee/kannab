use crate::io::tui::Tui;
use crate::message;
use crate::message::action::Action;
use crate::model::app_state::AppState;
use color_eyre::eyre::Result;
use crossterm::event::{self};
use std::time::Duration;

const EVENT_TICK_RATE: Duration = Duration::new(0, 15);
const SAVE_TICK_RATE: Duration = Duration::new(5, 0);

/// Main application state container
pub struct App {
    pub model: AppState,
    pub tick_rate: Duration, // Tick rate for event handling
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let mut model = AppState::new();
        // Add dummy data for visualization
        model
            .board_map
            .insert("Project Alpha".to_string(), "alpha.json".to_string());
        model
            .board_map
            .insert("Personal Tasks".to_string(), "personal.json".to_string());

        App {
            model,
            tick_rate: EVENT_TICK_RATE,
            should_quit: false,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        // initialize
        let mut tui = Tui::new()?.enter()?;

        // run
        loop {
            tui.terminal.draw(|f| crate::view::app::render(self, f))?;
            if event::poll(self.tick_rate)?
                && let event::Event::Key(key) = event::read()?
            {
                let event = crate::io::events::Event::Key(key);
                if let Some(action) = crate::io::events::handle_event(self, event) {
                    match action {
                        Action::Quit => self.should_quit = true,
                        _ => self.update(action),
                    }
                }
            }

            if self.should_quit {
                break;
            }
        }
        tui.exit()?;

        Ok(())
    }

    pub fn update(&mut self, mut action: Action) {
        while let Some(next_action) = message::update::update(&mut self.model, action) {
            action = next_action;
        }
    }
}
