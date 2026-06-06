use std::rc::Rc;

use flake::{App, input};
use sdl3::video::{WindowBuildError, WindowBuilder, WindowFlags};

fn main() {
    // create a reference to the initialized SDL
    let mut app = App::create("Video Game Title", 600, |win| {
        win.position_centered().resizable();
    })
    .expect("could not create application instance");

    loop {
        app.poll_events();

        if input::keys::is_down(sdl3::keyboard::Keycode::A) {
            println!("Down");
        }

        if input::keys::is_held(sdl3::keyboard::Keycode::A) {
            println!("Held");
        }

        if input::keys::is_up(sdl3::keyboard::Keycode::A) {
            println!("Up");
        }

        if input::keys::is_released(sdl3::keyboard::Keycode::A) {
            println!("Re");
        }

        if input::keys::is_down(sdl3::keyboard::Keycode::D) {
            break;
        }

        flake::input::end_frame();

        app.frame_sleep();
    }
}
