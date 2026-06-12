/// State machine for Game Object
#[derive(Debug, Clone, Copy)]
pub(crate) enum GameObjectState {
    /// The game object was awoken this frame
    Awake,
    /// The game object was started this frame
    Start,
    /// The game object is ready (Update and other states are applied)
    Ready,
    /// The game object is being removed
    Removing,
}

impl GameObjectState {
    /// # Move State
    ///
    /// Moves the state machine forward.
    pub fn move_state(self) -> GameObjectState {
        match self {
            GameObjectState::Awake => GameObjectState::Start,
            GameObjectState::Start => GameObjectState::Ready,
            GameObjectState::Ready => GameObjectState::Ready,
            GameObjectState::Removing => GameObjectState::Removing,
        }
    }
}
