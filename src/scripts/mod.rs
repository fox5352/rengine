use crate::{
    engine::{
        structures::{AnimatedObject, StaticObject},
        traits::{Object, PhysicsObject},
    },
    types::state_machines::get_current_input_action,
    units::{Point, Size},
};

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
    fn update(&mut self, _delta_time: f32) {
        //     self.pos.x += (self.velocity.x * delta_time) as i128;
    }

    fn process(&mut self, delta_time: f32) {
        if let Some(action) = get_current_input_action() {
            println!(
                "key:{} repeat:{} at:{}",
                action.keycode, action.repeat, action.timestamp
            );
        }
    }
}
