use std::sync::{LazyLock, Mutex, RwLock};

use sdl3::mouse::MouseButton;

use crate::{
    input::{InputState, input_state::InputType},
    math::Vec2,
};

mod scroll;
pub use scroll::Scroll;

/// The current mouse_button's internal state.
pub fn mouse_state(mouse_button: MouseButton) -> InputState {
    let mouse_button = InputType::Mouse(mouse_button);
    crate::input::state(&mouse_button)
}

/// True if the mouse_button is pressed this frame
pub fn is_down(mouse_button: MouseButton) -> bool {
    let state = mouse_state(mouse_button);

    matches!(state, InputState::Pressed)
}

/// True if the mouse_button is pressed this frame
pub fn is_held(mouse_button: MouseButton) -> bool {
    let state = mouse_state(mouse_button);

    matches!(state, InputState::Pressed) || matches!(state, InputState::Held)
}

/// True if the mouse_button is up for 1 or more frames.
pub fn is_up(mouse_button: MouseButton) -> bool {
    let state = mouse_state(mouse_button);

    matches!(state, InputState::Released) || matches!(state, InputState::Up)
}

/// True if the mouse_button was released this frame
pub fn is_released(mouse_button: MouseButton) -> bool {
    let state = mouse_state(mouse_button);

    matches!(state, InputState::Released)
}

static MOUSE_POSITION: LazyLock<RwLock<Vec2>> = LazyLock::new(|| RwLock::new(Vec2::default()));

/// The position of the mouse
pub fn position() -> Vec2 {
    let pos = MOUSE_POSITION
        .read()
        .expect("failed to read mouse position: ");

    *pos
}

/// Sets the position of the global mouse.
pub(crate) fn set_position(pos: Vec2) -> () {
    *MOUSE_POSITION
        .write()
        .expect("faile to write to mouse position: ") = pos;
}


pub fn on_scroll(scroll_event: impl Fn(Scroll))
