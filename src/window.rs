// Standard lib imports for timekeeping
use std::time::{Duration, Instant};

// SDL2 drawing and rectangle
use sdl2::pixels::Color;
use sdl2::rect::Point;

// Game logic modules you’ve built
use crate::manager::GameLoop;
use crate::scene::World;
use crate::types::KeyAction;
use crate::types::state_machines::{GLOBAL_STATE, push_input_action};
use crate::utils::collision_cal::transform_shape;

// Target ~60 FPS => 1_000_000 µs / 60 ≈ 16,666 µs
const FRAME_TIME: Duration = Duration::from_micros(16_666);

// Helper function to panic with message if SDL init fails
fn window_match_helper<T, E>(result: Result<T, E>, error_message: &str) -> T
where
    E: std::fmt::Display,
{
    match result {
        Ok(value) => value,
        Err(e) => panic!("{}: {}", error_message, e),
    }
}

pub struct Renderer {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Renderer {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = window_match_helper(
            sdl_context.video(),
            "Failed to initialize SDL video subsystem",
        );
        // Create the main game window
        let window = window_match_helper(
            video_subsystem
                .window("Rengine", 800, 600)
                .position_centered()
                .resizable()
                .build(),
            "Failed to create window",
        );

        let canvas = window_match_helper(
            window.into_canvas().accelerated().build(),
            "Failed to create canvas",
        );

        Self { canvas }
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        self.canvas.clear();
    }

    pub fn render(&mut self) {
        let global_state = GLOBAL_STATE.write().expect("Failed to lock global state");

        for row in 0..255 {
            for a_obj_id in global_state.s_z_index[row].iter() {
                // TODO: draw static object
                let id = a_obj_id.clone().to_string();

                if let Some(obj) = global_state.get_s_map_value(&id) {
                    let obj = obj.lock().unwrap();

                    println!("drawing {}", obj.get_name());

                    let cords: Vec<Point> =
                        transform_shape(&obj.get_pos(), &obj.get_size(), &obj.get_shape())
                            .iter()
                            .map(|(x, y)| Point::new(*x as i32, *y as i32))
                            .collect();

                    self.canvas.set_draw_color(Color::RGBA(255, 0, 24, 255));
                    self.canvas.draw_lines(&cords[..]).unwrap();
                }
            }

            for a_col_id in global_state.a_z_index[row].iter() {
                // TODO: draw animated object
                let id = a_col_id.clone().to_string();

                if let Some(obj) = global_state.get_a_map_value(&id) {
                    let obj = obj.lock().unwrap();

                    println!("drawing {}", obj.get_name());

                    let cords: Vec<Point> =
                        transform_shape(&obj.get_pos(), &obj.get_size(), &obj.get_shape())
                            .iter()
                            .map(|(x, y)| Point::new(*x as i32, *y as i32))
                            .collect();

                    self.canvas.set_draw_color(Color::RGBA(204, 85, 0, 255));

                    self.canvas.draw_lines(&cords[..]).unwrap();
                }
            }
        }
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn execute_func<F>(&mut self, func: F)
    where
        F: FnOnce(&mut sdl2::render::Canvas<sdl2::video::Window>),
    {
        func(&mut self.canvas);
    }
}

// Main entry point for rendering a scene
pub fn start_window(scene: World) {
    // Initialize SDL2 context and video system
    let sdl_context = window_match_helper(sdl2::init(), "Failed to initialize SDL context");
    let mut renderer = Renderer::new(&sdl_context);

    // Event handler for input (quit, keyboard, etc.)
    let mut event_pump =
        window_match_helper(sdl_context.event_pump(), "Failed to create event pump");

    // Game state holds your world/scene logic
    let mut game_state = GameLoop::new(scene);

    // imput stuff

    // Main game/render loop
    'window_loop: loop {
        let start_time_of_frame = Instant::now();

        // Input handling
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { timestamp } => {
                    println!("Quit at {} seconds", timestamp);
                    break 'window_loop;
                }
                // You can expand this to handle keys, mouse, etc.
                sdl2::event::Event::KeyDown {
                    timestamp,
                    window_id,
                    keycode: Some(keycode),
                    scancode: _,
                    keymod,
                    repeat,
                } => {
                    push_input_action(KeyAction::new(
                        window_id, keycode, keymod, repeat, timestamp,
                    ));
                }
                _ => (),
            }
        }

        // Update game state (e.g., physics, AI, etc.)
        #[cfg(debug_assertions)]
        {
            println!("Updating game state");
        }
        game_state.update();

        // Clear the screen to black
        #[cfg(debug_assertions)]
        {
            println!("Clearing screen");
        }
        renderer.clear();

        // // ----- DRAWING START -----
        #[cfg(debug_assertions)]
        {
            println!("Drawing");
        }
        renderer.render();
        // You can draw more shapes here!
        // ----- DRAWING END -----

        // Present (flip the screen)
        renderer.present();

        // Frame limiting to ~60 FPS
        let elapsed = start_time_of_frame.elapsed();
        if elapsed < FRAME_TIME {
            std::thread::sleep(FRAME_TIME - elapsed);
        }
    }
}
