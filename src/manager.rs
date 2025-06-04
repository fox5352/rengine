use std::time::Instant;

use crate::scene::World;

/// Main game loop function
pub struct GameLoop {
    last_time: Instant,
    scene: World, // Holds references valid for 'w
}

impl GameLoop {
    pub fn new(scene: World) -> Self {
        Self {
            last_time: Instant::now(),
            scene,
        }
    }

    pub fn update(&mut self) {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(self.last_time);
        let dt = delta_time.as_secs_f32(); // Convert to seconds as f32

        self.update_game(dt);

        self.last_time = current_time;
    }

    pub fn update_game(&mut self, delta_time: f32) {
        self.scene
            .animated_objects
            .iter_mut()
            .for_each(|obj| obj.update(delta_time));
    }
}
