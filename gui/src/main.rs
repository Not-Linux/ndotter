use app::App;
use relm::Widget;

mod app;
mod utils;
mod widgets;

fn main() {
    App::run(()).unwrap();
}