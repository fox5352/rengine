use crate::units::Velocity;

pub use super::units::{Point, Size};

/// The `engine` module defines game objects and their traits.
pub trait Object {
    fn set_size(self, size: Size);
    fn set_pos(self, pos: Point);
}

pub trait PhysicsObject {
    fn update(&mut self, delta_time: f32);
}

/// A static object with position and size
pub struct StaticObject {
    pub pos: Point,
    pub size: Size,
}

impl StaticObject {
    /// Create a new StaticObject
    pub fn new(pos: Point, size: Size) -> Self {
        Self { pos, size }
    }
}

/// Placeholder for animated objects (not implemented)
#[derive(Default)]
pub struct AnimatedObject {
    pub pos: Point,
    pub size: Size,
    pub velocity: Velocity,
}

impl AnimatedObject {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(size: Size, pos: Point, velocity: Velocity) -> Self {
        Self {
            pos,
            size,
            velocity,
        }
    }
}
