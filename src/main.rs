mod app;
mod message;
mod model;
mod view;
use app::App;

fn main() {
    println!("Hello, world!");

    let mut app = App::new();

    app.run();
}
