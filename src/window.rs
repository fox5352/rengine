// Standard lib imports for timekeeping
use std::time::{Duration, Instant};

// SDL2 drawing and rectangle
use sdl2::pixels::Color;
use sdl2::rect::Rect;

// Game logic modules you’ve built
use crate::manager::GameLoop;
use crate::scene::World;

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

// Main entry point for rendering a scene
pub fn start_window(scene: World) {
    // Initialize SDL2 context and video system
    let sdl_context = window_match_helper(sdl2::init(), "Failed to initialize SDL context");
    let video_subsystem = window_match_helper(
        sdl_context.video(),
        "Failed to initialize SDL video subsystem",
    );

    // Create the main game window
    let window = window_match_helper(
        video_subsystem
            .window("Rengine", 800, 600)
            .position_centered()
            .build(),
        "Failed to create window",
    );

    // Create canvas for rendering
    let mut canvas = window_match_helper(
        window.into_canvas().accelerated().build(),
        "Failed to create canvas",
    );

    // Event handler for input (quit, keyboard, etc.)
    let mut event_pump =
        window_match_helper(sdl_context.event_pump(), "Failed to create event pump");

    // Game state holds your world/scene logic
    let mut game_state = GameLoop::new(scene);

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
                sdl2::event::Event::KeyDown { .. } => {}
                _ => (),
            }
        }

        // Update game state (e.g., physics, AI, etc.)
        game_state.update();

        // Clear the screen to black
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 1));
        canvas.clear();

        // ----- DRAWING START -----
        // Red filled rect
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.fill_rect(Rect::new(100, 100, 200, 150)).unwrap();

        // Blue filled square
        canvas.set_draw_color(Color::RGB(0, 0, 255));
        canvas.fill_rect(Rect::new(350, 200, 100, 100)).unwrap();

        // Green outlined rectangle
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.draw_rect(Rect::new(500, 100, 150, 100)).unwrap();

        canvas.present();

        // You can draw more shapes here!
        // ----- DRAWING END -----

        // Present (flip the screen)
        canvas.present();

        // Frame limiting to ~60 FPS
        let elapsed = start_time_of_frame.elapsed();
        if elapsed < FRAME_TIME {
            std::thread::sleep(FRAME_TIME - elapsed);
        }
    }
}

