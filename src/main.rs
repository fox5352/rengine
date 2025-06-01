use rengine::{
    engine::{AnimatedObject, Object, Point, Size, StaticObject},
    manager::GameLoop,
    scene::{PhysicsObjectTrait, World},
};

fn main() {
    let pos = Point::new(250, 250, None);
    let size = Size::new(50, 50);

    let _static_obj = StaticObject::new(pos, size);
    let _animated_obj = AnimatedObject::new(pos, size);

    let _world = World {
        static_objects: vec![Box::new(_static_obj) as Box<dyn Object>],
        animated_objects: vec![Box::new(_animated_obj) as Box<dyn PhysicsObjectTrait>],
    };

    let game_loop = GameLoop::new();

    game_loop.run(Some(_world));
}
