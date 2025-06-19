use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::manager::GameLoop;
use crate::scene::World;
use crate::types::KeyAction;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

const FRAME_TIME: Duration = Duration::from_micros(16_666);

struct App {
    window: Option<Arc<Mutex<Window>>>,
    game_loop: Option<GameLoop>,
    last_physics_tick: Instant,
}

impl App {
    fn new(scene: World) -> Self {
        Self {
            window: None,
            game_loop: Some(GameLoop::new(scene)),
            last_physics_tick: Instant::now(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .expect("Failed to create window");

        self.window = Some(Arc::new(Mutex::new(window)));

        if let Some(window_arc) = &self.window {
            let _size = window_arc
                .lock()
                .expect("falied to initialize window size")
                .inner_size();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // This is the render step
                // TODO: Call your draw function here
                // e.g., self.game_loop.as_mut().unwrap().draw();

                // Then request another redraw
                if let Some(window_arc) = &self.window {
                    window_arc
                        .lock()
                        .expect("Failed to lock window on request redraw")
                        .request_redraw();
                }
            }
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                let key_action = KeyAction::from_key_event(event, is_synthetic, device_id);

                // println!(
                //     "KeyboardInput, event: {:?}, is_synthetic: {}",
                //     event, is_synthetic
                // );
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(game_loop) = self.game_loop.as_mut() {
            let now = Instant::now();
            let elapsed = now.duration_since(self.last_physics_tick);

            if elapsed >= FRAME_TIME {
                game_loop.update(); // fixed-timestep update
                self.last_physics_tick = now;
            }
        }

        if let Some(window_arc) = &self.window {
            window_arc
                .lock()
                .expect("Failed to lock window on about to wait")
                .request_redraw();
        }
    }
}

pub fn start_window(scene: World) {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new(scene);

    event_loop
        .run_app(&mut app)
        .expect("event window loop failed");
}
