use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use rayon::{
    iter::{
        IntoParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
    },
    result::Iter,
};
use sdl3::keyboard::Scancode::Mute;

use crate::game::game_object::GameObject;

pub type GameObjectId = u128;

static GAMEOBJECT_ID_TRACKER: LazyLock<Mutex<GameObjectId>> = LazyLock::new(|| Mutex::new(0));

static GAMEOBJECT_MANAGER: LazyLock<RwLock<ObjectManager>> =
    LazyLock::new(|| RwLock::new(ObjectManager::default()));

/// The object manager
pub(crate) fn object_manager<'go>() -> RwLockReadGuard<'go, ObjectManager> {
    GAMEOBJECT_MANAGER
        .read()
        .expect("failed to obtain game object manager")
}

/// The mut object manager
pub(crate) fn object_manager_mut<'go>() -> RwLockWriteGuard<'go, ObjectManager> {
    GAMEOBJECT_MANAGER
        .write()
        .expect("failed to obtain game object write manager")
}

pub struct ObjectManager {
    objects: HashMap<GameObjectId, GameObject>,
}

impl Default for ObjectManager {
    fn default() -> Self {
        Self {
            objects: HashMap::default(),
        }
    }
}

/// Generates a Game Object id and increments to a new unique ID
fn generate_go_id() -> GameObjectId {
    let mut guard = GAMEOBJECT_ID_TRACKER.lock().expect("poisoned id tracker");

    *guard += 1;

    *guard
}

impl ObjectManager {
    pub fn insert(&mut self, game_object: GameObject) -> &mut GameObject {
        let id = generate_go_id();
        self.objects.insert(id, game_object);

        // acceptable unwrap here
        self.objects.get_mut(&id).unwrap()
    }

    /// # Objects
    ///
    /// The objects within the manager.
    ///
    /// ## Returns
    ///
    /// A rayon parallel iterator reference
    pub fn objects<'go>(
        &'go self,
    ) -> impl rayon::prelude::ParallelIterator<Item = &'go GameObject> {
        self.objects.par_iter().map(|(_, v)| v)
    }

    /// # Objects Mut
    ///
    /// The objects within the manager.
    ///
    /// ## Returns
    ///
    /// A rayon parallel iterator mut reference
    pub fn objects_mut<'go>(
        &'go mut self,
    ) -> impl rayon::prelude::ParallelIterator<Item = &'go mut GameObject> {
        self.objects.par_iter_mut().map(|(_, v)| v)
    }

    /// # Find By Id
    ///
    /// Attempts to find a game object in the manager by a given ID
    ///
    /// ## Returns
    ///
    /// A reference to the gameobject
    pub fn find_by_id(&self, id: GameObjectId) -> Option<&GameObject> {
        self.objects.get(&id)
    }

    /// # Find By Id Mut
    ///
    /// Attempts to find a game object in the manager by a given ID
    ///
    /// ## Returns
    ///
    /// A reference to the gameobject (mut)
    pub fn find_by_id_mut(&mut self, id: GameObjectId) -> Option<&mut GameObject> {
        self.objects.get_mut(&id)
    }
}
