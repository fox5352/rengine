use std::{
    sync::Arc,
    thread::sleep,
    time::{Duration, Instant},
};

use crate::{scene::World, types::state_machines::GLOBAL_STATE};

/// Populates the global state from a given scene by extracting and registering
/// all static and physics objects, their identifiers, and associated mask indices.
///
/// This function:
/// - Acquires a write lock on the global state.
/// - Iterates through all static and physics objects in the scene.
/// - Registers each object's ID into global state lookup structures.
/// - Assigns objects to appropriate mask indices.
///
/// # Panics
/// - If locking any object or the global state fails.
/// - If insertion into global maps fails due to internal panics (e.g., duplicate IDs).
///
/// # Errors
/// - Mask appending failures are logged but do not stop execution.
pub fn populate_global_state(scene: &World) {
    let mut global_state = GLOBAL_STATE.write().expect("Failed to lock global state");

    // Populate static objects
    for obj in &scene.s_objects {
        let static_obj = obj
            .lock()
            .expect("Failed to lock static object during population");

        let id = static_obj.get_id().to_string();
        let mask_indices = static_obj.get_masks();

        // z_index
        let z_index = static_obj.get_z_index();

        global_state.s_z_index[z_index as usize].push(id.clone());

        // Register object in appropriate masks
        for index in mask_indices {
            if let Err(e) = global_state.append_mask(index, id.clone()) {
                eprintln!("Failed to append static object to mask: {}", e);
            }
        }

        // Register object in static lookups
        global_state.s_identifiables.push(id.clone());
        global_state.insert_s_map(id, Arc::clone(&obj));
    }

    // Populate active (physics) objects
    for obj in &scene.a_objects {
        let physics_obj = obj
            .lock()
            .expect("Failed to lock physics object during population");

        let id = physics_obj.get_id().to_string();
        let mask_indices = physics_obj.get_masks();

        let z_index = physics_obj.get_z_index();
        global_state.a_z_index[z_index as usize].push(id.clone());

        // Register object in appropriate masks
        for index in mask_indices {
            if let Err(e) = global_state.append_mask(index, id.clone()) {
                eprintln!("Failed to append active object to mask: {}", e);
            }
        }

        // Register object in physics lookups
        global_state.a_identifiables.push(id.clone());
        global_state.insert_a_map(id, Arc::clone(&obj));
    }
}

/// Main game loop structure that manages timing and scene updates
pub struct GameLoop {
    /// Tracks the last frame's timestamp for delta time calculation
    last_time: Instant,
    /// The game world containing all objects to be updated
    scene: World,
}

impl GameLoop {
    /// Creates a new GameLoop instance with the given scene.
    ///
    /// Initializes the global state by populating it with object identifiers and masks
    /// from the provided `scene`.
    ///
    /// # Arguments
    /// * `scene` - The `World` containing all static and active game objects.
    pub fn new(scene: World) -> Self {
        populate_global_state(&scene);
        Self {
            last_time: Instant::now(),
            scene,
        }
    }

    /// Advances the game loop by one frame.
    ///
    /// Calculates the delta time (elapsed time since the last frame) and updates
    /// all game objects accordingly.
    pub fn update(&mut self) {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(self.last_time);
        let dt = delta_time.as_secs_f32(); // Delta time in seconds for physics updates

        self.update_game(dt);
        self.last_time = current_time;
    }

    /// Updates all objects in the scene based on the given delta time.
    ///
    /// Static objects are not updated in the current implementation.
    /// Physics (active) objects have their `update` method called.
    ///
    /// # Arguments
    /// * `delta_time` - Time elapsed since the last update, in seconds.
    pub fn update_game(&mut self, delta_time: f32) {
        // Placeholder for future static object updates
        self.scene.s_objects.iter().for_each(|_obj| {
            // Static objects are currently not updated
        });

        // Update physics (active) objects
        self.scene.a_objects.iter().for_each(|obj| {
            obj.lock().unwrap().process(delta_time);
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
        // TODO: #[cfg(debug_assertions)]
        // {
        //     counter += 1;
        //     // Pause execution every 5 frames for debugging/testing
        //     if counter >= 50 {
        //         println!("ran 50 cycles");
        //         let mut buffer = String::new();
        //         std::io::stdin()
        //             .read_line(&mut buffer)
        //             .expect("Failed to read debug input");
        //         counter = 0;
        //     }
        // }

        // Frame rate limiting - ensure consistent 60 FPS
        let elapsed = start.elapsed();
        if elapsed < FRAME_TIME {
            // Sleep for remaining time to maintain target frame rate
            sleep(FRAME_TIME - elapsed);
        }
    }
}
