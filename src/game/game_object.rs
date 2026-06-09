use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::game::{
    Transform,
    object_manager::{GameObjectId, object_manager_mut},
    texture::Texture,
};

mod component;

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
}

impl GameObject {
    pub fn new<'go>(name: String, transform: Option<Transform>) -> &'go mut GameObject {
        let object = Self {
            name,
            transform: transform.unwrap_or_default(),
            id: 0,
            components: HashMap::default(),
            texture: None,
        };

        todo!()
    }
}

impl GameObject {
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
}
