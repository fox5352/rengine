// Standard lib imports for timekeeping
use std::time::{Duration, Instant};

// SDL2 drawing and rectangle
use sdl2::pixels::Color;
use sdl2::rect::Point;

// Game logic modules you’ve built
use crate::manager::GameLoop;
use crate::scene::World;
use crate::state::engine_state::{
    get_animated_object, get_animated_z_index_row, get_static_object, get_static_z_index_row,
};
use crate::types::KeyAction;
use crate::types::state_machines::push_input_action;
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

// render helper functions
/// Computes a list of integer coordinate points that approximate the line between two points.
///
/// This function uses a form of linear interpolation to generate evenly spaced
/// integer points between two `f32` coordinates by calculating the number of steps
/// based on the greater delta between x or y. The coordinates are rounded to the
/// nearest integers at each step.
///
/// # Arguments
///
/// * `p1` - A tuple representing the starting point (x, y) as `f32`.
/// * `p2` - A tuple representing the ending point (x, y) as `f32`.
///
/// # Returns
///
/// A `Vec<(i32, i32)>` containing the interpolated points from `p1` to `p2`, excluding `p2`.
///
/// # Example
///
/// ```
/// let points = compute_points_between((0.0, 0.0), (3.0, 3.0));
/// assert_eq!(points, vec![(0, 0), (1, 1), (2, 2)]);
/// ```
// TODO: switch to Bresenham’s Line Generation later to fill holes
pub fn compute_points_between(p1: (f32, f32), p2: (f32, f32)) -> Vec<(i32, i32)> {
    let mut slope: Vec<(i32, i32)> = vec![];
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;

    let steps = dx.abs().max(dy.abs()) as i32;
    let x_step = dx / steps as f32;
    let y_step = dy / steps as f32;

    for index in 0..steps {
        let x = (p1.0 + (x_step * index as f32)).round() as i32;
        let y = (p1.1 + (y_step * index as f32)).round() as i32;

        slope.push((x, y));
    }

    slope
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

    fn fill_triangle(&mut self, pivot: Point, points: &[Point; 2]) {
        let points = compute_points_between(
            (points[0].x as f32, points[0].y as f32),
            (points[1].x as f32, points[1].y as f32),
        );

        points.iter().for_each(|(x, y)| {
            self.canvas.draw_line(pivot, Point::new(*x, *y)).unwrap();
        });
    }

    pub fn render(&mut self) {
        for row_index in 1..255 {
            for s_obj_id in get_static_z_index_row(row_index).unwrap() {
                let obj = get_static_object(&s_obj_id).unwrap();
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
            for a_obj_id in get_animated_z_index_row(row_index).unwrap() {
                let obj = get_animated_object(&a_obj_id).unwrap();
                let obj = obj.lock().unwrap();

                println!("drawing {}", obj.get_name());

                let cords: Vec<Point> =
                    transform_shape(&obj.get_pos(), &obj.get_size(), &obj.get_shape())
                        .iter()
                        .map(|(x, y)| Point::new(*x as i32, *y as i32))
                        .collect();

                self.canvas.set_draw_color(Color::RGBA(204, 85, 0, 255));
                self.canvas.draw_lines(&cords[..]).unwrap();

                let p1 = cords[..][0];
                let [p2, p3] = [cords[1..][0], cords[1..][1]];
                self.fill_triangle(p1, &[p2, p3]);
                self.fill_triangle(p2, &[p1, p3]);
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
        // TODO:
        #[cfg(debug_assertions)]
        {
            println!("Updating game state");
        }

        game_state.update();

        // Clear the screen to black
        // TODO:
        // #[cfg(debug_assertions)]
        // {
        //     println!("Clearing screen");
        // }
        renderer.clear();

        // // ----- DRAWING START -----
        //  TODO: #[cfg(debug_assertions)]
        // {
        //     println!("Drawing");
        // }
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

// testing
#[cfg(test)]
mod test_helper_functions {
    use super::compute_points_between;

    #[test]
    fn test_horizontal_line() {
        let result = compute_points_between((0.0, 0.0), (4.0, 0.0));
        assert_eq!(result, vec![(0, 0), (1, 0), (2, 0), (3, 0)]);
    }

    #[test]
    fn test_vertical_line() {
        let result = compute_points_between((0.0, 0.0), (0.0, 4.0));
        assert_eq!(result, vec![(0, 0), (0, 1), (0, 2), (0, 3)]);
    }

    #[test]
    fn test_diagonal_line() {
        let result = compute_points_between((1.0, 1.0), (4.0, 4.0));
        assert_eq!(result, vec![(1, 1), (2, 2), (3, 3)]);
    }

    #[test]
    fn test_reverse_direction() {
        let result = compute_points_between((3.0, 3.0), (0.0, 0.0));
        assert_eq!(result, vec![(3, 3), (2, 2), (1, 1)]);
    }

    #[test]
    fn test_steep_slope() {
        let result = compute_points_between((0.0, 0.0), (1.0, 4.0));
        assert_eq!(result, vec![(0, 0), (0, 1), (0, 2), (0, 3)]);
    }

    #[test]
    fn test_flat_slope() {
        let result = compute_points_between((2.0, 2.0), (6.0, 2.0));
        assert_eq!(result, vec![(2, 2), (3, 2), (4, 2), (5, 2)]);
    }

    #[test]
    fn test_same_point() {
        let result = compute_points_between((1.0, 1.0), (1.0, 1.0));
        assert_eq!(result, vec![]);
    }
}
