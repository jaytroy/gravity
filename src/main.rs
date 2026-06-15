use crate::app::App;

mod planet;
mod triangle;
mod app;
mod renderer;
mod transform;
mod entity;

fn main() {
    let mut app = App::new();

    app.run();
}