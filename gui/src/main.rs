use app::App;
use log::LevelFilter;
use relm::Widget;

mod app;
mod utils;
mod widgets;

fn main() {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .init();

    notrelm::utils::check_fonts();

    App::run(()).unwrap();
}