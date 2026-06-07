use flake::App;
use sdl3::{keyboard::Keycode, pixels::Color};

fn main() {
    // create a reference to the initialized SDL
    let mut app = App::create("Video Game Title", 600, |win| {
        win.position_centered().resizable();
    })
    .expect("could not create application instance");

    'running: loop {
        app.poll_events();

        {
            let canvas = app.canvas_mut();
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
        }

        // START GAME CODE

        if flake::input::keys::is_down(Keycode::D) {
            break 'running;
        }

        // END GAME CODE

        app.canvas_mut().present();
        flake::input::end_frame();

        app.frame_sleep();
    }
}
