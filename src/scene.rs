use crate::{
    engine::traits::{PhysicsObjectTrait, StaticObjectTrait},
    types::List,
};

/// The World holds objects which are iterable StaticObjects
pub struct World {
    pub s_objects: List<Box<dyn StaticObjectTrait>>,
    pub a_objects: List<Box<dyn PhysicsObjectTrait>>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            s_objects: List::new(),
            a_objects: List::new(),
        }
    }
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_static(&mut self, static_objects: Vec<Box<dyn StaticObjectTrait>>) {
        for obj in static_objects {
            self.s_objects.append(obj);
        }
    }

    pub fn add_animated(&mut self, animated_objects: Vec<Box<dyn PhysicsObjectTrait>>) {
        for obj in animated_objects {
            self.a_objects.append(obj);
        }
    }
}
