use rengine::{
    engine::structures::AnimatedObject,
    manager::run,
    scene::World,
    units::{Point, Size, Velocity},
    utils::shapes::CustomShape,
};

fn main() {
    let pos = Point::new(250, 250, None);
    let size = Size::new(50.0, 50.0);

    let platform = Box::new(AnimatedObject::new(
        String::from("Platform"),
        pos,
        size,
        Velocity { y: 0.0, x: 25.0 },
        Some(vec![1]),
        CustomShape::gen_rectangle(),
    ));

    let mut _world = World::new();
    _world.add_animated(vec![platform]);

    run(_world);

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
