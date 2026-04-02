//! The main entry point for the kanab application.
//!
//! kanab is a terminal-based Kanban board manager designed for efficiency and ease of use.

mod app;
mod io;
mod message;
mod model;
mod view;
mod widgets;
use app::App;

/// The name of the application.
pub const APP_NAME: &str = "kanab";

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut app = App::new();
    app.run()
}
