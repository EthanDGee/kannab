use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode};

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
            tick_rate: TICK_RATE,
            should_quit: false,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        // initialize
        let mut tui = Tui::new()?.enter()?;

        // run
        while !self.should_quit {
            tui.terminal.draw(|f| crate::view::app::render(self, f))?;

            // Simple event handling to allow quitting and prevent high CPU usage
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                        // Basic navigation just to see it works
                        KeyCode::Down | KeyCode::Char('j') => {
                            self.model.picker_state.index = (self.model.picker_state.index + 1)
                                % self.model.board_map.len().max(1);
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            if self.model.picker_state.index > 0 {
                                self.model.picker_state.index -= 1;
                            } else {
                                self.model.picker_state.index =
                                    self.model.board_map.len().saturating_sub(1);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        if self.model.pending_changes {
            self.update(Action::Save);
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
