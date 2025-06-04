use crate::engine::{Object, PhysicsObject};

pub trait PhysicsObjectTrait: Object + PhysicsObject {}
impl<T: Object + PhysicsObject> PhysicsObjectTrait for T {}

/// The World holds objects which are iterable StaticObjects
pub struct World {
    pub static_objects: Vec<Box<dyn Object>>,
    pub animated_objects: Vec<Box<dyn PhysicsObjectTrait>>,
}

impl World {
    pub fn new(
        static_objects: Vec<Box<dyn Object>>,
        animated_objects: Vec<Box<dyn PhysicsObjectTrait>>,
    ) -> Self {
        Self {
            static_objects,
            animated_objects,
        }
    }
}
