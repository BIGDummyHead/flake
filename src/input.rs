use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock},
};

mod input_state;
pub use input_state::InputState;

pub(crate) use crate::input::input_state::InputType;

pub mod keys;
pub mod mouse;

static INPUT_STATES: LazyLock<RwLock<HashMap<InputType, InputState>>> =
    LazyLock::new(|| RwLock::new(HashMap::default()));

/// Updates the state of the input type
pub(crate) fn update_state(code: InputType, is_pressed: bool) -> () {
    let new_state = state(&code).progress_state(is_pressed);
    INPUT_STATES
        .write()
        .expect("write lock for input states not acquired")
        .insert(code.clone(), new_state);
}

/// # End Frame
///
/// Ends the input frame.
///
/// If the state of a key or mouse was pressed the state is switched to Held
///
/// If the state of a key or mouse was Released the state is switched to Up
pub fn end_frame() {
    let mut states = INPUT_STATES.write().unwrap();

    for state in states.values_mut() {
        *state = match *state {
            InputState::Pressed => InputState::Held,
            InputState::Released => InputState::Up,
            s => s,
        };
    }
}

/// Returns the current state of the given key code
pub(crate) fn state(code: &InputType) -> InputState {
    initialize_input_type(code);
    INPUT_STATES
        .read()
        .expect("read lock for input states not acquire")
        .get(code)
        .expect("key not initialized")
        .clone()
}

/// Sets the key code value if not set. Ensuring value is always set
fn initialize_input_type(code: &InputType) -> () {
    let mut input_guard = INPUT_STATES
        .write()
        .expect("default write failed to get lock");
    if let None = input_guard.get(code) {
        input_guard.insert(code.clone(), InputState::default());
    }
}
