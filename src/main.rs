//! The main entry point for the kannab application.
//!
//! kannab is a terminal-based Kanban board manager designed for efficiency and ease of use.

mod app;
mod io;
mod message;
mod model;
mod view;
mod widgets;
use app::App;
use clap::Parser;

/// The name of the application.
pub const APP_NAME: &str = "kannab";

/// A kanban tool built around vim navigation and a polished user experience.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Board to open to (Optional defaults to home/picker screen).
    #[arg(short, long, default_value(None))]
    board_name: Option<String>,
}
/// The main entry point for the application.
///
/// Initializes the application, loads the boards, and starts the TUI loop.
fn main() -> color_eyre::Result<()> {
    // Arguments is only used to be able to print version and about
    #[allow(unused_variables)]
    let args = Args::parse();

    color_eyre::install()?;
    let mut app = App::new();
    app.run()
}
