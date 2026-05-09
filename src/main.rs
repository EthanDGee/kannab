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

/// The cli arguments for launching/information.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Open to a board directly (Optional defaults to home/picker screen).
    #[arg(short, long, default_value(None))]
    board_name: Option<String>,

    /// List all available boards
    #[arg(short, long)]
    list_boards: bool,
}
/// The main entry point for the application.
///
/// Initializes the application, loads the boards, and starts the TUI loop.
fn main() -> color_eyre::Result<()> {
    // Arguments is only used to be able to print version and about
    let args = Args::parse();

    // list all boards
    if args.list_boards {
        if let Some(boards) = crate::io::file_handling::load_board_list() {
            if boards.is_empty() {
                println!("No boards found.");
            } else {
                println!("Available boards:");
                for board in boards {
                    println!("- {}", board.title);
                }
            }
        } else {
            println!("No boards found.");
        }
        return Ok(());
    }

    color_eyre::install()?;
    let mut app = App::new();

    // attempt to open board directly
    if let Some(name) = args.board_name {
        if let Some(index) = app.model.board_list.iter().position(|b| b.title == name) {
            app.model.picker_state.index = index;
            app.update(crate::message::action::Action::OpenBoard);
        } else {
            eprintln!("Board '{}' not found.", name);
            return Ok(());
        }
    }
    app.run()
}
