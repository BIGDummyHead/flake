use std::{
    sync::{
        LazyLock, Once, OnceLock,
        atomic::{AtomicBool, AtomicI32, AtomicI64, Ordering},
    },
    thread::{self, Thread, ThreadId},
    time::Duration,
};

use sdl3::{
    EventPump, Sdl, VideoSubsystem,
    event::Event,
    render::Canvas,
    video::{Window, WindowBuildError, WindowBuilder},
};

mod settings;

pub use settings::Settings;

use crate::{
    input::{InputType, update_state},
    math::Vec2,
};

// THIS IS NOT SHARED BETWEEN THREADS
// HOWEVER IT IMPLEMENTS SEND AND SYNC FOR REASONS OF STATIC USE ON THE MAIN THREAD!
// DO NOT SHARE BETWEEN THREADS
struct LocalSdl(Sdl);

unsafe impl Sync for LocalSdl {}
unsafe impl Send for LocalSdl {}

static INITIALIZED_SDL: OnceLock<ThreadId> = OnceLock::new();

// instance of SDL if not initialized
static SDL_INSTANCE: OnceLock<LocalSdl> = OnceLock::new();

fn assert_same_thread() -> () {
    if let Some(thread_id) = INITIALIZED_SDL.get() {
        let current_thread_id = thread::current().id();
        //ensure that this is the same thread
        assert!(current_thread_id == *thread_id);
    } else {
        panic!("sdl not initialized correctly");
    }
}

fn sdl_instance() -> &'static Sdl {
    if let Some(instance) = SDL_INSTANCE.get() {
        assert_same_thread();
        &instance.0
    } else {
        //set the thread id
        INITIALIZED_SDL
            .set(thread::current().id())
            .expect("thread id already set");

        let instance = LocalSdl(sdl3::init().expect("sdl failed to initialize"));
        if let Err(_) = SDL_INSTANCE.set(instance) {
            panic!("SDL was already set in once lock");
        }

        sdl_instance()
    }
}

/// # App
///
/// Represents an application for handling video and events.
///
/// Only one instance should ever be created
pub struct App {
    video: VideoSubsystem,
    input: EventPump,
    canvas: Canvas<Window>,
    settings: Settings,
}

impl App {
    /// # Create
    ///
    /// Creates a new application.
    ///
    /// You must provide the `title` of the window. The `size` of the window, and a `builder function` which can help build out the window.
    pub fn create(
        name: &str,
        size: u32,
        builder_fn: impl FnOnce(&mut WindowBuilder) -> (),
    ) -> anyhow::Result<Self> {
        let instance = sdl_instance();

        let video = instance.video()?;
        let input = instance.event_pump()?;

        // create a new window and let the user have full control
        let mut window_builder = video.window(name, size, size);
        builder_fn(&mut window_builder);

        let canvas = window_builder.build()?.into_canvas();

        Ok(Self {
            video,
            input,
            canvas,
            settings: Settings::default(),
        })
    }

    /// # Video
    ///
    /// The video subsystem reference.
    pub fn video(&self) -> &VideoSubsystem {
        &self.video
    }

    /// # Poll Events
    ///
    /// Polls each event in the pump and handles accordingly. Must be called per frame.
    pub fn poll_events(&mut self) -> () {
        for event in self.input.poll_iter() {
            Self::handle_event(event);
        }
    }

    fn handle_event(event: Event) -> () {
        match event {
            Event::KeyDown {
                keycode: Some(keycode),
                repeat,
                ..
            } => {
                if !repeat {
                    update_state(InputType::Key(keycode), true);
                }
            }
            Event::KeyUp {
                keycode: Some(keycode),
                repeat,
                ..
            } => {
                if !repeat {
                    update_state(InputType::Key(keycode), false);
                }
            }

            Event::MouseMotion { x, y, .. } => {
                crate::input::mouse::set_position(Vec2::new(x, y));
            }
            Event::MouseButtonDown { mouse_btn, .. } => {
                update_state(InputType::Mouse(mouse_btn), true);
            }

            Event::MouseButtonUp { mouse_btn, .. } => {
                update_state(InputType::Mouse(mouse_btn), false);
            }

            Event::MouseWheel {
                x, y, direction, ..
            } => todo!(),
            _ => {}
        }
    }

    /// App settings reference (mut)
    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    /// App settings reference
    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Sleeps for the given frame settings for this thread.
    pub fn frame_sleep(&self) -> () {
        thread::sleep(Duration::new(0, 1_000_000_000u32 / self.settings().fps));
    }

    /// A reference to the canvas that was created for the window (mut).
    pub fn canvas_mut(&mut self) -> &mut Canvas<Window> {
        &mut self.canvas
    }

    /// A reference to the canvas that was created for the window.
    pub fn canvas(&self) -> &Canvas<Window> {
        &self.canvas
    }
}
