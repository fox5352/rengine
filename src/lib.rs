pub mod types;
pub mod utils;

// Declare the modules so Rust knows about them
pub mod engine; // Contains core game object definitions and traits
pub mod manager;
pub mod scene; // Defines the game world and holds collections of objects // Contains the game loop and manages object updates/input
pub mod units;

use engine::{AnimatedObject, Object, PhysicsObject, Point, Size, StaticObject};

impl Object for StaticObject {
    fn set_pos(mut self, pos: Point) {
        self.pos = pos;
    }
    fn set_size(mut self, size: Size) {
        self.size = size;
    }
}

impl Object for AnimatedObject {
    fn set_pos(mut self, pos: Point) {
        self.pos = pos;
    }
    fn set_size(mut self, size: Size) {
        self.size = size;
    }
}

impl PhysicsObject for AnimatedObject {
    fn update(&mut self, delta_time: f32) {
        self.pos.x += self.velocity.x * delta_time;
    }
}
