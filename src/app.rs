//! The core application module managing the main loop and high-level state.
//!
//! This module defines the `App` struct which orchestrates the TUI lifecycle,
//! event handling, and model-view updates.

use crate::io::tui::Tui;
use crate::message;
use crate::message::action::Action;
use crate::model::app_state::AppState;
use color_eyre::eyre::Result;
use crossterm::event::{self};
use std::time::{Duration, Instant};

/// Interval for checking input events (10 milliseconds).
const EVENT_TICK_RATE: Duration = Duration::new(0, 10);

/// Interval for auto-saving (5 seconds).
const SAVE_TICK_RATE: Duration = Duration::new(5, 0);

/// The main application container that orchestrates state, TUI, and event handling.
pub struct App {
    /// The global application state.
    pub model: AppState,
    /// The timestamp of the last successful auto-save.
    pub last_save_time: Instant,
    /// An action waiting to be processed by the update loop.
    pub pending_action: Option<Action>,
}

impl App {
    /// Initializes a new application instance, loading existing boards from disk.
    pub fn new() -> Self {
        let mut model = AppState::new();
        // Load boards from disk
        if let Some(boards) = crate::io::file_handling::load_board_list() {
            model.board_list = boards;
        }

        App {
            model,
            last_save_time: Instant::now(),
            pending_action: None,
        }
    }

    /// Enters the main application loop, handling rendering and events until shutdown.
    ///
    /// # Errors
    /// Returns an error if the TUI fails to initialize or the loop encounters a fatal issue.
    pub fn run(&mut self) -> Result<()> {
        // initialize
        let mut tui = Tui::new()?.enter()?;

        // run
        loop {
            tui.terminal
                .draw(|f| crate::view::app_view::render(self, f))?;

            if let event::Event::Key(key) = event::read()? {
                let event = crate::io::events::Event::Key(key);
                self.pending_action = crate::io::events::handle_event(self, event);
            }

            while let Some(action) = self.pending_action.take() {
                self.pending_action = message::update::update(&mut self.model, action);
            }

            if self.last_save_time.elapsed() >= SAVE_TICK_RATE {
                self.last_save_time = Instant::now();
                self.update(Action::Tick);
            }

            if self.model.should_quit {
                break;
            }
        }
        tui.exit()?;

        Ok(())
    }

    /// Processes an action and all subsequent triggered actions until completion.
    pub fn update(&mut self, mut action: Action) {
        while let Some(next_action) = message::update::update(&mut self.model, action) {
            action = next_action;
        }
    }
}
