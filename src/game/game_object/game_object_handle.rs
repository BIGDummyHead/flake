use crate::game::{
    game_object::GameObject,
    object_manager::{GameObjectId, object_manager, object_manager_mut},
};

/// # Game Object Handle
///
/// A handle to the game object.
///
/// Simply contains a Game Object ID and allows you to invoke functions with the game object in context.
///
/// ## Notes
///
/// This game object handle should be copied or cloned. It simply has a Game Object ID attached.
#[derive(Copy, Clone, Debug)]
pub struct GameObjectHandle(GameObjectId);

impl GameObjectHandle {
    pub(crate) fn from_object(id: GameObjectId) -> Self {
        Self(id)
    }

    /// # With
    ///
    /// Provides you with the ability to read the game object
    pub fn with(&self, go_fn: impl Fn(&GameObject) -> ()) -> () {
        let guard = object_manager();
        if let Some(game_object) = guard.find_by_id(self.0) {
            go_fn(game_object);
        }
    }

    /// # With Mut
    ///
    /// Provides you with the ability to modify the game object.
    pub fn with_mut(&self, go_fn: impl Fn(&mut GameObject) -> ()) -> () {
        let mut guard = object_manager_mut();
        if let Some(game_object) = guard.find_by_id_mut(self.0) {
            go_fn(game_object);
        }
    }

    /// # Valid
    ///
    /// True if the game object still exist in the object manager.
    pub fn valid(&self) -> bool {
        object_manager().find_by_id(self.0).is_some()
    }
}
