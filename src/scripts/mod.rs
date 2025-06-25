use crate::{
    engine::{
        structures::{AnimatedObject, StaticObject},
        traits::{CollisionTrait, Object, PhysicsObject},
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
        let new_pos = PointWithDeg {
            x: self.pos.x + self.velocity.x * delta_time,
            y: self.pos.y + self.velocity.y * delta_time,
            deg: self.pos.deg,
        };

        let col = self.check_collision(new_pos);

        // if ! {
        //     self.pos = new_pos;
        // } else {
        //     println!("Collision detected BANG!!!");
        // }
    }
}
