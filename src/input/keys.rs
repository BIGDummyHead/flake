use sdl3::keyboard::Keycode;

use crate::input::{InputState, input_state::InputType};

/// The current key's internal state.
pub fn key_state(key: Keycode) -> InputState {
    let key = InputType::Key(key);
    crate::input::state(&key)
}

/// True if the key is pressed this frame
pub fn is_down(key: Keycode) -> bool {
    let state = key_state(key);

    matches!(state, InputState::Pressed)
}

/// True if the key is pressed this frame
pub fn is_held(key: Keycode) -> bool {
    let state = key_state(key);

    matches!(state, InputState::Held)
}

/// True if the key is up for 1 or more frames.
pub fn is_up(key: Keycode) -> bool {
    let state = key_state(key);

    matches!(state, InputState::Up)
}

/// True if the key was released this frame
pub fn is_released(key: Keycode) -> bool {
    let state = key_state(key);

    matches!(state, InputState::Released)
}
