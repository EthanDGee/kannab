//! TUI (Terminal User Interface) management and lifecycle.
//!
//! This module handles the initialization, entry, and exit of the terminal interface,
//! ensuring that the terminal state is correctly restored even in case of panics.

use color_eyre::Result;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{cursor, execute};

use ratatui::{Terminal, prelude::CrosstermBackend};
use std::io::{Stdout, stdout};

/// A wrapper around the Ratatui terminal that manages its lifecycle.
pub struct Tui {
    /// The underlying Ratatui terminal instance.
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    /// Creates a new TUI instance with a Crossterm backend.
    pub fn new() -> Result<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(Tui { terminal })
    }

    /// Prepares the terminal for the application (enters alternate screen, hides cursor).
    pub fn enter(mut self) -> Result<Self> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen, cursor::Hide)?;
        self.terminal.hide_cursor()?;
        Ok(self)
    }

    /// Restores the terminal to its original state (leaves alternate screen, shows cursor).
    pub fn exit(&mut self) -> Result<()> {
        execute!(stdout(), LeaveAlternateScreen, cursor::Show)?;
        disable_raw_mode()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        // Ensure terminal is restored on panic
        let _ = self.exit();
    }
}
