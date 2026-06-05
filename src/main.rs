use std::rc::Rc;

use sdl3::video::{WindowBuildError, WindowBuilder, WindowFlags};

fn main() {
    // create a reference to the initialized SDL
    let sdl_instance = sdl3::init().expect("failed to init sdl");

    let video_subsystem = sdl_instance
        .video()
        .expect("video subsystem failed to initialize");

    let title = "Video Game Title";
    let size = 600;
    let window = video_subsystem
        .window(title, size, size)
        .position_centered()
        .resizable()
        .build()
        .expect("failed to create window");

    let window_canvas = window.into_canvas();
}
