use std::{
    sync::Arc,
    thread::sleep,
    time::{Duration, Instant},
};

use crate::{scene::World, types::state_machines::GLOBAL_STATE};

fn populate_global_state(scene: &World) {
    // Initialize vectors to store object IDs for quick lookup into the global state
    let mut globale_state = GLOBAL_STATE.write().expect("Failed to lock global state");
    // Build static objects lookup structures
    scene.s_objects.iter().for_each(|obj| {
        // Extract the unique ID from each static object
        let id = obj
            .lock()
            .expect("Failed to lock on building static identifiables list")
            .get_id()
            .to_string();

        // // Store ID in both vector and hash map for different access patterns
        globale_state.s_identifiables.push(id.clone());
        globale_state.insert_s_map(id.clone(), Arc::clone(&obj));
    });

    // Build active objects lookup structures
    scene.a_objects.iter().for_each(|obj| {
        // Extract the unique ID from each active/physics object
        let id = obj
            .lock()
            .expect("Failed to lock on building active identifiables list")
            .get_id()
            .to_string();

        // Store ID in both vector and hash map for different access patterns
        globale_state.a_identifiables.push(id.clone());
        globale_state.insert_a_map(id.clone(), Arc::clone(&obj));
    });
}

/// Main game loop structure that manages timing and scene updates
pub struct GameLoop {
    /// Tracks the last frame's timestamp for delta time calculation
    last_time: Instant,
    /// The game world containing all objects to be updated
    scene: World,
}

impl GameLoop {
    /// Creates a new GameLoop instance with the given scene
    ///
    /// # Arguments
    /// * `scene` - The World containing all game objects
    pub fn new(scene: World) -> Self {
        populate_global_state(&scene);
        Self {
            last_time: Instant::now(),
            scene,
        }
    }

    /// Updates the game loop, calculating delta time and calling game update
    pub fn update(&mut self) {
        // Calculate time elapsed since last frame
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(self.last_time);
        let dt = delta_time.as_secs_f32(); // Convert to seconds as f32 for physics calculations

        // Update all game objects with the calculated delta time
        self.update_game(dt);

        // Store current time for next frame's delta calculation
        self.last_time = current_time;
    }

    /// Updates all objects in the scene with the given delta time
    ///
    /// # Arguments
    /// * `delta_time` - Time elapsed since last frame in seconds
    pub fn update_game(&mut self, delta_time: f32) {
        // Update static objects (currently no-op, but placeholder for future functionality)
        self.scene.s_objects.iter().for_each(|_obj| {
            // Static objects don't need updates in current implementation
        });

        // Update all active/physics objects with delta time
        self.scene.a_objects.iter().for_each(|obj| {
            // Lock the mutex to access the object and call its update method
            obj.lock().unwrap().update(delta_time);
        });
    }
}

/// Main game loop runner function that sets up object tracking and runs the game
///
/// # Arguments
/// * `scene` - The World containing all game objects to run
pub fn run(scene: World) {
    // Create the main game loop instance
    let mut game_loop = GameLoop::new(scene);

    // Debug counter for development testing
    let mut counter = 0;
    // Target 60 FPS (16.666ms per frame)
    const FRAME_TIME: Duration = Duration::from_micros(16_666);

    // Main game loop - runs indefinitely
    loop {
        // Record frame start time for timing calculations
        let start = Instant::now();

        // Update all game objects for this frame
        game_loop.update();

        // Debug code - only compiled in debug builds
        #[cfg(debug_assertions)]
        {
            counter += 1;
            // Pause execution every 5 frames for debugging/testing
            if counter >= 50 {
                println!("ran 50 cycles");
                let mut buffer = String::new();
                std::io::stdin()
                    .read_line(&mut buffer)
                    .expect("Failed to read debug input");
                counter = 0;
            }
        }

        // Frame rate limiting - ensure consistent 60 FPS
        let elapsed = start.elapsed();
        if elapsed < FRAME_TIME {
            // Sleep for remaining time to maintain target frame rate
            sleep(FRAME_TIME - elapsed);
        }
    }
}
