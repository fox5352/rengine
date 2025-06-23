// Standard lib imports for timekeeping
use std::time::{Duration, Instant};

// SDL2 drawing and rectangle
use sdl2::pixels::Color;
use sdl2::rect::Rect;

// Game logic modules you’ve built
use crate::manager::GameLoop;
use crate::scene::World;
use crate::types::KeyAction;
use crate::types::state_machines::push_input_action;

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
        game_state.update();

        // Clear the screen to black
        renderer.clear();

        // ----- DRAWING START -----
        renderer.execute_func(|canvas| {
            canvas.set_draw_color(Color::RGBA(255, 0, 0, 255)); // Full red, fully opaque
            canvas.fill_rect(Rect::new(100, 100, 200, 150)).unwrap();

            canvas.set_draw_color(Color::RGBA(0, 0, 255, 255)); // Blue opaque
            canvas.fill_rect(Rect::new(350, 200, 100, 100)).unwrap();

            canvas.set_draw_color(Color::RGBA(0, 255, 0, 255)); // Green opaque
            canvas.draw_rect(Rect::new(500, 100, 150, 100)).unwrap();
        });
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
