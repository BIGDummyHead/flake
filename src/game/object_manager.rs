use std::{
    collections::HashMap,
    rc::Rc,
    sync::{Arc, LazyLock, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use rayon::{
    iter::{
        IntoParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
    },
    result::Iter,
};
use sdl3::keyboard::Scancode::Mute;

use crate::game::game_object::{GameObject, GameObjectHandle};

pub type GameObjectId = u128;

static GAMEOBJECT_ID_TRACKER: LazyLock<Mutex<GameObjectId>> = LazyLock::new(|| Mutex::new(0));

static GAMEOBJECT_MANAGER: LazyLock<RwLock<ObjectManager>> =
    LazyLock::new(|| RwLock::new(ObjectManager::new()));

/// The object manager
pub fn object_manager<'go>() -> RwLockReadGuard<'go, ObjectManager> {
    GAMEOBJECT_MANAGER
        .read()
        .expect("failed to obtain game object manager")
}

/// The mut object manager
pub fn object_manager_mut<'go>() -> RwLockWriteGuard<'go, ObjectManager> {
    GAMEOBJECT_MANAGER
        .write()
        .expect("failed to obtain game object write manager")
}

pub struct ObjectManager {
    objects: HashMap<GameObjectId, GameObject>,
}

/// Generates a Game Object id and increments to a new unique ID
fn generate_go_id() -> GameObjectId {
    let mut guard = GAMEOBJECT_ID_TRACKER.lock().expect("poisoned id tracker");

    *guard += 1;

    *guard
}

impl ObjectManager {
    pub(crate) fn new() -> Self {
        Self {
            objects: HashMap::default(),
        }
    }

    /// # Insert
    ///
    /// Inserts an object into the game object manager.
    ///
    /// Automatically generates an ID for the game object, sets it, and adds it to the manager.
    ///
    /// Returns a mut reference to the inserted variable.
    pub fn insert<'g>(&'g mut self, game_object: GameObject) -> GameObjectHandle {
        let id = generate_go_id();
        self.objects.insert(id, game_object);

        GameObjectHandle::from_object(id)
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

    pub(crate) fn remove(&mut self, id: GameObjectId) -> () {
        if let Some(mut object) = self.objects.remove(&id) {
            todo!()
        }
    }
}
