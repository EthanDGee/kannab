mod app;
mod model;
mod update;
mod view;
use app::App;

fn main() {
    println!("Hello, world!");

    let mut app = App::new();

    app.run();
}
