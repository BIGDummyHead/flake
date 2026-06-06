use sdl3::{keyboard::Keycode, mouse::MouseButton};

/// Key State of a key code at a given frame
#[derive(Debug, Clone, Copy)]
pub enum InputState {
    /// The key was pressed this frame
    Pressed,
    /// The key has been pressed for more than 1 frame
    Held,
    /// The key was released this frame
    Released,
    /// The Key has been up for more than 1 frame
    Up,
}

/// Types of input
#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InputType {
    Key(Keycode),
    Mouse(MouseButton),
}

impl Default for InputState {
    fn default() -> Self {
        InputState::Up
    }
}

impl InputState {
    /// # Progress State
    ///
    /// Progresses the state of the current key state
    ///
    /// ## State Transformation:
    ///
    /// * `Pressed` (is_pressed) -> `Held`
    /// * `Held` (is_pressed) -> `Held`
    /// * `Pressed/Held` (!is_pressed) -> `Released`
    /// * `Released` (!is_pressed) -> `Up`
    /// * `Up` (!is_pressed) -> `Up`
    /// * `Released/Up` (is_pressed) -> `Pressed`
    ///
    pub fn progress_state(self, is_pressed: bool) -> InputState {
        match (self, is_pressed) {
            (InputState::Pressed, true) => InputState::Held,
            (InputState::Held, true) => InputState::Held,

            (InputState::Pressed, false) | (InputState::Held, false) => InputState::Released,

            (InputState::Released, false) => InputState::Up,
            (InputState::Up, false) => InputState::Up,

            (InputState::Released, true) | (InputState::Up, true) => InputState::Pressed,
        }
    }
}
