
extern crate tempfile;
extern crate clap;

mod editor;
mod app;

use app::application::Application;

fn main() {
    let app = Application::new();
    let result  = app.run();
}
