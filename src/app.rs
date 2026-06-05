use std::{
    sync::{
        LazyLock, Once, OnceLock,
        atomic::{AtomicBool, AtomicI32, AtomicI64, Ordering},
    },
    thread::{self, Thread, ThreadId},
};

use sdl3::{
    EventPump, Sdl, VideoSubsystem,
    event::Event,
    render::Canvas,
    video::{Window, WindowBuildError, WindowBuilder},
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
            _ => todo!(),
        }
    }
}
