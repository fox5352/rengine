use crate::engine::{Object, PhysicsObject};

pub trait PhysicsObjectTrait: Object + PhysicsObject {}
impl<T: Object + PhysicsObject> PhysicsObjectTrait for T {}

/// The World holds objects which are iterable StaticObjects
pub struct World<S, A>
where
    S: IntoIterator<Item = Box<dyn Object>>,
    A: IntoIterator<Item = Box<dyn PhysicsObjectTrait>>,
{
    pub static_objects: S,
    pub animated_objects: A,
}

impl<S, A> World<S, A>
where
    S: IntoIterator<Item = Box<dyn Object>>,
    A: IntoIterator<Item = Box<dyn PhysicsObjectTrait>>,
{
    pub fn new(static_objects: S, animated_objects: A) -> Self {
        Self {
            static_objects,
            animated_objects,
        }
    }
}
