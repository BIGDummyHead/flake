use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, RwLockWriteGuard},
};

use crate::game::{
    Transform,
    object_manager::{GameObjectId, ObjectManager, object_manager_mut},
    texture::Texture,
};

mod component;
mod game_object_handle;
mod game_object_state;

pub use game_object_handle::GameObjectHandle;
pub(crate) use game_object_state::GameObjectState;

pub use component::Component;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator};

/// # Object
///
/// A game object that can be created and is stored in the object manager for the game.
pub struct GameObject {
    pub(crate) id: GameObjectId,
    pub name: String,
    transform: Transform,
    components: HashMap<TypeId, Box<dyn Component + Send + Sync>>,
    texture: Option<Box<dyn Texture + Send + Sync>>,
    state: GameObjectState,
}

impl GameObject {
    pub fn new(name: impl Into<String>, transform: Option<Transform>) -> GameObjectHandle {
        let game_object = Self {
            name: name.into(),
            transform: transform.unwrap_or_default(),
            id: 0,
            components: HashMap::default(),
            texture: None,
            state: GameObjectState::Awake,
        };

        object_manager_mut().insert(game_object)
    }
}

impl GameObject {
    pub fn poll(&mut self) -> () {
        let components = std::mem::take(&mut self.components);

        for (_, component) in &components {
            match &self.state {
                GameObjectState::Awake => {
                    component.awake(self);
                }
                GameObjectState::Start => {
                    component.start(self);
                }
                GameObjectState::Ready => {
                    component.update(self);
                }
                GameObjectState::Removing => {
                    component.remove(self);
                }
            }
        }

        if !matches!(self.state, GameObjectState::Removing) {
            // re-take the components before overwriting them
            let addt_components = std::mem::take(&mut self.components);

            self.components = components;

            // take the additional and add upon
            for (c_key, c) in addt_components {
                if let None = self.components.get(&c_key) {
                    self.components.insert(c_key, c);
                }
            }
        }

        // moves the state forward
        self.state = self.state.clone().move_state();
    }

    /// Forces the state of the game object for the next poll.
    pub(crate) fn force_state(&mut self, state: GameObjectState) -> () {
        self.state = state;
    }

    pub fn id(&self) -> GameObjectId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) -> () {
        self.name = name;
    }

    pub fn add_component<C>(&mut self, c: C) -> ()
    where
        C: Component + Send + Sync + 'static,
    {
        let id = c.type_id();

        if let None = self.components.get(&id) {
            self.components.insert(id, Box::new(c));
        } else {
            dbg!("Adding existing component to object.");
        }
    }

    /// Removes a component via the given type
    pub fn remove_component<C>(&mut self, c: C) -> ()
    where
        C: Component + Send + Sync + 'static,
    {
        self.components.remove(&c.type_id());
    }

    pub fn get_component<C>(&self) -> Option<&(dyn Component + Send + Sync)>
    where
        C: Component + Send + Sync + Sized + 'static,
    {
        let id = TypeId::of::<C>();

        self.components.get(&id).map(|c| c.as_ref())
    }

    pub fn get_component_mut<C>(&mut self) -> Option<&mut (dyn Component + Send + Sync + 'static)>
    where
        C: Component + Send + Sync + Sized + 'static,
    {
        let id = TypeId::of::<C>();

        self.components.get_mut(&id).map(|c| c.as_mut())
    }

    /// # Set Texture
    ///
    /// Sets the texture of the boxed pointer
    pub fn set_texture<T>(&mut self, texture: T) -> ()
    where
        T: Texture + Send + Sync + 'static,
    {
        self.texture = Some(Box::new(texture));
    }

    /// # Texture
    ///
    /// Returns the attached texture
    pub fn texture_mut(&mut self) -> Option<&mut (dyn Texture + Send + Sync + 'static)> {
        self.texture.as_deref_mut()
    }

    /// # Texture
    ///
    /// Returns a reference to the attached texture
    pub fn texture(&self) -> Option<&(dyn Texture + Send + Sync + 'static)> {
        self.texture.as_deref()
    }

    /// # Transform
    ///
    /// The transform (positional information) of the game object.
    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    /// # Transform Mut
    ///
    /// The transform (positional information) of the game object.
    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn destroy(&mut self) -> () {
        self.force_state(GameObjectState::Removing);
        self.poll();
        object_manager_mut().remove(self.id());
    }
}
