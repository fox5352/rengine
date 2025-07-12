use rengine::{
    engine::structures::{AnimatedObject, StaticObject},
    scene::World,
    units::{PointWithDeg, Size, Velocity},
    utils::shapes::CustomShape,
    window::start_window,
};

fn main() {
    let floor = Box::new(StaticObject::new(
        1,
        String::from("Floor"),
        PointWithDeg::new(400.0, 600.0, None),
        Size::new(1400.0, 4.0),
        Some(vec![1]),
        CustomShape::gen_rectangle(),
    ));

    let pos = PointWithDeg::new(250.0, 250.0, None);
    let size = Size::new(50.0, 50.0);
    let moving_platform = Box::new(AnimatedObject::new(
        1,
        String::from("Moving Shape"),
        pos,
        size,
        Velocity { y: 0.0, x: 80.0 },
        Some(vec![1]),
        CustomShape::gen_triangle(),
    ));

    let right_wall = Box::new(StaticObject::new(
        1,
        String::from("Right Wall"),
        PointWithDeg::new(500.0, 250.0, None),
        Size::new(10.0, 80.0),
        Some(vec![1]),
        CustomShape::gen_rectangle(),
    ));

    let mut _world = World::new();
    _world.add_static(vec![floor, right_wall]);
    _world.add_animated(vec![moving_platform]);

    start_window(_world);

    // let mut game_loop = GameLoop::new(_world);
    //
    // let mut counter = 0;
    // const FRAME_TIME: Duration = Duration::from_micros(16_666); // 60 FPS
    //
    // loop {
    //     let start = Instant::now();
    //
    //     game_loop.update();
    //
    //     #[cfg(debug_assertions)]
    //     {
    //         counter += 1;
    //         if counter >= 5 {
    //             println!("ran 5 cycles");
    //             let mut buffer = String::new();
    //             std::io::stdin().read_line(&mut buffer).expect("testing");
    //             counter = 0;
    //         }
    //     }
    //
    //     // Frame limiter to simulate ~60 FPS
    //     let elapsed = start.elapsed();
    //     if elapsed < FRAME_TIME {
    //         sleep(FRAME_TIME - elapsed);
    //     }
    // }
}
