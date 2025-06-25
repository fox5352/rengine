use crate::{
    engine::{
        structures::{AnimatedObject, StaticObject},
        traits::{Object, PhysicsObject},
    },
    units::{PointWithDeg, Size},
};

impl Object for StaticObject {
    fn set_pos(mut self, pos: PointWithDeg) {
        self.pos = pos;
    }
    fn set_size(mut self, size: Size) {
        self.size = size;
    }
}

impl Object for AnimatedObject {
    fn set_pos(mut self, pos: PointWithDeg) {
        self.pos = pos;
    }
    fn set_size(mut self, size: Size) {
        self.size = size;
    }
}

impl PhysicsObject for AnimatedObject {
    fn update(&mut self, _delta_time: f32) {}

    fn process(&mut self, delta_time: f32) {
        // self.pos.x += self.velocity.x * delta_time;
        // println!("updating pos of {}:{}", self.name, delta_time);
        // println!("updating pos to {}", self.pos.x);
    }
}
