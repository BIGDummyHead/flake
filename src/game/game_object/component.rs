use crate::game::game_object::GameObject;

/// # Component
///
/// Represents an attachable component for a game object.
///
/// Once a game object has an attached component
pub trait Component {
    /// # Awake
    ///
    /// Called as soon as the game object is recognized within an Object Manager.
    ///
    /// You should note that all game object will NOT be initialized at the time of this call.
    ///
    /// See `start` for a function that implements this behaviour
    fn awake(&self, go: &mut GameObject);

    /// # Start
    ///
    /// Called as soon as all game objects have been awoken.
    ///
    /// You should note that all game objects will be initialized at the time of this call.
    ///
    /// This allows for you to search for game objects.
    fn start(&self, go: &mut GameObject);

    /// # Update
    ///
    /// Called each frame of the game object.
    fn update(&self, go: &mut GameObject);

    /// # Remove
    ///
    /// Called once the component is removed from a game object.
    fn remove(&self, go: &mut GameObject);
}
