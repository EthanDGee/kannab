use crate::io::tui::Tui;
use crate::message;
use crate::message::action::Action;
use crate::model::app_state::AppState;
use color_eyre::eyre::Result;
use crossterm::event::{self};
use std::time::{Duration, Instant};

const EVENT_TICK_RATE: Duration = Duration::new(0, 10);
const SAVE_TICK_RATE: Duration = Duration::new(5, 0);

/// Main application state container
pub struct App {
    pub model: AppState,
    pub last_save_time: Instant,
    pub pending_action: Option<Action>,
}

impl App {
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

    pub fn update(&mut self, mut action: Action) {
        while let Some(next_action) = message::update::update(&mut self.model, action) {
            action = next_action;
        }
    }
}
