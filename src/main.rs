mod app;
mod io;
mod message;
mod model;
mod view;
use app::App;

pub const APP_NAME: &str = "kanab";

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut app = App::new();
    app.run()
}
