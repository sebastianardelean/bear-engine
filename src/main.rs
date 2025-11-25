//https://github.com/imgui-rs/imgui-rs
//https://crates.io/crates/sdl2




mod window;
mod log;
mod draw;
mod circuit;
mod editor;
mod utils;

use std::io;


fn main() -> io::Result<()>{

    log::init_logger();

    let window_title:String = String::from("QC Schematics");
    
    window::create_window(&window_title,800,600);
    
    return Ok(());

}
