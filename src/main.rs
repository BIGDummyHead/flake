use flake::game::{
    Game,
    game_object::{Component, GameObject},
};
use rayon::iter::ParallelIterator;
use sdl3::{keyboard::Keycode, pixels::Color};

#[derive(Debug, Default, Clone, Copy)]
struct MyComponent {}

impl Component for MyComponent {
    fn awake(&self, go: &mut GameObject) {}

    fn start(&self, go: &mut GameObject) {}

    fn update(&self, go: &mut GameObject) {}

    fn remove(&self, go: &mut GameObject) {}
}

fn main() {
    // create a reference to the initialized SDL
    let mut app = Game::create("Video Game Title", 600, |win| {
        win.position_centered().resizable();
    })
    .expect("could not create application instance");

    let go_id = GameObject::new("test", None);
    go_id.with_mut(|g| {
        let c = MyComponent::default();
        g.add_component(c);
    });

    let poll_fn = |go: &mut GameObject| {
        go.poll();
    };

    flake::game::object_manager_mut()
        .objects_mut()
        .for_each(poll_fn);

    loop {
        app.poll_events();

        {
            let canvas = app.canvas_mut();
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
        }

        // START GAME CODE
        flake::game::object_manager_mut()
            .objects_mut()
            .for_each(poll_fn);
        // END GAME CODE

        if flake::input::keys::is_held(Keycode::D) {
            break;
        }

        app.canvas_mut().present();
        flake::input::end_frame();

        app.frame_sleep();
    }
}
