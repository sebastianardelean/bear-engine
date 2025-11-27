mod circuit;
mod draw;
mod editor;
mod log;
mod utils;
mod window;

use std::io;

fn main() -> io::Result<()> {
    log::init_logger();

    let window_title: String = String::from("Bear Engine");

    window::create_window(&window_title, 800, 600);

    return Ok(());
}
