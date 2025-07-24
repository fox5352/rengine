use crate::{
    engine::{
        structures::{AnimatedObject, StaticObject},
        traits::{CollisionTrait, Object, PhysicsObject, PointTrait, VelocityTrait},
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

fn _safe_move(obj: &mut AnimatedObject, delta_time: f32) -> bool {
    // Check if velocity is effectively zero
    if obj.get_velocity().x.abs() <= 0.001 && obj.get_velocity().y.abs() <= 0.001 {
        return true; // Movement complete
    }

    let virtual_pos = PointWithDeg {
        x: obj.pos.x + obj.velocity.x * delta_time,
        y: obj.pos.y + obj.velocity.y * delta_time,
        deg: obj.pos.deg,
    };

    if !obj.check_collision(virtual_pos) {
        // Safe to move
        obj.pos = virtual_pos;
        true
    } else {
        // Collision detected, scale down velocity and try again
        obj.velocity.scale_mut(0.9);
        _safe_move(obj, delta_time)
    }
}

impl PhysicsObject for AnimatedObject {
    fn update(&mut self, _delta_time: f32) {}

    fn process(&mut self, delta_time: f32) {
        let pos = self.get_pos();
        if !_safe_move(self, delta_time) {
            println!("Collision detected BANG!!!");
        }

        let new_pos = self.get_pos();
        println!(
            "x moved by:{}|| y moved by:{}",
            new_pos.x - pos.x,
            new_pos.y - pos.y
        );

        println!("Velocity: x:{} y:{}", self.velocity.x, self.velocity.y);
    }
}
